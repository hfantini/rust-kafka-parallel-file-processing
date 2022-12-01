mod args;

use clap::Parser;
use common::{log, LogLevel, LogFrom, File};
use std::{fs::read_dir, time::Duration};
use kafka::producer::{Producer, Record, RequiredAcks};
use args::Args;

fn scan_directory(directory: String, producer:&mut Producer)
{
    log(producer, LogLevel::TRACE, LogFrom::CRAWLER, format!("READING DIRECTORY: {}", directory));
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
                                    log(producer, LogLevel::TRACE, LogFrom::CRAWLER, format!("FOUND DIRECTORY: {}", str_path));
                                    scan_directory(str_path, producer);
                                }
                                else if metadata.is_file()
                                {
                                    let file:File = File { path: str_path };
                                    match serde_json::to_string_pretty(&file) 
                                    {
                                        Ok(value) =>
                                        {
                                            match producer.send(&Record::from_value("topic_files", value))
                                            {
                                                Ok(_) => (),
                                                Err(error) =>
                                                {
                                                    log(producer, LogLevel::ERROR, LogFrom::CRAWLER, format!("CANNOT SEND MESSAGE TO APACHE KAFKA: {}", error.to_string()));
                                                }
                                            }
                                        },
                                        Err(error) =>
                                        {
                                            log(producer, LogLevel::ERROR, LogFrom::CRAWLER, format!("CANNOT SERIALIZE FILE STRUCTURE: {}", error.to_string()));
                                        }
                                    }

                                }
                            }

                            Result::Err(error) =>
                            {
                                log(producer, LogLevel::ERROR, LogFrom::CRAWLER, format!("CANNOT OBTAIN METADATA FROM DIR_ENTRY: {}", error.to_string()));
                            }
                        }
                    }

                    Result::Err(error) =>
                    {
                        log(producer, LogLevel::ERROR, LogFrom::CRAWLER, format!("CANNOT READ DIRECTORY FROM DIR_ENTRY: {}", error.to_string()));
                    }
                }
            }
        }
        Result::Err(error) => 
        {
            log(producer, LogLevel::ERROR, LogFrom::CRAWLER, format!("CANNOT READ DIRECTORY: {}", error.to_string()));
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
                panic!("CANNOT CREATE APACHE KAFKA PRODUCER: {}", error.to_string());
            } 
        };

    let args:Args = Args::parse();
    let path = args.path.into_os_string().into_string().unwrap();

    log(&mut producer, LogLevel::TRACE, LogFrom::CRAWLER, format!("STARTING CRAWLER OVER: {path}", ));
    scan_directory(path, &mut producer);
}
