use std::collections::HashMap;
use std::io::{self, Error, ErrorKind};
use std::process::Command;

mod builtins;

fn split_line(line: &str) -> impl Iterator<Item = &str> {
    line.trim().split_whitespace()
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

pub struct Evaluator {
    builtins: HashMap<String, fn(&[String]) -> io::Result<()>>,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            builtins: builtins::create_builtins(),
        }
    }

    pub fn execute(&self, input: &str) -> io::Result<()> {
        let args = conv(split_line(input));
        if args.len() < 1 {
            return Err(Error::new(ErrorKind::InvalidInput, "command is not given"));
        }
        let cmd = &args[0];
        match self.builtins.get(cmd) {
            Some(func) => func(&args[1..]),
            None => launch(&cmd, &args[1..]),
        }
    }
}
