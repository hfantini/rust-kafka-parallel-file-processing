use clap::Parser;

#[derive(Parser)]
pub struct Args {
    pub name: String,
    pub output: std::path::PathBuf,
    pub search: String,
}