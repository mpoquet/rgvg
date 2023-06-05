use std::process::exit;
use clap::Parser;
use rgvg::input::framework::Convertible;

fn search(args: rgvg::input::Args) {
    let tool = &args.tool;
    let color = rgvg::common::color(&args.color);
    //println!("{:?}", args);
    let mut g = rgvg::input::tools::picker(&tool);
    let q = g.populate(&args);
    //println!("{:?}", q);
    let p = g.generate(q);
    if args.dry {
        println!(
            "{} {}",
            p.0,
            p.1.iter()
                .map(|s| String::from("\"".to_owned() + s + "\""))
                .collect::<Vec<String>>()
                .join(" ")
        );
        return;
    }
    let r = rgvg::common::command::call(p).unwrap();
    if !r.status.success() {
        if r.stderr.len() == 0 {
            println!("No matches.");
            return;
            //eprintln!("Command failed without error string. It is likely that it could not parse your regular expression.");
        } else {
            eprint!(
                "{}",
                &String::from_utf8(r.stderr).expect("Invalid utf-8 in error string")
            );
            exit(1)
        }
    }

    let s = &String::from_utf8(r.stdout).expect("Invalid utf-8 in output string");
    let result = match &args.order_results {
        false => {
            rgvg::output::read_display(rgvg::output::picker(&tool), s, color, args.remove_leading)
        }
        true => {
            let mut r =
                rgvg::output::read(rgvg::output::picker(&tool), s, color, args.remove_leading);
            r.sort();
            rgvg::output::display(&r, color);
            r
        }
    };
    rgvg::output::write(&result);
}

fn main() {
    let args = rgvg::input::Args::parse();
    match args.list_tools {
        false => match &args.regex_pattern {
            None => rgvg::common::last(rgvg::common::color(&args.color)),
            Some(_) => search(args),
        },
        true => {
            let tools = ["grep", "ripgrep", "ugrep"];
            for t in tools {
                println!("{}", t);
            }
        }
    }
}
