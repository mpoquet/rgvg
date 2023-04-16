use super::framework::{Entry, Formatter, Name, Argument, DefaultValue, Convertible, Transformable};
use super::Args;
use std::collections::{BTreeSet};


#[derive(Clone)] 
pub struct Grepper {
    /// A regular expression used for searching.
    regex_pattern: Entry,
    /// A file or directory to search. Directories may be searched recursively.
    file: Entry,
    /// Case sensitivity flag
    casei: Entry,
    /// The default arguments. Do not use spaces within the args!
    default_args: &'static str,
}

pub const GREP: Grepper = Grepper {
    regex_pattern: Entry { 
        defaults_to: DefaultValue::Mandatory,
        format: (Formatter::Default, Formatter::Default),
        target_name: Name::Blank(0),
        target_type: Argument::Text(None),
    },
    file: Entry {
        defaults_to: DefaultValue::Skip,
        format: (Formatter::Default, Formatter::Default),
        target_name: Name::Blank(1),
        target_type: Argument::PathPattern(None),
    },
    casei: Entry::ignore(),/*{
        defaults_to: DefaultValue::Skip,
        source: SourceFormatter::Default,
        target_name: Name::Short('i'),
        target_type: Argument::BooleanFlag(None),
    }*/
    default_args:  "--color=always -Hnr",
};

impl Convertible<Args> for Grepper {
    /// Yipeee ^-^
    /// 
    /// First we resolve positions,
    ///   Try to know where each element will be.
    ///   As a rule of thumb, we'd rather want elements with names to be placed later. 
    ///   Not that this would matter that much, usually!
    ///   
    ///   What we do here, is log which arguments have a fixed position, 
    ///     then throw all the non-ordered ones after.
    ///   To optimize the whole thing, we generate the arguments in the same time;
    ///     throw the non-positionals in a vec, and the positionals in a tree. 
    fn populate(&mut self, with: Args) -> BTreeSet<Entry> {
        let mut r: BTreeSet<Entry> = BTreeSet::new();
        self.regex_pattern.fill(&with.regex_pattern);
        self.file.fill(&with.file);
        self.casei.fill(&with.casei);
        r.insert(self.regex_pattern.clone());
        r.insert(self.file.clone());
        r.insert(self.casei.clone());

        return r;
    }
    fn generate(&self, with: BTreeSet<Entry>) -> Vec<String> {
        let mut r: Vec<String> = Vec::new();

        for i in with {
            r.extend(i.transform());
        }
        r.extend(self.default_args.split(" ").map(|c| c.to_string()).collect::<Vec<String>>());
        return r;
    }
}