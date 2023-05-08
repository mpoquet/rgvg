use std::fmt::{Display};

use regex::Regex;

type Item = (usize,&'static str,&'static str);

//rg the ./documents --color=always --line-number -H --no-heading
//ggrep the ./documents --color=always -Hnr --exclude=t.txt --exclude-dir=edge
//ugrep the ./documents -rn --color=always

const NAME_LEN: usize = 512;
const MATCH_LEN: usize = 512;

pub struct OutputFormat {
    /// The filename. Obviously mandatory.
    filename: Item,
    /// Display line number. At least one of line and char must be present
    line: Item,
    /// Display global position in file. Points to the start of the line matched.
    // char: Item,
    /// The matched LINE itself.
    matched: Item,
    //// True if the matched STRING is highlighted. If false, finding the column (of the first matched string) will require running a regex search.
    // is_match_highlighted: bool,
}

impl From<&Match> for Vec<u8> {
    fn from(value: &Match) -> Self {
        let mut r: Vec<u8> = Vec::from(value.filename.0.clone());
        r.extend(value.line.to_be_bytes().into_iter());
        r.extend(Vec::from(value.matched.0.clone()));
        return r;
    }
}
#[derive(Debug)]
pub struct Match {
    filename: (String, bool),
    line: usize,
    matched: (String, bool),
}
impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn disp_flag(b: bool) -> &'static str {
            match b {
                false => "",
                true => "\x1b[1m\x1b[31m🆇\x1b[39m\x1b[0m",
            }
        }
        write!(f, "\x1b[35m{}{} \x1b[34m{} \x1b[32m{}{}\x1b[0m", self.filename.0, disp_flag(self.filename.1), self.line, self.matched.0, disp_flag(self.matched.1))
    }
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
        let result = source[..j].to_string();
        // Pad the end to make sure it's the exact size.
        let result = result + &String::from_iter(std::iter::repeat('\x00').take(max - j)); 
        return result;
    }
}

fn strip(text: &str) -> String {
    let r = Regex::new(r"\u{1b}\[([0-9;]*)[a-zA-Z]").unwrap();
    return r.replace_all(text, "").to_string();
}

pub fn read(format: OutputFormat, text: &str) -> Vec<Match> {
    let mut c: Vec<(usize, String)> = Vec::new();
    c.push((format.filename.0, format.filename.1.to_string() + r"(?P<f>.*)" + format.filename.2));
    c.push((format.line.0, format.line.1.to_string() + r"(?P<l>(\d)*)" + format.line.2));
    c.push((format.matched.0, format.matched.1.to_string() + r"(?P<m>.*)" + format.matched.2));

    c.sort_by(|a, b| return a.0.cmp(&b.0));

    let mut q = String::new();
    for i in c {
        q += &i.1;
    }

    let r = Regex::new(&q).unwrap();
    let mut matches = Vec::new();

    for m in r.captures_iter(text) {
        let file_string = strip(m.name("f").unwrap().as_str());
        let max_flag_f = NAME_LEN < file_string.len();   

        let lf = strip(m.name("l").unwrap().as_str());

        let mf = strip(m.name("m").unwrap().as_str());
        let max_flag_m = NAME_LEN < mf.len();

        matches.push(Match {
            filename: (String::restrict(&file_string, NAME_LEN), max_flag_f),
            line: lf.parse().expect("Unreadable line numbers"),
            matched: (String::restrict(&mf, MATCH_LEN), max_flag_m),
        });
        //println!("{} {}", matches.last().unwrap().matched.0.len(), matches.last().unwrap().matched);
    }

    return matches;
}

///todo! voir le cout performance la couleur
pub const MACGREP: OutputFormat = OutputFormat {
    filename: (0, "", ""),
    line: (1, ":", ":"),
    matched: (2, "", "\n"),
};

pub const GREP: OutputFormat = OutputFormat {
    filename: (0, r"\[35m\[K", r"\[m\[K"),
    line: (1, r"\[36m\[K:\[m\[K\[32m\[K", r"\[m\[K\[36m\[K:\[m\[K"),
    matched: (2, "", "\n"),
};

pub const RIPGREP: OutputFormat = OutputFormat {
    filename: (0, r"\[0m\[35m", r"\[0m"),
    line: (1, r":\[0m\[32m", r"\[0m:"),
    matched: (2, "", "\n"),
};

pub const UGREP: OutputFormat = OutputFormat {
    filename: (0, r"\[1;35m", r"\[m\[36m"),
    line: (1, r":\[m\[1;32m", r"\[m\[36m:"),
    matched: (2, r"\[m", "\n"),
};

pub fn picker(tool: &str) -> OutputFormat {
    match tool {
        "grep" => self::GREP,
        "ripgrep" => self::RIPGREP,
        "ugrep" => self::UGREP,
        _ => panic!("Unkown tool requested"),
    }
}

pub fn display(result: &Vec<Match>) {
    let mut i = 0;
    for m in result {
        match i%2 {
            0 => println!("\x1b[31m{}\x1b[39m {}", i, m),
            1 => println!("\x1b[31m{}\x1b[39m \x1b[1m{}\x1b[0m", i, m),
            _ => panic!("CPU borken :("),
        };
        i+=1;
    }
}

pub fn write(result: &Vec<Match>) {
    let v: Vec<Vec<u8>> = result.iter().map(|m| m.into()).collect();
    let v = v.concat();
    let mut s: Vec<u8> = Vec::new();
    s.extend(NAME_LEN.to_be_bytes());
    s.extend(MATCH_LEN.to_be_bytes());

    s.extend(v);
    let h = home::home_dir().expect("Could not find home dir.").join(".rgvg_last");
    std::fs::write(h, s);
}