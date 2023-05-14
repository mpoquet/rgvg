pub mod framework;
pub mod tools;

use clap::{Parser, command};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author = "SliceOfArdath", version, about = "Find code, fast.", long_about = None)]
pub struct Args {
    /// Externary option - defines the tool to use. Possible tools are grep, ripgrep, ugrep.
    #[arg(short,long, default_value="grep")]
    pub tool: String,
    /// Externary option - lists tools
    #[arg(short,long)]
    pub list_tools: bool, 
    /// Color mode
    #[arg(long, default_value="yes")]
    pub color: String, 
    /// The regular expression used for searching.
    #[arg(value_name="PATTERN")]
    pub regex_pattern: Option<String>,
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