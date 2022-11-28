#[macro_use]
extern crate log;

use std::io::BufRead;
use std::{fs::File, io::BufReader};
use std::time::Duration;
use kafka::producer::Record;
use kafka::{consumer::{Consumer, FetchOffset, GroupOffsetStorage}, producer::{Producer, RequiredAcks}};

fn read(producer: &mut Producer, path:&String)
{
    info!("Reading file: {}", path);
    match File::open(path,) 
    {
        Ok(file) =>
        {
            let reader = BufReader::new(file);

            for line in reader.lines()
            {
                let line = line.unwrap_or_default();
                let split_arr = line.split(" ");

                for fragment_arr in split_arr
                {
                    let word_arr = fragment_arr.split("\t");

                    for word in word_arr
                    {
                        producer.send(&Record::from_value("topic_words", word.as_bytes().to_owned())).unwrap();
                    }
                }
            }
        }
        Err(error) =>
        {
            error!("Cannot read file {}: {}", path, error.to_string());
        }
    }
}

fn consume(producer:&mut Producer, consumer:&mut Consumer)
{
    loop
    {
        for ms in consumer.poll().unwrap().iter()
        {
            for m in ms.messages()
            {
                let path:String = String::from_utf8_lossy(m.value).to_string();
                read(producer, &path);
            }

            consumer.consume_messageset(ms).unwrap();
        }

        consumer.commit_consumed().unwrap();
    }
}

fn main() 
{
    let mut producer:Producer = 
    match Producer::from_hosts( vec!("localhost:9092".to_owned()))
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create() {
            Ok(producer) => producer,
            Err(error) =>
            {
                panic!("Cannot create Apache Kafka Producer: {}", error.to_string());
            } 
        };

    let mut consumer:Consumer = 
    match Consumer::from_hosts(vec!("localhost:9092".to_owned()))
    .with_topic_partitions("topic_selected_files".to_owned(), &[0])
    .with_fallback_offset(FetchOffset::Earliest)
    .with_group("default2".to_owned())
    .with_offset_storage(GroupOffsetStorage::Kafka)
    .create() {
        Ok(consumer) => consumer,
        Err(error) =>
        {
            panic!("Cannot create Apache Kafka Consumer: {}", error.to_string())
        }
    };

    env_logger::init();
    consume(&mut producer, &mut consumer);
}
