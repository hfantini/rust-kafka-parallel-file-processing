use serde::{Deserialize, Serialize};
use crate::{LogFrom, LogLevel};

#[derive(Serialize, Deserialize, Debug)]
pub struct LogMessage
{
    pub level: LogLevel,
    pub from: LogFrom,
    pub name: String,
    pub message: String
}