use crate::common::command::Cmd;
use regex::Regex;
use std::collections::BTreeMap;
use std::fmt::{self, Display};
use std::path::PathBuf;

type Index = u8;

/// A name of the form -<short> or --<long>. The name sent as a command. Short and long as thus mutually excusive here.
#[derive(Clone, Debug)]
pub enum Name {
    /// A short name, format -<short>, i.e. -j
    Short(char),
    /// A long name, format --<long>, i.e. --exclude
    Long(String),
    /// Results of a limit in consts, similar to long
    LongC(&'static str),
    /// A blank name, positional. The position is only in regard to other blanks.
    /// The values used for position are not nearly as important as their order (think like z-level for 2d renderers).
    Blank(Index),
    /// Skip this entry
    Undefined,
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Argument {
    /// A collection type. Hopefully.
    CollectionText(Option<Vec<String>>),
    /// A string designating a file (or a PathBuff)
    PathPattern(Option<PathBuf>),
    /// A list of file designators
    CollectionPathPattern(Option<Vec<PathBuf>>),
    /// A regular string
    Text(Option<String>),
    /// Either there or not.. What do the stars say, my dear pippin, what do they say? - That we fight the good cause, merry. That we will see each other in the end.
    BooleanFlag(Option<bool>),
    /// A numeric value
    Number(Option<isize>),
    /// Nothing
    Empty(Option<()>),
}
#[derive(Clone, Debug)]
pub enum Formatter {
    /// [in,out] No formatting
    Default,
    /// [in,out] A prefix,
    Prefix(&'static str),
}
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum DefaultValue {
    /// CANNOT be ommited
    Mandatory,
    /// Just forgedaboutit
    Skip,
    /// provide a default.
    Default(Argument),
}
#[derive(Clone, Debug)]
pub struct Entry {
    pub defaults_to: DefaultValue,
    pub format: (Formatter, Formatter),
    pub target_name: Name,
    pub target_type: Argument,
}
pub struct Error {}

pub trait Transform<T> {
    fn transform(&mut self, value: &T);
}
pub trait Generate {
    /// Vec and not option, because of collection arguments.
    fn generate(self) -> Vec<String>;
}
trait Vectorize: Sized {
    fn vec(self) -> Vec<String>; //
}
pub trait Transformable<U> {
    /// Takes an empty entry and fills it.
    fn fill(&mut self, with: &U);
}
pub trait Expand {
    type Key;
    type Item;
    fn expand(&mut self, key: Self::Key, value: Self::Item)
    where
        Self::Key: Ord;
    #[allow(unused_variables)]
    fn expand_field(&mut self, value: &Self::Item) {}
}

impl Default for Name {
    fn default() -> Self {
        return Name::Undefined;
    }
}
impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Name::Short(c) => write!(f, "-{}", c),
            Name::Long(s) => write!(f, "--{}", s),
            Name::Blank(_) => fmt::Result::Ok(()), //Nothing displayed!
            _ => panic!("Unsupported name type!"),
        }
    }
}
impl TryFrom<(regex::Match<'_>, &str)> for Name {
    type Error = Error;
    fn try_from(value: (regex::Match, &str)) -> Result<Self, Self::Error> {
        let s = value.0.as_str();
        match value.1 {
            "d1" => Ok(Name::Blank(s[1..].parse().unwrap())),
            "d2" => Ok(Name::Short(s.chars().nth(1).unwrap())),
            "d3" => Ok(Name::Long(s[2..].to_string())),
            _ => Err(Error {}),
        }
    }
}

impl Name {
    fn cleanup(&mut self) {
        match self {
            Name::LongC(s) => *self = Name::Long(s.to_string()),
            _ => return,
        };
    }
    fn name(&self, arglist: &mut Vec<String>) {
        match self {
            Name::Blank(_) => return,
            Name::Undefined => return,
            _ => {
                for c in arglist {
                    if c.len() > 0 {
                        *c = self.to_string() + "=" + c;
                    } else {
                        *c = self.to_string() + c;
                    }
                }
            }
        }
    }
}

/// All the `From<T> for argument` help use default values. they are, in no way, needed.
impl From<DefaultValue> for Argument {
    fn from(value: DefaultValue) -> Self {
        match value {
            DefaultValue::Skip => Argument::Empty(Some(())),
            DefaultValue::Default(x) => x,
            DefaultValue::Mandatory => Argument::Empty(None),
        }
    }
}

impl From<String> for Argument {
    fn from(value: String) -> Self {
        return Argument::Text(Some(value));
    }
}
impl From<PathBuf> for Argument {
    fn from(value: PathBuf) -> Self {
        return Argument::PathPattern(Some(value));
    }
}
impl From<&'static str> for Argument {
    fn from(value: &'static str) -> Self {
        return Argument::Text(Some(value.to_string()));
    }
}

/// Transform functions are the obligatory path from raw clap data into arguments.
/// To implement Transform<T> for a new T, you need to implement the transform function.
///   That is, provide a match against all valid transformation fields, to cast into self.
///   You should always provide a case to cast into empty. Thankfully, that cast is always the same.
impl Transform<String> for Argument {
    fn transform(&mut self, value: &String) {
        match self {
            Argument::Text(x) => *x = Some(value.to_owned()),
            Argument::Empty(x) => *x = Some(()),
            _ => panic!("Unspported transformation!"),
        }
    }
}
impl Transform<PathBuf> for Argument {
    fn transform(&mut self, value: &PathBuf) {
        match self {
            Argument::PathPattern(x) => *x = Some(value.to_path_buf()),
            Argument::Text(x) => *x = Some(value.display().to_string()),
            Argument::Empty(x) => *x = Some(()),
            _ => panic!("Unspported transformation!"),
        }
    }
}
impl Transform<bool> for Argument {
    fn transform(&mut self, value: &bool) {
        match self {
            Argument::BooleanFlag(x) => *x = Some(*value),
            Argument::Text(x) => *x = Some(value.to_string()),
            Argument::Empty(x) => *x = Some(()),
            _ => panic!("Unspported transformation!"),
        }
    }
}

impl Transform<Option<PathBuf>> for Argument {
    fn transform(&mut self, value: &Option<PathBuf>) {
        match self {
            Argument::PathPattern(x) => match value {
                Some(p) => *x = Some(p.to_path_buf()),
                None => *x = None,
            },
            Argument::Text(x) => match value {
                Some(p) => *x = Some(p.display().to_string()),
                None => *x = None,
            },
            Argument::Empty(x) => *x = Some(()),
            _ => panic!("Unspported transformation!"),
        }
    }
}
impl Transform<Option<String>> for Argument {
    fn transform(&mut self, value: &Option<String>) {
        match self {
            Argument::Text(x) => match value {
                Some(p) => *x = Some(p.to_owned()),
                None => *x = None,
            },
            Argument::Empty(x) => *x = Some(()),
            _ => panic!("Unspported transformation!"),
        }
    }
}
impl Transform<Vec<String>> for Argument {
    fn transform(&mut self, value: &Vec<String>) {
        match self {
            Argument::CollectionText(x) => *x = Some(value.to_owned()),
            Argument::Empty(x) => *x = Some(()),
            _ => panic!("Unspported transformation!"),
        }
    }
}
impl Transform<Vec<PathBuf>> for Argument {
    fn transform(&mut self, value: &Vec<PathBuf>) {
        match self {
            Argument::CollectionPathPattern(x) => *x = Some(value.to_owned()),
            Argument::CollectionText(x) => {
                *x = Some(value.iter().map(|p| p.display().to_string()).collect())
            }
            Argument::Empty(x) => *x = Some(()),
            _ => panic!("Unspported transformation!"),
        }
    }
}

/// This set of transform functions is used in destination formatting.
///   When adding a new (dest) formatter, you need to edit each implementation.
///   When adding compatibility for a new type, you need to add a new implementation
#[allow(unreachable_patterns)] //Formatters may in the future not be supported as destination formatters
impl Transform<Formatter> for Vec<String> {
    fn transform(&mut self, value: &Formatter) {
        match value {
            Formatter::Default => return, // No change, this line is the same everywhere.
            Formatter::Prefix(s) => *self = self.iter().map(|c| s.to_string() + c).collect(),
            _ => panic!("Unsupported formatter for destination type Vec<String>."),
        }
    }
}
impl Transform<Vec<Entry>> for Vec<String> {
    fn transform(&mut self, value: &Vec<Entry>) {
        let mut c = vec![];
        for i in value {
            let mut e = i.clone().generate();
            e.transform(&i.format.1);
            i.target_name.name(&mut e);
            c.extend(e);
        }
        // We assume formatters as identical for a same flag!
        self.extend(c);
    }
}

#[allow(unreachable_patterns)]
impl Generate for Entry {
    fn generate(self) -> Vec<String> {
        let defaults = &self.defaults_to.clone().into();
        match self.target_type {
            Argument::BooleanFlag(x) => match x {
                None => vec![], //Should not occur
                Some(true) => vec!["".to_string()],
                Some(false) => vec![],
            },
            Argument::Text(x) => optional_vectorization(x, defaults),
            Argument::PathPattern(x) => optional_vectorization(x, defaults),
            Argument::Empty(x) => optional_vectorization(x, defaults),
            Argument::CollectionText(x) => optional_vectorization(x, defaults),
            Argument::CollectionPathPattern(x) => optional_vectorization(x, defaults),
            Argument::Number(x) => optional_vectorization(x, defaults),
            _ => panic!("Unsupported type: {:?}", self),
        }
    }
}

fn optional_vectorization<T: Vectorize>(v: Option<T>, defaults: &Argument) -> Vec<String> {
    match v {
        Some(n) => n.vec(),
        None => match defaults {
            Argument::Empty(Some(())) => Vec::new(), //Skipped
            Argument::Empty(None) => panic!("Mandatory argument was not provided"),
            _ => Entry {
                defaults_to: DefaultValue::Mandatory,
                format: (Formatter::Default, Formatter::Default),
                target_name: Name::Undefined, //Irrelevant
                target_type: defaults.clone(),
            }
            .generate(),
        },
    }
}
impl<T> Vectorize for Vec<T>
where
    T: Vectorize,
{
    fn vec(self) -> Vec<String> {
        //, name: Name, defaults: DefaultValue
        let mut r: Vec<String> = Vec::new();
        for c in self {
            r.extend(c.vec());
        }
        return r;
    }
}

impl Vectorize for String {
    fn vec(self) -> Vec<String> {
        vec![self]
    }
}
impl Vectorize for PathBuf {
    fn vec(self) -> Vec<String> {
        return vec![self.display().to_string()];
    }
}
impl Vectorize for isize {
    fn vec(self) -> Vec<String> {
        return Vec::new();
    }
}
impl Vectorize for () {
    fn vec(self) -> Vec<String> {
        return vec![];
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            //Reminder; we *expect* entries to be diferrent.
            Name::Blank(i) => match &other {
                Name::Blank(j) => i == j,
                Name::Long(_) => false,
                Name::LongC(_) => false,
                Name::Short(_) => false,
                Name::Undefined => false,
            },
            Name::Long(s) => match &other {
                Name::Blank(_) => false,
                Name::Long(t) => s == t,
                Name::LongC(t) => s == t,
                Name::Short(_) => false,
                Name::Undefined => false,
            },
            Name::LongC(s) => match &other {
                Name::Blank(_) => false,
                Name::Long(t) => s == t,
                Name::LongC(t) => s == t,
                Name::Short(_) => false,
                Name::Undefined => false,
            },
            Name::Short(c) => match &other {
                Name::Blank(_) => false,
                Name::Long(_) => false,
                Name::LongC(_) => false,
                Name::Short(d) => c == d,
                Name::Undefined => false,
            },
            Name::Undefined => match &other {
                Name::Blank(_) => false,
                Name::Long(_) => false,
                Name::LongC(_) => false,
                Name::Short(_) => false,
                Name::Undefined => true,
            },
        }
    }
}
impl Eq for Name {}
impl PartialOrd for Name {
    /// Conventional order: command <blanks> -<shorts> --<longs>. Why? good question.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match &self {
            //Reminder; we *expect* entries to be diferrent.
            Name::Blank(i) => match &other {
                Name::Blank(j) => i.partial_cmp(j),
                Name::Long(_) => Some(std::cmp::Ordering::Greater),
                Name::Short(_) => Some(std::cmp::Ordering::Greater),
                _ => None,
            },
            Name::Long(s) => match &other {
                Name::Blank(_) => Some(std::cmp::Ordering::Less),
                Name::Long(t) => s.partial_cmp(t),
                Name::Short(_) => Some(std::cmp::Ordering::Greater),
                _ => None,
            },
            Name::Short(c) => match &other {
                Name::Blank(_) => Some(std::cmp::Ordering::Less),
                Name::Long(_) => Some(std::cmp::Ordering::Less),
                Name::Short(d) => c.partial_cmp(d),
                _ => None,
            },
            _ => None,
        }
    }
}
impl Ord for Name {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //We treat undefined like infinity. It's not a problem if they are equal then, as we expect undefined to get ignored.
        //! THis implementation poses a problem for argument joining! to keep in mind.
        match &self {
            Name::Blank(i) => match &other {
                Name::Blank(j) => i.cmp(j),
                Name::Long(_) => std::cmp::Ordering::Greater,
                Name::LongC(_) => std::cmp::Ordering::Greater,
                Name::Short(_) => std::cmp::Ordering::Greater,
                Name::Undefined => std::cmp::Ordering::Less,
            },
            Name::Long(s) => match &other {
                Name::Blank(_) => std::cmp::Ordering::Less,
                Name::Long(t) => s.cmp(t),
                Name::LongC(t) => s.cmp(&t.to_string()),
                Name::Short(_) => std::cmp::Ordering::Greater,
                Name::Undefined => std::cmp::Ordering::Less,
            },
            Name::LongC(s) => match &other {
                Name::Blank(_) => std::cmp::Ordering::Less,
                Name::Long(t) => s.cmp(&t.as_str()),
                Name::LongC(t) => s.cmp(t),
                Name::Short(_) => std::cmp::Ordering::Greater,
                Name::Undefined => std::cmp::Ordering::Less,
            },
            Name::Short(c) => match &other {
                Name::Blank(_) => std::cmp::Ordering::Less,
                Name::Long(_) => std::cmp::Ordering::Less,
                Name::LongC(_) => std::cmp::Ordering::Less,
                Name::Short(d) => c.cmp(d),
                Name::Undefined => std::cmp::Ordering::Less,
            },
            Name::Undefined => match &other {
                Name::Blank(_) => std::cmp::Ordering::Greater,
                Name::Long(_) => std::cmp::Ordering::Greater,
                Name::LongC(_) => std::cmp::Ordering::Greater,
                Name::Short(_) => std::cmp::Ordering::Greater,
                Name::Undefined => std::cmp::Ordering::Equal,
            },
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        return self.target_name == other.target_name;
    }
}
impl Eq for Entry {}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.target_name.partial_cmp(&other.target_name);
    }
}
impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.target_name.cmp(&other.target_name);
    }
}

impl TryFrom<&str> for Entry {
    type Error = Error;
    /// Formatter for args, defined as:
    ///   With d the default flag, d ∈ {λ,!,<?*>}
    ///     Where:
    ///       - λ designs an empty string,
    ///       - ! the ! symbol,
    ///       - and <?*> any string, surrounded by the symbols < and >.
    ///     Corresponds to:
    ///       - λ => DefaultValue::Skip
    ///       - ! => DefaultValue::Mandatory
    ///       - <?*> => DefaultValue::Default(?*)
    ///
    ///   With n the target name, n ∈ {#?i, -?, --?*}
    ///     Where:
    ///       - #?i designs the # symbol followed by any number,
    ///       - -? the - symbol followed by any character,
    ///       - and --?* the -- symbol followed by any string.
    ///     Corresponds to:
    ///       - #?i => Name::Blank(?i)
    ///       - -? => Name::Short(?)
    ///       - --?* => Name::Long(?*)
    ///   With t the target type (or kind), t ∈ {λ} ∪ [.S.] and S = {str}
    ///     λ corresponds to a boolean flag
    ///     Corresponds to:
    ///      - str => Option<String>
    ///      - path => Option<PathBuff>
    ///
    ///   With s = {_} where _ is any string.
    ///     Corresponds to the source item, as read by clap.
    ///
    /// A format as: ndst
    ///   
    /// Examples: #1<->{path}[path]
    ///           -i{casei}
    ///           #0!{pattern}[str]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let r = Regex::new(r"((?P<n1>#\d{1,3})|(?P<n2>-\pL)|(?P<n3>--\pL+))((?P<d1>!)|(?P<d2><[\pL-]*>)|(?P<d3>))(?P<s>\{\pL+\})((?P<t1>\[\pL+\])|(?P<t2>))").unwrap();
        let _c = r.captures(value);

        return Err(Error {});
    }
}

impl<U> Transformable<U> for Entry
where
    Argument: Transform<U>,
{
    fn fill(&mut self, with: &U) {
        self.target_name.cleanup();
        self.target_type.transform(with);
    }
    /*fn conform(self) -> Vec<String> {
        let mut replacement = self.clone();
        match &replacement.defaults_to {
            DefaultValue::Skip => return self.generate(),
            DefaultValue::Mandatory => {
                let e = self.generate();
                if e.len() == 0 {
                    panic!("Mandatory argument was not filled");
                } else {
                    return e;
                }
            }
            DefaultValue::Default(x) => {
                let e = self.generate();
                if e.len() == 0 {
                    replacement.target_type = x.clone();
                    return replacement.generate();
                } else {
                    return e;
                }
            }
        }
    }*/
}

impl Entry {
    #[allow(dead_code)]
    pub const fn ignore() -> Self {
        return Entry {
            defaults_to: DefaultValue::Skip,
            format: (Formatter::Default, Formatter::Default),
            target_name: Name::Undefined,
            target_type: Argument::Empty(None),
        };
    }
    pub const fn bool(name: Name) -> Self {
        return Entry {
            defaults_to: DefaultValue::Skip,
            format: (Formatter::Default, Formatter::Default),
            target_name: name,
            target_type: Argument::BooleanFlag(None),
        };
    }
}

impl Expand for BTreeMap<Name, Vec<Entry>> {
    type Key = Name;
    type Item = Entry;
    fn expand(&mut self, key: Self::Key, value: Self::Item) {
        if self.contains_key(&key) {
            self.get_mut(&key).unwrap().push(value);
        } else {
            self.insert(key, vec![value]);
        }
    }
    fn expand_field(&mut self, value: &Self::Item) {
        self.expand(value.clone().target_name, value.clone());
    }
}

pub trait Convertible<T> {
    /// Polulate entry with clap data, returns the ordered entry bundle
    fn populate(&mut self, with: &T) -> BTreeMap<Name, Vec<Entry>>;
    /// Takes clap data, and converts it to a command string.
    fn generate(&self, with: BTreeMap<Name, Vec<Entry>>) -> Cmd;
}
