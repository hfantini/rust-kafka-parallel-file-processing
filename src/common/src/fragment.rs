use serde::{Deserialize, Serialize};
use crate::file::File;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fragment
{
    pub file: File,
    pub value: String,
    pub line: i64
}