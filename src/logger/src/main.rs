#[macro_use]
extern crate log;

use common::{LogMessage, LogLevel};
use kafka::{consumer::{Consumer, FetchOffset, GroupOffsetStorage}};

fn consume(consumer: &mut Consumer) 
{
    loop
    {
        for ms in consumer.poll().unwrap().iter()
        {
            for m in ms.messages()
            {
                match serde_json::from_str::<LogMessage>(&String::from_utf8_lossy(m.value))
                {
                    Ok(log_message) =>
                    {
                        match log_message.level
                        {
                            LogLevel::TRACE =>
                            {
                                trace!("[{} :: {}] TRACE -> {}", log_message.from, log_message.name, log_message.message)
                            },
                            LogLevel::INFO =>
                            {
                                info!("[{} :: {}] INFO -> {}", log_message.from, log_message.name, log_message.message)
                            },   
                            LogLevel::WARNING =>
                            {
                                warn!("[{} :: {}] WARN -> {}", log_message.from, log_message.name, log_message.message)
                            },
                            LogLevel::ERROR =>
                            {
                                error!("[{} :: {}] ERROR -> {}", log_message.from, log_message.name, log_message.message)
                            }                                                                     
                        }
                    },
                    Err(error) =>
                    {
                        error!("LOGGER: CANNOT PARSE INCOMING MESSAGE: {}", error.to_string())
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

    let mut consumer:Consumer = 
    match Consumer::from_hosts(vec!("localhost:9092".to_owned()))
    .with_topic("topic_log".to_owned())
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

    consume(&mut consumer);
}
