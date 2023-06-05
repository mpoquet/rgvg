use std::process::{exit};

use clap::Parser;

///Creates a command from a command string.
/*fn build(command: Vec<&str>) -> Command {
    let mut output = Command::new(command.get(0).expect("No command attached!"));

    for i in 1..command.len() {
        output.arg(command[i]);
    }
    return output;
}*/

//NOTE: Search for command in PATH, try to find rust crate
// Format stdout facile a lire avec grep


use rgvg::input::framework::Convertible;

/*fn rc<'a>(cap: &Option<regex::Match<'a>>) -> &'a str {
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
}*/

fn search(args: rgvg::input::Args) {
    let tool = &args.tool;
    let color = rgvg::common::color(&args.color);
    //println!("{:?}", args);
    let mut g = rgvg::input::tools::picker(&tool);
    let q = g.populate(&args);
    //println!("{:?}", q);
    let p = g.generate(q);
    if args.dry {
        println!("{} {}", p.0, p.1.iter().map(|s| String::from("\"".to_owned() + s + "\"")).collect::<Vec<String>>().join(" "));
        return;
    }
    let r = rgvg::common::command::call(p).unwrap();
    if !r.status.success() { 
        if r.stderr.len() == 0 {
            println!("No matches.");
            return;
            //eprintln!("Command failed without error string. It is likely that it could not parse your regular expression.");
        } else {
            eprint!("{}", &String::from_utf8(r.stderr).expect("Invalid utf-8 in error string"));
            exit(1)
        }
    }
    //let stop = std::time::Instant::now();

    //println!("t1: {:?}", stop - start);
    let s = &String::from_utf8(r.stdout).expect("Invalid utf-8 in output string");
    //let stop = std::time::Instant::now();
    //println!("t2: {:?}", stop - start);
    let result = match &args.order_results {
        false => rgvg::output::read_display(rgvg::output::picker(&tool), s, color, args.remove_leading),
        true => {
            let mut r = rgvg::output::read(rgvg::output::picker(&tool), s, color, args.remove_leading);
            r.sort();
            rgvg::output::display(&r, color);
            r
        }
    };
    //let stop = std::time::Instant::now();
    //println!("t3: {:?}", stop - start);
    rgvg::output::write(&result);
    //let stop = std::time::Instant::now();
    //println!("t4: {:?}", stop - start);
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
    rgvg::output::read(rgvg::output::GREP, text);
    let text = r"[0m[35m./documents/t.txt[0m:[0m[32m1[0m:haha [0m[1m[31mthe[0m ha
[0m[35m./documents/t.txt[0m:[0m[32m2[0m:elh[0m[1m[31mthe[0m:)
";
    rgvg::output::read(rgvg::output::RIPGREP, text);*/

    //let start = std::time::Instant::now();
    let args = rgvg::input::Args::parse();
    match args.list_tools {
        false => match &args.regex_pattern {
            None => rgvg::common::last(rgvg::common::color(&args.color)),
            Some(_) => search(args)
        },
        true => {
            let tools = ["grep", "ripgrep", "ugrep"];
            for t in tools {
                println!("{}", t);
            }
        },
    }

    
}
