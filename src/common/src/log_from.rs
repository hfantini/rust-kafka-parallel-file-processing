use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum LogFrom
{
    CRAWLER = 0,
    CLASSIFICATOR = 1,
    READER = 2,
    FINDER = 3
}

impl fmt::Display for LogFrom 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        match *self 
        {
            LogFrom::CRAWLER => write!(f, "CRAWLER"),
            LogFrom::CLASSIFICATOR => write!(f, "CLASSIFICATOR"),
            LogFrom::READER => write!(f, "READER"),
            LogFrom::FINDER => write!(f, "FINDER")
        }
    }
}