use std::fmt::{Display};

use regex::Regex;

type Item = (usize,&'static str,&'static str);

//rg the ./documents --color=always --line-number -H --no-heading
//ggrep the ./documents --color=always -Hnr --exclude=t.txt --exclude-dir=edge
//ugrep the ./documents -rn --color=always

const NAME_LEN: usize = 512;
const MATCH_LEN: usize = 64;
const UTF_BYTES: usize = 4;

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
trait Bleed {

}
#[derive(Debug)]
pub struct Overflowable<T: Clone>(Vec<T>,usize);
impl<T: Clone> Overflowable<T> {
    fn new(value: &[T], max: usize) -> Self {
        return Overflowable(value[0..std::cmp::min(value.len(),max)].to_vec(), max);
    }
}
impl<T: Clone> Display for Overflowable<T> where String: FromIterator<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 == self.0.len() {
            true => write!(f, "{}\x1b[1m\x1b[31m◀︎\x1b[0m", String::from_iter(self.0.clone())),
            false => write!(f, "{}", String::from_iter(self.0.clone())),
        }
    }
}
// impl From<Overflowable<char>> for Vec<u8> {
//     fn from(value: Overflowable<char>) -> Self {
//         let mut buffer = [0_u8; 4];
//         let mut bytes = vec![];
//         for c in value.0 {
//             bytes.extend(c.encode_utf8(&mut buffer).as_bytes());
//         }
//     }
// }
impl<T: Clone> From<Overflowable<T>> for Vec<u8> where Vec<u8>: FromIterator<T> {
    fn from(value: Overflowable<T>) -> Self {
        let c = value.0.len();
        let mut r = Vec::from_iter(value.0);
        r.extend(std::iter::repeat(b'0').take(value.1 - c));
        return r;
    }
}


/// Big rust L incoming
struct Thin<T>(T);
struct Thinter<T>(isize,Vec<T>);
impl<T> Iterator for Thinter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0+=1;
        match self.1.get(self.0 as usize) {
            None => None,
            Some(x) => Some(*x.to_owned()),
        }
    }
}
impl FromIterator<Thin<char>> for Vec<u8>  {
    fn from_iter<T: IntoIterator<Item = Thin<char>>>(iter: T) -> Self {
        let mut r = Vec::new();
        for c in iter {
            r.extend_from_slice(&u32::from(c.0).to_be_bytes());
        }
        return r;
    }
}
impl<V> FromIterator<V> for Thinter<Thin<V>> {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut r = vec![];
        for v in iter {
            r.push(Thin(v));
        }
        return Thinter(-1, r);
    }
}

impl From<Match> for Vec<u8> {
    fn from(value: Match) -> Self {
        let mut r: Vec<u8> = value.id.to_be_bytes().to_vec();
        r.extend(Vec::from(value.filename));
        r.extend(value.line.to_be_bytes().into_iter());
        //r.extend(value.matched.into());
        return r;
    }
}
#[derive(Debug)]
pub struct Match {
    id: usize,
    filename: Overflowable<char>,
    line: usize,
    matched: Overflowable<char>,
}
impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[31m{}\x1b[39m \x1b[35m{} \x1b[34m{} \x1b[32m{}\x1b[0m", self.id, self.filename, self.line, self.matched)
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
        let rf =  strip(m.name("f").unwrap().as_str());
        let rf: Vec<char> = rf.chars().collect(); //Collect by UFVs
        

        let lf = strip(m.name("l").unwrap().as_str());

        let mf = strip(m.name("m").unwrap().as_str());
        let mf: Vec<char> = mf.chars().collect();

        matches.push(Match {
            id: matches.len(),
            filename: Overflowable::new(&rf, NAME_LEN),
            line: lf.parse().expect("Unreadable line numbers"),
            matched: Overflowable::new(&mf, MATCH_LEN),
        });
        //println!("{} {}", matches.last().unwrap().matched.0.len(), matches.last().unwrap().matched);
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

pub fn display(result: &Vec<Match>) {
    let mut i = 0;
    for m in result {
        match i%2 {
            0 => println!("{}", m),
            1 => println!("\x1b[1m{}\x1b[0m", m),
            _ => panic!("CPU borken :("),
        };
        i+=1;
    }
}

pub fn write(result: Vec<Match>) {
     let f = std::fs::write::<std::path::PathBuf,Vec<u8>>(std::env::home_dir().expect("Could not find home dir."), result.iter().into().join(b"\n"));
}