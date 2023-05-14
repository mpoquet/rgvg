use std::{path::PathBuf, process::exit};

use crate::common::{Match, NAME_LEN, MATCH_LEN, VERSION, LAST_PATH, self};

use regex::Regex;

type Item = (usize,&'static str,&'static str);

//rg the ./documents --color=always --line-number -H --no-heading
//ggrep the ./documents --color=always -Hnr --exclude=t.txt --exclude-dir=edge
//ugrep the ./documents -rn --color=always

pub struct OutputFormat {
    /// The filename. 
    filename: Item,
    /// Display line number.
    line: Item,
    /// Display global position in file. Points to the start of the line matched.
    // char: Item,
    /// The matched LINE itself.
    matched: Item,
    //// True if the matched STRING is highlighted. If false, finding the column (of the first matched string) will require running a regex search.
    // is_match_highlighted: bool,
}




trait Restrict<T> {
    fn restrict(source: T, max: usize) -> Self;
}
impl Restrict<&str> for String {
    fn restrict(source: &str, max: usize) -> Self {
        let top = std::cmp::min(max, source.len());

        let mut j = top;
        while j > 0 && !source.is_char_boundary(j) {
            j-=1;
        }
        let result = source[..j].to_owned();
        return result;
    }
}

fn strip(text: &str) -> String {
    let r = Regex::new(r"\u{1b}\[([0-9;]*)[a-zA-Z]").unwrap();
    return r.replace_all(text, "").to_string();
}

fn create_outputformat(format: OutputFormat) -> Regex {
    let mut c: Vec<(usize, String)> = Vec::new();
    c.push((format.filename.0, format.filename.1.to_string() + r"(.+)" + format.filename.2));
    c.push((format.line.0, format.line.1.to_string() + r"(\d+)" + format.line.2));
    c.push((format.matched.0, format.matched.1.to_string() + r"(.+)" + format.matched.2));

    c.sort_by(|a, b| return a.0.cmp(&b.0));

    let mut q = String::new();
    for i in c {
        q += &i.1;
    }

    return Regex::new(&q).unwrap();
}

pub fn read(format: OutputFormat, text: &str, color: bool) -> Vec<Match> {
    let r = create_outputformat(format);
    let mut matches = Vec::new();

    if color {
        for m in r.captures_iter(text) {
            matches.push(Match {
                filename: String::restrict(&m[1], NAME_LEN),
                line: m[2].parse().expect("Unreadable line number"), //&("Unreadable line number".to_owned() + &m[0])
                matched: String::restrict(strip(&m[3]).as_str(), MATCH_LEN),
            });
            //println!("{} {}", matches.last().unwrap().matched.0.len(), matches.last().unwrap().matched);
        }
    } else {
        for m in r.captures_iter(text) {
            matches.push(Match {
                filename: String::restrict(&m[1], NAME_LEN),
                line: m[2].parse().expect("Unreadable line number"), //&("Unreadable line number".to_owned() + &m[0])
                matched: String::restrict(&m[3], MATCH_LEN),
            });
            //println!("{} {}", matches.last().unwrap().matched.0.len(), matches.last().unwrap().matched);
        }
    }

    return matches;
}

pub fn read_display(format: OutputFormat, text: &str, color: bool) -> Vec<Match> {
    common::display_head(PathBuf::new(), color);
    let r = create_outputformat(format);
    let mut matches = Vec::new();

    if color {
        for m in r.captures_iter(text) {
            let m = Match {
                filename: String::restrict(&m[1], NAME_LEN),
                line: m[2].parse().expect("Unreadable line number"), //&("Unreadable line number".to_owned() + &m[0])
                matched: String::restrict(strip(&m[3]).as_str(), MATCH_LEN),
            };
            common::display_once(&m, color);
            matches.push(m);
            //println!("{} {}", matches.last().unwrap().matched.0.len(), matches.last().unwrap().matched);
        }
    } else {
        for m in r.captures_iter(text) {
            let m = Match {
                filename: String::restrict(&m[1], NAME_LEN),
                line: m[2].parse().expect("Unreadable line number"), //&("Unreadable line number".to_owned() + &m[0])
                matched: String::restrict(&m[3], MATCH_LEN),
            };
            common::display_once(&m, color);
            matches.push(m);
            //println!("{} {}", matches.last().unwrap().matched.0.len(), matches.last().unwrap().matched);
        }
    }

    return matches;
}

///todo! voir le cout performance de la couleur
pub const GREP: OutputFormat = OutputFormat {
    filename: (0, "", ""),
    line: (1, ":", ":"),
    matched: (2, "", "\n"),
};

pub const RIPGREP: OutputFormat = OutputFormat {
    filename: (0, r"", r""),
    line: (1, r":", r":"),
    matched: (2, "", "\n"),
};

pub const UGREP: OutputFormat = OutputFormat {
    filename: (0, r"", r""),
    line: (1, r":", r":"),
    matched: (2, r"", "\n"),
};

pub fn picker(tool: &str) -> OutputFormat {
    match tool {
        "grep" => self::GREP,
        "ripgrep" => self::RIPGREP,
        "ugrep" => self::UGREP,
        _ => panic!("Unkown tool requested"),
    }
}

#[allow(dead_code)]
pub fn display(result: &Vec<Match>, color: bool) {
    crate::common::display(PathBuf::new(), result, color);
}

#[cfg(target_family = "unix")]
fn pthtob(v: PathBuf) -> Vec<u8> {
    use std::os::unix::prelude::OsStrExt;
    return v.as_os_str().as_bytes().to_vec();
}
#[cfg(target_family = "windows")]
fn pthtob(v: PathBuf) -> Vec<u8> { //! Untested
    use std::os::windows::prelude::OsStrExt;
    return v.encode_wide().map(|v| v.to_be_bytes()).collect().concat();
}

fn header() -> Vec<u8> {
    let mut s: Vec<u8> = (VERSION as usize).to_be_bytes().to_vec(); //Version tag
    let v = std::env::current_dir().expect("Could not find current directory");
    let c = pthtob(v);
    let l = c.len();
    s.extend(c);
    s.extend(std::iter::repeat(0).take(NAME_LEN - l));
    return s;
}


pub fn write(result: &Vec<Match>) {
    //println!("Writing to \x1b[35m*${{HOME}}/.rgvg_last\x1b[39m...");
    let v: Vec<Vec<u8>> = result.iter().map(|m| m.into()).collect();
    let v = v.concat();
    let mut s: Vec<u8> = header();

    s.extend(v);
    let h = home::home_dir().expect("Could not find home dir.").join(LAST_PATH);
    std::fs::write(h, s).expect("Could not write to history file.");
}