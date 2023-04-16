use std::fmt::{Display, write};

use regex::Regex;

type Item = (usize,&'static str,&'static str);

//rg 'the' ./documents --color=always --line-number -H 
//grep 'the' ./documents --color=always -Hn -r
//ugrep "the" ./documents -rn --color=always


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

#[derive(Debug)]
pub struct Match {
    filename: String,
    line: String,
    matched: String,
}
impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[35m{} \x1b[34m{} \x1b[32m{}\x1b[0m", self.filename, self.line, self.matched)
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
        matches.push(Match {
            filename: m.name("f").unwrap().as_str().to_string(),
            line: m.name("l").unwrap().as_str().to_string(),
            matched: strip(m.name("m").unwrap().as_str()),
        });
        //println!("{:?}", matches.last().unwrap())
    }

    return matches;
}

pub const GREP: OutputFormat = OutputFormat {
    filename: (0, "", ""),
    line: (1, ":", ":"),
    matched: (2, "", "\n"),
};
pub const RIPGREP: OutputFormat = OutputFormat {
    filename: (0, r"\[0m\[35m", r"\[0m"),
    line: (1, r":\[0m\[32m", r"\[0m:"),
    matched: (2, "", "\n"),
};

pub fn display(result: Vec<Match>) {
    let mut i = 0;
    for m in result {
        match i%2 {
            0 => println!("\x1b[31m{}\x1b[39m: {}", i, m),
            1 => println!("\x1b[1m\x1b[31m{}\x1b[39m: {}\x1b[0m", i, m),
            _ => panic!("CPU borken :("),
        };
        i+=1;
    }
}