use std::io::{self, Write};
mod eval;

fn main_loop() -> io::Result<()> {
    let mut input = String::new();
    let eval = &eval::Evaluator::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input)?;
        eval.execute(&input)?;
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
