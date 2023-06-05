pub mod framework;
pub mod tools;

use clap::{Parser, command, ArgGroup};
use std::path::PathBuf;


/// Interesting tools: clap_complete, clap_man
#[derive(Parser, Debug)]
#[command(author = "SliceOfArdath", version, about = "Find code, fast.", long_about = None)]
#[command(group(ArgGroup::new("regex").args(["tool", "file", "casei", "include_files", "include_dir", "exclude_files", "exclude_dir", "order_results", "dry"]).multiple(true).requires("regex_pattern")))]
#[command(group(ArgGroup::new("tools-l").args(["list_tools"]).conflicts_with_all(["regex", "color"])))]

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
    /// Dry run - Prints command instead of running them.
    #[arg(long,short='n')]
    pub dry: bool,
    /// Order results alphabeticacally according to file name, line number, and line number
    #[arg(long,short)]
    pub order_results: bool,
    /// Remove leading space
    #[arg(long)]
    pub remove_leading: bool,
    /// The regular expression used for searching.
    #[arg(value_name="PATTERN")]
    pub regex_pattern: Option<String>,
    /// The file or directory to search.
    #[arg(value_name="PATH")]
    file: Vec<PathBuf>,
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
    include_dir: Vec<String>, //todo!
    /// Globs d'exclusion
    #[arg(long)]
    exclude_dir: Vec<String>,
}
