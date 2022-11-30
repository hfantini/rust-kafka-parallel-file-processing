#[macro_use]
extern crate log;

mod log_from;
mod log_level;
mod log_message;
mod file;
mod fragment;

use kafka::producer::{Record, Producer};

pub use log_from::LogFrom;
pub use log_level::LogLevel;
pub use log_message::LogMessage;
pub use file::File;
pub use fragment::Fragment;

pub fn log(producer: &mut Producer, level:LogLevel, from:LogFrom, message:String)
{
    let message:LogMessage = LogMessage { level: level, from: from, message: message };
    match serde_json::to_string_pretty(&message) 
    {
        Ok(value) =>
        {
            producer.send(&Record::from_value("topic_log", value.as_bytes().to_owned())).unwrap();
        }
        Err(error) =>
        {
            error!("CANNOT SERIALIZE LogMessage STRUCT: {}", error.to_string());
        }
    }
}