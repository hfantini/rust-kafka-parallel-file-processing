#[macro_use]
extern crate log;

use std::{fs::read_dir, time::Duration};
use kafka::producer::{Producer, Record, RequiredAcks};

fn scan_directory(directory: String, producer:&mut Producer)
{
    match read_dir(directory) 
    {
        Result::Ok(paths) => 
        {
            for dir_entry in paths 
            {
                match dir_entry
                {
                    Result::Ok(dir_entry) =>
                    {
                        let str_path = dir_entry.path().display().to_string().replace("\\", "/");

                        match dir_entry.metadata()
                        {
                            Result::Ok(metadata) =>
                            {
                                if metadata.is_dir()
                                {
                                    scan_directory(str_path, producer);
                                }
                                else if metadata.is_file()
                                {
                                    producer.send(&Record::from_value("topic_files", str_path.as_bytes().to_owned())).unwrap();
                                }
                            }

                            Result::Err(e) =>
                            {
                                error!("Cannot obtain metadata from dir_entry: {}", e.to_string())
                            }
                        }
                    }

                    Result::Err(e) =>
                    {
                        error!("Cannot read directory: {}", e.to_string())
                    }
                }
            }
        }
        Result::Err(e) => 
        {
            error!("Cannot read directory: {}", e.to_string())
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

    env_logger::init();
    info!("STARTING CRAWLER OVER: {}", "C:/");
    scan_directory("C:/".to_string(), &mut producer);
}
