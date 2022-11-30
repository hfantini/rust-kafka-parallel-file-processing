use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum LogLevel
{
    TRACE,
    INFO,
    WARNING,
    ERROR
}

impl fmt::Display for LogLevel 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        match *self 
        {
            LogLevel::TRACE => write!(f, "TRACE"),
            LogLevel::INFO => write!(f, "INFO"),
            LogLevel::WARNING => write!(f, "WARNING"),
            LogLevel::ERROR => write!(f, "ERROR")
        }
    }
}