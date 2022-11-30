use common::{log, LogLevel, LogFrom, File};
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
                match serde_json::from_str::<File>(&String::from_utf8_lossy(m.value))
                {
                    Ok(file) =>
                    {
                        if file.path.contains(".txt")
                        {
                            log(producer, LogLevel::INFO, LogFrom::CLASSIFICATOR, format!("FOUND .TXT FILE AT: {}", file.path));
                            match producer.send(&Record::from_value("topic_selected_files", m.value))
                            {
                                Ok(_) => (),
                                Err(error) =>
                                {
                                    log(producer, LogLevel::ERROR, LogFrom::CLASSIFICATOR, format!("CANNOT SEND MESSAGE TO APACHE KAFKA: {}", error.to_string()));
                                }
                            }
                        }
                    }
                    Err(error) =>
                    {
                        log(producer, LogLevel::ERROR, LogFrom::CLASSIFICATOR, format!("CANNOT PARSE MESSAGE TO FILE STRUCT: {}", error.to_string()));
                    }
                }
            }

            match consumer.consume_messageset(ms)
            {
                Ok(_) => (),
                Err(error) =>
                {
                    log(producer, LogLevel::ERROR, LogFrom::CLASSIFICATOR, format!("CANNOT CONSUME MESSAGESET: {}", error.to_string()));
                }
            }            
        }

        match consumer.commit_consumed()
        {
            Ok(_) => (),
            Err(error) =>
            {
                log(producer, LogLevel::ERROR, LogFrom::CLASSIFICATOR, format!("CANNOT COMMIT CONSUMED MESSAGESET: {}", error.to_string()));
            }            
        }
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
                panic!("CANOOT CREATE APACHE KAFKA PRODUCER: {}", error.to_string());
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
            panic!("CANNOT CREATE APACHE KAFKA CONSUMER: {}", error.to_string())
        }
    };

    log(&mut producer, LogLevel::TRACE, LogFrom::CLASSIFICATOR, "STARTING CLASSIFICATOR".to_string());
    consume(&mut producer, &mut consumer);
}
