pub mod framework;
pub mod tools;

use clap::{Parser, command};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author = "SliceOfArdath", version, about = "Find code, fast.", long_about = None)]
pub struct Args {
    /// The regular expression used for searching.
    #[arg(required=true,value_name="PATTERN")]
    regex_pattern: String,
    /// The file or directory to search.
    #[arg(value_name="PATH")]
    file: Option<PathBuf>,
    /// Case insensitive mode
    #[arg(short='i')]
    casei: bool,
    /// Globs d'inclusion
    #[arg(long)]
    include_files: Vec<String>,
    /// Globs d'exclusion
    #[arg(long)]
    exclude_files: Vec<String>,
    /// Globs d'inclusion
    #[arg(long)]
    include_dir: Vec<String>,
    /// Globs d'exclusion
    #[arg(long)]
    exclude_dir: Vec<String>,
}