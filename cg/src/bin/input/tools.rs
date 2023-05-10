use super::framework::{Entry, Formatter, Name, Argument, DefaultValue, Convertible, Transformable, Expand, Transform};
use super::Args;
use std::collections::{BTreeMap};
use crate::common::command::Cmd;


#[derive(Clone)] 
pub struct Grepper {
    /// A regular expression used for searching.
    regex_pattern: Entry,
    /// A file or directory to search. Directories may be searched recursively.
    file: Entry,
    /// Case sensitivity flag
    casei: Entry,
    
    include_files: Entry,
    exclude_files: Entry,
    include_dir: Entry,
    exclude_dir: Entry,

    /// The default arguments. Do not use spaces within the args!
    default_args: &'static str,
    //: The command name
    command: &'static str,
}

///todo!
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
    casei: Entry::bool(Name::Short('i')), 
    include_files: Entry {
        defaults_to: DefaultValue::Skip,
        format: (Formatter::Default, Formatter::Default),
        target_name: Name::LongC("include"),
        target_type: Argument::CollectionText(None),
    },
    exclude_files: Entry {
        defaults_to: DefaultValue::Skip,
        format: (Formatter::Default, Formatter::Default),
        target_name: Name::LongC("exclude"),
        target_type: Argument::CollectionText(None),
    },
    include_dir: Entry {
        defaults_to: DefaultValue::Skip,
        format: (Formatter::Default, Formatter::Default),
        target_name: Name::LongC("include_dir"),
        target_type: Argument::CollectionText(None),
    },
    exclude_dir: Entry {
        defaults_to: DefaultValue::Skip,
        format: (Formatter::Default, Formatter::Default),
        target_name: Name::LongC("exclude_dir"),
        target_type: Argument::CollectionText(None),
    },
    default_args:  "-Hnr",
    command: "grep",
};

pub const RIPGREP: Grepper = Grepper {
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
    casei: Entry::bool(Name::Short('i')), 
    include_files: Entry {
        defaults_to: DefaultValue::Skip,
        format: (Formatter::Default, Formatter::Prefix("")),
        target_name: Name::Short('g'),
        target_type: Argument::CollectionText(None),
    },
    exclude_files: Entry {
        defaults_to: DefaultValue::Skip,
        format: (Formatter::Default, Formatter::Prefix("!")),
        target_name: Name::Short('g'),
        target_type: Argument::CollectionText(None),
    },
    include_dir: Entry {
        defaults_to: DefaultValue::Skip,
        format: (Formatter::Default, Formatter::Prefix("")),
        target_name: Name::Short('g'),
        target_type: Argument::CollectionText(None),
    },
    exclude_dir: Entry {
        defaults_to: DefaultValue::Skip,
        format: (Formatter::Default, Formatter::Prefix("!")),
        target_name: Name::Short('g'),
        target_type: Argument::CollectionText(None),
    },
    default_args:  "-Hn --no-heading",
    command: "rg",
};

pub const UGREP: Grepper = Grepper {
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
    casei: Entry::bool(Name::Short('i')), 
    include_files: Entry {
        defaults_to: DefaultValue::Skip,
        format: (Formatter::Default, Formatter::Default),
        target_name: Name::LongC("include"),
        target_type: Argument::CollectionText(None),
    },
    exclude_files: Entry {
        defaults_to: DefaultValue::Skip,
        format: (Formatter::Default, Formatter::Default),
        target_name: Name::LongC("exclude"),
        target_type: Argument::CollectionText(None),
    },
    include_dir: Entry {
        defaults_to: DefaultValue::Skip,
        format: (Formatter::Default, Formatter::Default),
        target_name: Name::LongC("include-dir"),
        target_type: Argument::CollectionText(None),
    },
    exclude_dir: Entry {
        defaults_to: DefaultValue::Skip,
        format: (Formatter::Default, Formatter::Default),
        target_name: Name::LongC("exclude-dir"),
        target_type: Argument::CollectionText(None),
    },
    default_args:  "-rn",
    command: "ugrep",
};

pub fn picker(tool: &str) -> Grepper {
    match tool {
        "grep" => self::GREP,
        "ripgrep" => self::RIPGREP,
        "ugrep" => self::UGREP,
        _ => panic!("Unkown tool requested"),
    }
}

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
    fn populate(&mut self, with: Args) -> BTreeMap<Name, Vec<Entry>> {
        let mut r: BTreeMap<Name, Vec<Entry>> = BTreeMap::new();
        self.regex_pattern.fill(&with.regex_pattern);
        self.file.fill(&with.file);
        self.casei.fill(&with.casei);
        self.include_files.fill(&with.include_files);
        self.exclude_files.fill(&with.exclude_files);
        self.include_dir.fill(&with.include_dir);
        self.exclude_dir.fill(&with.exclude_dir);

        r.expand_field(&self.regex_pattern);
        r.expand_field(&self.file);
        r.expand_field(&self.casei);
        r.expand_field(&self.include_files);
        r.expand_field(&self.exclude_files);
        r.expand_field(&self.include_dir);
        r.expand_field(&self.exclude_dir);

        return r;
    }
    fn generate(&self, with: BTreeMap<Name, Vec<Entry>>) -> Cmd {
        let mut r: Vec<String> = Vec::new();

        for i in with {
            r.transform(&i.1);
        }
        r.extend(self.default_args.split(" ").map(|c| c.to_string()).collect::<Vec<String>>());
        return (self.command.to_string(), r);
    }
}