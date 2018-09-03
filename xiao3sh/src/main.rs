use std::io::{self, Write};

fn split_line(line: &str) -> impl Iterator<Item=&str> {
    // TODO fix
    line.trim().split(char::is_whitespace)
}
fn execute<'a, I: Iterator<Item = &'a str>>(args: I) -> io::Result<()> {
    // TODO implement
    for arg in args {
        println!("{}", arg);
    }
    Ok(())
}

fn main_loop() -> io::Result<()> {
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input)?;
        execute(split_line(&input))?;
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
