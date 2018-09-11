use std::collections::HashMap;
use std::io::{self, Error, ErrorKind, Write};
use std::process::Command;

fn split_line(line: &str) -> impl Iterator<Item = &str> {
    line.trim().split_whitespace()
}

fn builtin_exit(args: &[String]) -> io::Result<()> {
    if args.len() > 1 {
        return Err(Error::new(ErrorKind::InvalidInput, "too many arguments"));
    }
    let code = args.get(0).map_or(0, |n| n.parse::<i32>().unwrap_or(0));
    std::process::exit(code)
}

fn create_builtins() -> HashMap<String, fn(&[String]) -> io::Result<()>> {
    let mut r: HashMap<String, fn(&[String]) -> io::Result<()>> = HashMap::new();
    r.insert("exit".to_string(), builtin_exit);
    r
}

fn launch(cmd: &str, args: &[String]) -> io::Result<()> {
    match Command::new(cmd).args(args).status() {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("{}", err);
            Ok(())
        }
    }
}

fn execute(
    args: &[String],
    builtins: &HashMap<String, fn(&[String]) -> io::Result<()>>,
) -> io::Result<()> {
    if args.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, "command is not given"));
    }
    let cmd = &args[0];
    match builtins.get(cmd) {
        Some(func) => func(&args[1..]),
        None => launch(&cmd, &args[1..]),
    }
}

fn conv<'a, 'b, I>(x: I) -> Vec<String>
where
    I: Iterator<Item = &'a str>,
{
    let mut r: Vec<String> = Vec::new();
    for e in x {
        let s = e.to_string().clone();
        r.push(s);
    }
    r
}

fn main_loop() -> io::Result<()> {
    let mut input = String::new();
    let builtins = create_builtins();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input)?;
        execute(&conv(split_line(&input)), &builtins)?;
        input.truncate(0);
        io::stdout().flush().unwrap();
    }
}

fn main() -> io::Result<()> {
    // Load config files, if any.

    // Run command loop.
    let r = main_loop();

    // Perform any shutdown/cleanup.

    r
}
