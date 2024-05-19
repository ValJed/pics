use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(name = "pics")]
pub struct Cli {
    /// Path of the directory to rename files in
    pub path: PathBuf,

    /// Rename files recursively
    #[arg(short, long/* , default_value = false */)]
    pub recursive: bool,
}
