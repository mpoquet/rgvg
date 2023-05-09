use std::fs::write;
use std::process::{Command,Output,Stdio,Child};
use std::{io, fs};
use clap::Parser;

//use crate::input;

mod input;
mod output;
mod common;
 

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
    return last.wait_with_output(); //todo!
}

/// The full call
fn call(command: input::Cmd) -> Result<Output, io::Error> {
    finish(begin(command.0.to_string(), command.1))
}

//NOTE: Search for command in PATH, try to find rust crate
// Format stdout facile a lire avec grep


use crate::input::framework::Convertible;

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
    /*let text = r"./documents/t.txt:1:haha [01;31m[Kthe[m[K ha
./documents/t.txt:2:elh[01;31m[Kthe[m[K:)
";
    output::read(output::GREP, text);
    let text = r"[0m[35m./documents/t.txt[0m:[0m[32m1[0m:haha [0m[1m[31mthe[0m ha
[0m[35m./documents/t.txt[0m:[0m[32m2[0m:elh[0m[1m[31mthe[0m:)
";
    output::read(output::RIPGREP, text);*/

    let start = std::time::Instant::now();
    let args = input::Args::parse();
    //println!("{:?}", args);
    let mut g = input::tools::picker("ripgrep");
    let q = g.populate(args);
    //println!("{:?}", q);
    let p = g.generate(q);
    println!("{:?}", p);
    let r = call(p).unwrap();
    let stop = std::time::Instant::now();

    println!("t1: {:?}", stop - start);
    let s = &String::from_utf8(r.stdout).unwrap();
    let stop = std::time::Instant::now();
    println!("t2: {:?}", stop - start);
    let result = output::read(output::RIPGREP, s);
    let stop = std::time::Instant::now();
    println!("t3: {:?}", stop - start);
    //output::display(&result);
    output::write(&result);
}