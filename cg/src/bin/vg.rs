use clap::{Parser, command, ArgGroup};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author = "SliceOfArdath", version, about = "Open code, fast.", long_about = None)]
#[command(group(ArgGroup::new("use").args(["id"]).requires("tool")))]
#[command(group(ArgGroup::new("existing").args(["new_tool"]).conflicts_with_all(["use", "tool"])))]
pub struct Args {
    /// The regular expression used for searching.
    #[arg(value_name="ID")]
    id: Option<usize>,
    /// The tool name to use.
    #[arg(short,long)]
    tool: Option<String>,
    /// The tool to define.
    #[arg(short,long)]
    new_tool: Option<String>,
}

// vscode: code -g {file}:{line}
// pico: pico +{line} {file}
// nano: nano +{line} {file} 

fn open() {
    let h = home::home_dir().expect("Could not find home dir.").join(".rgvg_last");

    let s = std::fs::read(h);
    println!("Hello, world!");
}
fn add_tool() {

}
fn display() {

}
 
fn main() {
    let r = Args::parse();

    match r.id {
        Some(_) => open(),
        None => match r.new_tool {
            Some(_) => add_tool(),
            None => display(),
        }
    }
}
