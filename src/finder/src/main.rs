mod args;

use clap::Parser;
use std::{time::Duration, io::{LineWriter, Write}, fs::File, str::FromStr, sync::Mutex};
use kafka::{consumer::{Consumer, FetchOffset, GroupOffsetStorage}, producer::{Producer, RequiredAcks}};
use common::{log, LogLevel, LogFrom, Fragment};
use args::Args;

static NAME:Mutex<String> = Mutex::new(String::new());

fn write_line(file: &File, value: String)
{
    let line = [value, "\n".to_string()].concat();
    let mut line_writter = LineWriter::new(file);
    line_writter.write_all(&line.as_bytes()).unwrap();
}

fn consume(producer:&mut Producer, consumer: &mut Consumer, file: &File, search:Vec<&str>)
{
    loop
    {
        for ms in consumer.poll().unwrap().iter()
        {
            for m in ms.messages()
            {
                match serde_json::from_str::<Fragment>(&String::from_utf8_lossy(m.value))
                {
                    Ok(fragment) =>
                    {
                        for expected in search.iter()
                        {
                            let expected_str = String::from_str(*expected).unwrap();
                            if fragment.value.to_lowercase() == expected_str
                            {
                                let line:String = format!("FOUND '{}' WORD @ LINE {} IN FILE -> {}", *expected, fragment.line, fragment.file.path);
                                log(producer, LogLevel::INFO, LogFrom::FINDER, NAME.lock().unwrap().to_owned(), line.to_owned());
                                write_line(file, line.to_owned());
                            };
                        }
                    },
                    Err(error) =>
                    {
                        log(producer, LogLevel::ERROR, LogFrom::FINDER, NAME.lock().unwrap().to_owned(), format!("CANNOT PARSE MESSAGE INTO FRAGMENT STRCUTURE: {}", error.to_string()));
                    }
                }
            }

            consumer.consume_messageset(ms).unwrap();
        }

        consumer.commit_consumed().unwrap();
    }
}

fn main() 
{
    env_logger::init();

    let mut producer:Producer = 
    match Producer::from_hosts( vec!("localhost:9092".to_owned()))
    .with_ack_timeout(Duration::from_secs(60))
    .with_required_acks(RequiredAcks::All)
    .create() 
    {
        Ok(producer) => producer,
        Err(error) =>
        {
            panic!("CANOOT CREATE APACHE KAFKA PRODUCER: {}", error.to_string());
        } 
    };
        
    let mut consumer:Consumer = 
    match Consumer::from_hosts(vec!("localhost:9092".to_owned()))
    .with_topic("topic_words".to_owned())
    .with_fallback_offset(FetchOffset::Earliest)
    .with_group("default".to_owned())
    .with_offset_storage(GroupOffsetStorage::Kafka)
    .create() 
    {
            Ok(consumer) => consumer,
            Err(error) =>
            {
                panic!("Cannot create Apache Kafka Consumer: {}", error.to_string())
            }
    };

    let args = Args::parse();
    NAME.lock().unwrap().push_str(&args.name);
    let path = args.output.into_os_string().into_string().unwrap();
    let file = File::create(path).unwrap();
    let search:Vec<&str> = args.search.split(";").collect();

    consume(&mut producer, &mut consumer, &file, search);
}
