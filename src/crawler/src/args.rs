use clap::Parser;

#[derive(Parser)]
pub struct Args {
    pub path: std::path::PathBuf,
}