#[macro_use]
extern crate log;

use kafka::{consumer::{Consumer, FetchOffset, GroupOffsetStorage}};

fn consume(consumer:&mut Consumer)
{
    loop
    {
        for ms in consumer.poll().unwrap().iter()
        {
            for m in ms.messages()
            {
                let value:String = String::from_utf8_lossy(m.value).to_string().to_lowercase();
                let expected:String = "the".to_string();
                
                if value == expected
                {
                    info!("Found THE");
                }
            }

            consumer.consume_messageset(ms).unwrap();
        }

        consumer.commit_consumed().unwrap();
    }
}

fn main() 
{
    let mut consumer:Consumer = 
    match Consumer::from_hosts(vec!("localhost:9092".to_owned()))
    .with_topic_partitions("topic_words".to_owned(), &[0])
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
    consume(&mut consumer);
}
