use common::{log, LogLevel, LogFrom, Fragment};
use std::io::BufRead;
use std::str::FromStr;
use std::{fs::File, io::BufReader};
use std::time::Duration;
use kafka::producer::Record;
use kafka::{consumer::{Consumer, FetchOffset, GroupOffsetStorage}, producer::{Producer, RequiredAcks}};

fn read(producer: &mut Producer, m_file:&common::File)
{
    log(producer, LogLevel::INFO, LogFrom::READER, format!("READING FILE: {}", m_file.path));
    match File::open(&m_file.path,) 
    {
        Ok(file) =>
        {
            let reader = BufReader::new(file);
            let mut line_counter = 1;

            for line in reader.lines()
            {
                let line = line.unwrap_or_default();
                let splitted_line = line.split(" ");

                for fragment_arr in splitted_line
                {
                    let word_arr = fragment_arr.split("\t");

                    for word in word_arr
                    {
                        match serde_json::to_string_pretty( &Fragment { file: m_file.to_owned(), value: String::from_str(word).unwrap(), line: line_counter, pos: line.find(word).map(|s| s).unwrap() + 1 } )
                        {
                            Ok(value) =>
                            {
                                match producer.send(&Record::from_value("topic_words", value.as_bytes()))
                                {
                                    Ok(_) => (),
                                    Err(error) =>
                                    {
                                        log(producer, LogLevel::ERROR, LogFrom::READER, format!("CANNOT SEND MESSAGE TO APACHE KAFKA: {}", error.to_string()));
                                    }
                                }
                            },
                            Err(error) =>
                            {
                                log(producer, LogLevel::ERROR, LogFrom::READER, format!("CANNOT SERIALIZE DATA INTO FRAGMENT STRUCT: {}", error.to_string()));
                            }                            
                        }
                    }
                }
                
                line_counter += 1;
            }
        }
        Err(error) =>
        {
            log(producer, LogLevel::ERROR, LogFrom::READER, format!("CANNOT READ FILE {}: {}", m_file.path, error.to_string()));
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
                match serde_json::from_str::<common::File>(&String::from_utf8_lossy(m.value))
                {
                    Ok(file) =>
                    {
                        read(producer, &file);
                    },
                    Err(error) =>
                    {
                        log(producer, LogLevel::ERROR, LogFrom::READER, format!("CANNOT PARSE MESSAGE TO FILE STRUCT: {}", error.to_string()));
                    }
                }
            }

            match consumer.consume_messageset(ms) 
            {
                Ok(_) => (),
                Err(error) =>
                {
                    log(producer, LogLevel::ERROR, LogFrom::READER, format!("CANNOT CONSUME MESSAGESET: {}", error.to_string()));
                }
            };
        }

        match consumer.commit_consumed()
        {
            Ok(_) => (),
            Err(error) =>
            {
                log(producer, LogLevel::ERROR, LogFrom::READER, format!("CANNOT COMMIT CONSUMED MESSAGESET: {}", error.to_string()));
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
                panic!("Cannot create Apache Kafka Producer: {}", error.to_string());
            } 
        };

    let mut consumer:Consumer = 
    match Consumer::from_hosts(vec!("localhost:9092".to_owned()))
    .with_topic_partitions("topic_selected_files".to_owned(), &[0])
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

    consume(&mut producer, &mut consumer);
    
}
