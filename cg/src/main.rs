use std::collections::LinkedList;
use std::process::{Command,Output,Stdio,Child};
use std::{io, fs};
use clap::Parser;

mod command;

 

///Creates a command from a command string.
/*fn build(command: Vec<&str>) -> Command {
    let mut output = Command::new(command.get(0).expect("No command attached!"));

    for i in 1..command.len() {
        output.arg(command[i]);
    }
    return output;
}*/
fn build(command: String, args: Vec<String>) -> Command {
    let mut output = Command::new(command);

    for i in args {
        output.arg(i);
    }
    return output;
}

///Call the first command in a call chain
fn begin(command: String, args: Vec<String>) -> Child {
    return build(command, args).stdout(Stdio::piped()).spawn().expect("Failed command");
}
/// Links the first command's ouput to the second's input, then starts the second command.
fn link(first: Child, command: String, args: Vec<String>) -> Child {
    //first.stdout(Stdio::piped());
    return build(command,args).stdin(first.stdout.unwrap()).stdout(Stdio::piped()).spawn().expect("Failed command");
}
///Finishes a call stack
fn finish(last: Child) -> Result<Output, io::Error> {
    return last.wait_with_output();
}

//NOTE: Search for command in PATH, try to find rust crate
// Format stdout facile a lire avec grep

use regex::Regex;

use crate::command::framework::Convertible;

fn rc<'a>(cap: &Option<regex::Match<'a>>) -> &'a str {
    match cap {
        Some(x) => {
            if x.start() == x.end() {
                return "None"
            } else {
                return x.as_str()
            }
        },
        None => "",
    }
}

fn main() {
    /*let r = Regex::new(r"((?P<n1>#\d{1,3})|(?P<n2>-\pL)|(?P<n3>--\pL+))((?P<d1>!)|(?P<d2><[\pL-]*\[\pL*\]>)|(?P<d3>))(?P<s>\{\pL+\})((?P<t1>\[\pL+\])|(?P<t2>))").unwrap();
    let text = "#1<-[str]>{path}[path] -i{casei}  #0!{pattern}[str] --estrogen{estr}[int]";
    for cap in r.captures_iter(text) {
        println!("{}{}{} {}{}{} {} {}{}", 
            rc(&cap.name("n1")), 
            rc(&cap.name("n2")), 
            rc(&cap.name("n3")), 
            rc(&cap.name("d1")), 
            rc(&cap.name("d2")),
            rc(&cap.name("d3")),
            rc(&cap.name("s") ),
            rc(&cap.name("t1")),
            rc(&cap.name("t2")));
    }*/
    let args = command::Args::parse();
    //println!("{:?}", args);
    let q = command::tools::GREP.clone().populate(args);
    //println!("{:?}", q);
    let p = command::tools::Grepper::generate(q);
    println!("{:?}", p);
    let r = finish(begin("grep".to_string(), p)).unwrap();
    println!("{}", String::from_utf8(r.stdout).unwrap());
}