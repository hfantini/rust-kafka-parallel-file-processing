use std::{time::Duration, io::{LineWriter, Write}, fs::File};
use kafka::{consumer::{Consumer, FetchOffset, GroupOffsetStorage}, producer::{Producer, RequiredAcks}};
use common::{log, LogLevel, LogFrom, Fragment};

fn write_line(file: &File, value: String)
{
    let line = [value, "\n".to_string()].concat();
    let mut line_writter = LineWriter::new(file);
    line_writter.write_all(&line.as_bytes()).unwrap();
}

fn consume(producer:&mut Producer, consumer: &mut Consumer, file: &File)
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
                        let expected:String = "system".to_string();
                        if fragment.value.to_lowercase() == expected
                        {
                            let line:String = format!("FOUND '{}' WORD @ {} : {} IN FILE -> {}", expected, fragment.line, fragment.pos, fragment.file.path);
                            log(producer, LogLevel::INFO, LogFrom::FINDER, line.to_owned());
                            write_line(file, line.to_owned());
                        }
                    },
                    Err(error) =>
                    {
                        log(producer, LogLevel::ERROR, LogFrom::FINDER, format!("CANNOT PARSE MESSAGE INTO FRAGMENT STRCUTURE: {}", error.to_string()));
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
    .with_ack_timeout(Duration::from_secs(1))
    .with_required_acks(RequiredAcks::One)
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
    .with_topic_partitions("topic_words".to_owned(), &[0])
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

    let file = File::create("result.txt").unwrap();

    consume(&mut producer, &mut consumer, &file);
}
