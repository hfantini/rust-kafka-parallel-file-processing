use clap::Parser;

#[derive(Parser)]
pub struct Args {
    pub output: std::path::PathBuf,
    pub word: String,
}