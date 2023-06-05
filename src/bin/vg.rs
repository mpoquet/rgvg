use clap::{Parser, command, ArgGroup};
use std::io::Write;
use regex::Regex;


const TOOLS: &'static str = 
r"vscode: code -g {file}:{line}
pico: pico +{line} {file}
nano: nano +{line} {file}
vim: vim +{line} {file}
emacs: emacs +{line} {file}";

#[derive(Parser, Debug)]
#[command(author = "SliceOfArdath", version, about = "Open code, fast.", long_about = None)]
#[command(group(ArgGroup::new("use").args(["tool"]).requires("id")))]
#[command(group(ArgGroup::new("existing").args(["new_tool"]).conflicts_with_all(["use", "list_tools"])))]
#[command(group(ArgGroup::new("ids").args(["id"]).conflicts_with_all(["new_tool", "list_tools"])))]
pub struct Args {
    /// The regular expression used for searching.
    #[arg(value_name="ID")]
    id: Option<usize>,
    /// Color mode
    #[arg(long, default_value="yes")]
    color: String,
    /// The tool name to use.
    #[arg(short,long,default_value="pico")]
    tool: String,
    /// The tool to define.
    #[arg(short,long)]
    new_tool: Option<String>,
    /// Lists all available tools
    #[arg(short,long)]
    list_tools: bool,
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
    let (mut r,s) = rgvg::common::open_last().expect("No last file for user! Use 'cg' to create a last file.");
    let request = s.get(id).expect("Delivered id was not valid! Valid IDs range between 0..MAX_ID.");

    r.push(request.filename.clone());

    let t = get_tool(tool_list.split('\n').collect(), tool);
    let mut t: Vec<String> = t.split(" ").map(|s| {
        let r1 = Regex::new(r"\{file\}").unwrap();
        let r2 = Regex::new(r"\{line\}").unwrap();
        return r2.replace(&r1.replace(s, r.to_string_lossy()).to_string(), request.line.to_string()).to_string(); 
    }).collect();
    t.retain(|s| s != "");
    
    let c = (t.get(0).expect("Could not find executable!").to_string(), t[1..].to_vec());

    rgvg::common::command::blind_call(c).expect("An error occured opening in your target editor.");
}
fn add_tool(nt: String) {
    let h = home::home_dir().expect("Could not find home dir.").join(rgvg::common::OPEN_FORMAT_PATH);
        if !h.exists() {
            std::fs::write(h, TOOLS.to_owned() + &nt + "\n").expect("Could not create tool registry in your home directory!")
        } else if h.is_file() {
            let mut e = std::fs::OpenOptions::new().append(true).open(h).expect("Could not edit tool registry!");
            writeln!(e, "{}",  nt).expect("Could not write to the tool registry.");
        }
    }

 
fn main() {
    let r = Args::parse();

    let h = home::home_dir().expect("Could not find home dir.").join(rgvg::common::OPEN_FORMAT_PATH);
    let s = std::fs::read_to_string(h).unwrap_or(TOOLS.to_owned());
    match r.id {
        Some(id) => open(s, id, r.tool),
        None => match r.new_tool {
            Some(nt) => add_tool(nt),
            None => match r.list_tools {
                true => {
                    let r = Regex::new(&(r"(?m)^(\w+):.*$")).unwrap();
                    for t in r.captures_iter(&s) {
                        println!("{}", &t[1]);
                    }
                },
                false => rgvg::common::last(rgvg::common::color(&r.color)),
            }
        }
    }
}
