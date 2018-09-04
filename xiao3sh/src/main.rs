use std::io::{self, Error, ErrorKind, Write};

fn split_line(line: &str) -> impl Iterator<Item = &str> {
    line.trim().split_whitespace()
}

fn find_builtin<'a, I>(cmd: &str) -> Option<fn((I)) -> io::Result<()>>
where
    I: Iterator<Item = &'a str>,
{
    // TODO implement
    None
}

fn launch<'a, I>(cmd: &str, args: I) -> io::Result<()>
where
    I: Iterator<Item = &'a str>,
{
    // TODO implement
    Ok(())
}

fn execute<'a, I: Iterator<Item = &'a str>>(mut args: I) -> io::Result<()> {
    match args.next() {
        Some(cmd) => match find_builtin::<I>(&cmd) {
            Some(func) => func(args),
            None => launch(cmd, args),
        },
        None => Err(Error::new(ErrorKind::InvalidInput, "command is not given")),
    }
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
