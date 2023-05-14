use std::{fmt::{Display}, path::PathBuf, io::BufRead};

pub const LAST_PATH: &'static str = ".rgvg_last";
#[allow(dead_code)]
pub const OPEN_FORMAT_PATH: &'static str = ".rgvg_open_format";
pub const NAME_LEN: usize = 512;
pub const MATCH_LEN: usize = 512;
const DATA_LEN: usize = NAME_LEN + std::mem::size_of::<usize>() + MATCH_LEN;
pub const VERSION: usize = 1;

/// Writing to Disk
impl From<&Match> for Vec<u8> {
    fn from(value: &Match) -> Self {
        let mut r: Vec<u8> = Vec::from(value.filename.clone());
        r.extend(std::iter::repeat(0).take(NAME_LEN - value.filename.len()));
        r.extend((value.line).to_be_bytes().into_iter()); //Crucial cast to ensure size consistency!
        r.extend(Vec::from(value.matched.clone()));
        r.extend(std::iter::repeat(0).take(MATCH_LEN - value.matched.len()));
        return r;
    }
}
/// Reading from Disk
impl From<&[u8]> for Match {
    fn from(value: &[u8]) -> Self {
        let (f, rest) = value.split_at(NAME_LEN);
        let (l, rest) = rest.split_at(std::mem::size_of::<usize>());
        let m = rest;
        return Match {
            filename: String::from_utf8(f.to_vec()).expect("Data should be valid utf-8.").trim_end_matches('\x00').to_string(),
            line: usize::from_be_bytes(l.try_into().unwrap()),
            matched: String::from_utf8(m.to_vec()).expect("Data should be valid utf-8.").trim_end_matches('\x00').to_string(),
        };
    }
}

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn disp_flag(b: bool) -> &'static str {
            match b {
                false => "",
                true => ">",
            }
        }
        write!(f, "{}{} {} {}{}", self.filename, disp_flag(NAME_LEN == self.filename.len()), self.line, self.matched, disp_flag(MATCH_LEN == self.matched.len()))
    }
}
impl Match {
    fn disp(&self) -> String {
        fn disp_flag(b: bool) -> &'static str {
            match b {
                false => "",
                true => "\x1b[1m\x1b[31m🆇\x1b[39m\x1b[0m",
            }
        }
        let mut r = "\x1b[35m".to_owned();
        r += &self.filename;
        r += disp_flag(NAME_LEN == self.filename.len());
        r += "\x1b[34m ";
        r += &self.line.to_string();
        r += "\x1b[32m ";
        r += &self.matched;
        r += disp_flag(NAME_LEN == self.matched.len());
        r += "\x1b[0m";
        return r;
    }
}

#[derive(Debug, Clone)]
pub struct Match {
    pub filename: String,
    pub line: usize,
    pub matched: String,
}

#[cfg(target_family = "unix")]
fn btopth(v: &[u8]) -> PathBuf {
    use std::os::unix::ffi::OsStrExt;
    return std::ffi::OsStr::from_bytes(v).into();
}

#[cfg(target_family = "windows")]
fn btopth(v: &[u8]) -> PathBuf { //todo (windows :( )
    use std::os::windows::ffi::OsStringExt;
    let r = v.iter();
    return std::ffi::OsString::from_wide(r).into();
}

fn read_header(data: &mut Vec<u8>) -> (usize, PathBuf) {
    let (v, rest) = data.split_at(std::mem::size_of::<usize>());
    let (mut h, rest) = rest.split_at(NAME_LEN);
    let n = usize::from_be_bytes(v.try_into().unwrap());
    let mut pwd = Vec::new();
    h.read_until(0, &mut pwd).expect("Could not read last path!");
    let s = btopth(&pwd[..pwd.len()-1]);
    *data = rest.to_vec();

    return (n, s);
}

#[allow(dead_code)] //Needed bc its used in vg
pub fn open_last() -> Option<(PathBuf, Vec<Match>)> {
    let h = home::home_dir().expect("Could not find home dir.").join(LAST_PATH);

    let s = std::fs::read(h);
    
    let mut s = match s {
        Err(_) => return None, //No last file
        Ok(v) => v,
    };

    let (v,h) = read_header(&mut s);
    if v != VERSION { return None };

    let mut m = vec![];
    let mut i = 0;
    while s.len() >= DATA_LEN*(i+1) {
        let r = &s[DATA_LEN*i..DATA_LEN*(i+1)];
        m.push(r.into());
        i+=1;
    }

    return Some((h, m));
}

#[allow(dead_code)]
pub fn display(pwd: PathBuf, result: &Vec<Match>, color: bool) {
    let curpwd = std::env::current_dir().expect("Could not find current directory");
    let p = match pwd.as_os_str().len() == 0 || pwd == curpwd {
        true => "this directory",
        false => match pwd.to_str() {
            Some(s) => s,
            None => "{unprintable_string}",
        },
    };
    if color {
        println!("Within \x1b[35m{}\x1b[39m:", p);
    } else {
        println!("Within {}:", p);
    }
    if color {
        let mut i = 0;
        for m in result {
            match i%2 {
                0 => println!("\x1b[31m{}\x1b[39m {}", i, m.disp()),
                1 => println!("\x1b[31m{}\x1b[39m \x1b[1m{}\x1b[0m", i, m.disp()),
                _ => panic!("CPU borken :("),
            };
            i+=1;
        }
    } else {
        let mut i = 0;
        for m in result {
            match i%2 {
                0 => println!("{} {}", i, m),
                1 => println!("{} {}", i, m),
                _ => panic!("CPU borken :("),
            };
            i+=1;
        }
    }
}

#[allow(dead_code)]
pub fn display_head(pwd: PathBuf, color: bool) {
    let curpwd = std::env::current_dir().expect("Could not find current directory");
    let p = match pwd.as_os_str().len() == 0 || pwd == curpwd {
        true => "this directory",
        false => match pwd.to_str() {
            Some(s) => s,
            None => "{unprintable_string}",
        },
    };
    if color {
        println!("Within \x1b[35m{}\x1b[39m:", p);
    } else {
        println!("Within {}:", p);
    }
}
#[allow(dead_code)]
pub fn display_once(result: &Match, color: bool) {
    static mut I:i32 = 0;
    unsafe {
        if color {
            match I%2 {
                0 => println!("\x1b[31m{}\x1b[39m {}", I, result.disp()),
                1 => println!("\x1b[31m{}\x1b[39m \x1b[1m{}\x1b[0m", I, result.disp()),
                _ => panic!("CPU borken :("),
            };
            I+=1;
        } else {
            match I%2 {
                0 => println!("{} {}", I, result),
                1 => println!("{} {}", I, result),
                _ => panic!("CPU borken :("),
            };
            I+=1;
        }
    }
}

pub fn last(color: bool) {
    let (r,s) = open_last().expect("No last file for user! Use 'cg' to create a last file.");
    display(r, &s, color);
}

pub fn color(color: &String) -> bool {
    match color.as_str() {
        "always" => true,
        "yes" => true,
        "never" => false,
        "no" => false,
        _ => panic!("{} is not a valid color setting", color)
    }
}

pub mod command {
    use std::process::{Command,Output,Stdio,Child, ExitStatus};

    pub type Cmd = (String, Vec<String>);

    fn build(command: String, args: Vec<String>) -> Command {
        let mut output = Command::new(command);
    
        for i in args {
            output.arg(i);
        }
        return output;
    }
    
    ///Call the first command in a call chain
    fn begin(command: String, args: Vec<String>) -> Child {
        return build(command, args).stdout(Stdio::piped()).spawn().expect("Command could not be executed.");
    }
    ///Call the first command in a call chain
    fn blind_begin(command: String, args: Vec<String>) -> Child {
        return build(command, args).spawn().expect("Command could not be executed.");
    }
    /// Links the first command's ouput to the second's input, then starts the second command.
    /*fn link(first: Child, command: String, args: Vec<String>) -> Child {
        //first.stdout(Stdio::piped());
        return build(command,args).stdin(first.stdout.unwrap()).stdout(Stdio::piped()).spawn().expect("Failed command");
    }*/
    ///Finishes a call stack
    fn finish(last: Child) -> Result<Output, std::io::Error> {
        return last.wait_with_output(); //todo!
    }
    ///Finishes a call stack
    fn blind_finish(last: &mut Child) -> Result<ExitStatus, std::io::Error> {
        return last.wait();
    }

    #[allow(dead_code)]
    /// The full call
    pub fn call(command: Cmd) -> Result<Output, std::io::Error> {
        finish(begin(command.0.to_string(), command.1))
    }
    #[allow(dead_code)]
    pub fn blind_call(command: Cmd) -> Result<ExitStatus, std::io::Error> {
        blind_finish(&mut blind_begin(command.0.to_string(), command.1))
    }
    
}