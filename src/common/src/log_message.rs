use serde::{Deserialize, Serialize};
use crate::{LogFrom, LogLevel};

#[derive(Serialize, Deserialize, Debug)]
pub struct LogMessage
{
    pub level: LogLevel,
    pub from: LogFrom,
    pub message: String
}