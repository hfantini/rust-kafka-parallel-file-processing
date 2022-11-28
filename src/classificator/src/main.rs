#[macro_use]
extern crate log;

use std::time::Duration;
use kafka::{consumer::{Consumer, FetchOffset, GroupOffsetStorage}, producer::{Producer, RequiredAcks, Record}};

fn consume(producer:&mut Producer, consumer:&mut Consumer)
{
    loop
    {
        for ms in consumer.poll().unwrap().iter()
        {
            for m in ms.messages()
            {
                let path:String = String::from_utf8_lossy(m.value).to_string();
                if path.contains(".txt")
                {
                    info!("FOUND .TXT FILE AT: {}", path);
                    producer.send(&Record::from_value("topic_selected_files", path.as_bytes().to_owned())).unwrap();
                }
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
    .with_topic_partitions("topic_files".to_owned(), &[0])
    .with_fallback_offset(FetchOffset::Earliest)
    .with_group("default".to_owned())
    .with_offset_storage(GroupOffsetStorage::Kafka)
    .create() {
        Ok(consumer) => consumer,
        Err(error) =>
        {
            panic!("Cannot create Apache Kafka Consumer: {}", error.to_string())
        }
    };

    env_logger::init();
    info!("STARTING CLASSIFICATOR");
    consume(&mut producer, &mut consumer);
}
