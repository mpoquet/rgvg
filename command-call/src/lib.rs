use ccal_derive::CommandCall;

trait CommandCall {
    fn command_call() {
        println!("Hey!");
    }
}

#[derive(CommandCall)]
struct Grep {
    #[arg(short)]
    flag: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_syntax() {
       Grep::command_call();
    }
}
