use clap::{Parser, command, ArgGroup};
use common::command::{self, Cmd};
use std::path::PathBuf;
use regex::Regex;

mod common;

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

///tool definition format:
///{tool_name}: [string] where [string] is any string containing the substrings {line} and {file}
fn get_tool(tool_list: Vec<&str>, tool: String) -> String {
    let r = Regex::new(&("^".to_string() + &tool + ":")).unwrap();
    let tool_def = tool_list.iter().find(|c| r.is_match(c)).expect("No tool with requested name!");
    return r.replace(&tool_def, "").to_string();
}

fn open(tool_list: String, id: usize, tool: String) {
    let (r,s) = common::open_last().expect("No last file for user! Use 'cg' to create a last file.");
    let request = s.get(id).expect("Delivered id was not valid! Valid IDs range between 0..MAX_ID.");

    let t = get_tool(tool_list.split('\n').collect(), tool);
    let mut t: Vec<String> = t.split(" ").map(|s| {
        let r1 = Regex::new(r"\{file\}").unwrap();
        let r2 = Regex::new(r"\{line\}").unwrap();
        return r2.replace(&r1.replace(s, request.filename.to_owned()).to_string(), request.line.to_string()).to_string(); 
    }).collect();
    t.retain(|s| s != "");
    
    let c = (t.get(0).expect("Could not find executable!").to_string(), t[1..].to_vec());

    command::blind_call(c).expect("An error occured opening in your target editor.");
}
fn add_tool() {
    todo!();
}
fn display() {
    let (r,s) = common::open_last().expect("No last file for user! Use 'cg' to create a last file.");
    common::display(r, &s);
}
 
fn main() {
    let r = Args::parse();

    let h = home::home_dir().expect("Could not find home dir.").join(common::OPEN_FORMAT_PATH);
    let s = std::fs::read_to_string(h).unwrap_or(
r"vscode: code -g {file}:{line}
pico: pico +{line} {file}
nano: nano +{line} {file}".to_string());
    match r.id {
        Some(id) => open(s, id, r.tool.unwrap()),
        None => match r.new_tool {
            Some(_) => add_tool(),
            None => display(),
        }
    }
}
