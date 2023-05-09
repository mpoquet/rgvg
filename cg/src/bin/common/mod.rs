use std::{fmt::{Display}, path::PathBuf, io::BufRead};

pub const LAST_PATH: &'static str = ".rgvg_last";
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
                true => "\x1b[1m\x1b[31m🆇\x1b[39m\x1b[0m",
            }
        }
        write!(f, "\x1b[35m{}{} \x1b[34m{} \x1b[32m{}{}\x1b[0m", self.filename, disp_flag(NAME_LEN == self.filename.len()), self.line, self.matched, disp_flag(MATCH_LEN == self.matched.len()))
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

pub fn display(pwd: PathBuf, result: &Vec<Match>) {
    let curpwd = std::env::current_dir().expect("Could not find current directory");
    let p = match pwd.as_os_str().len() == 0 || pwd == curpwd {
        true => "this directory",
        false => match pwd.to_str() {
            Some(s) => s,
            None => "{unprintable_string}",
        },
    };

    println!("Within \x1b[35m{}\x1b[39m:", p);
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