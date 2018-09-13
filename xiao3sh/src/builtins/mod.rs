use std;
use std::collections::HashMap;
use std::env;
use std::io::{self, Error, ErrorKind};
use std::path::Path;

fn builtin_exit(args: &[String]) -> io::Result<()> {
    if args.len() > 1 {
        return Err(Error::new(ErrorKind::InvalidInput, "too many arguments"));
    }
    let code = args.get(0).map_or(0, |n| n.parse::<i32>().unwrap_or(0));
    std::process::exit(code)
}

fn builtin_cd(args: &[String]) -> io::Result<()> {
    if args.len() > 1 {
        return Err(Error::new(ErrorKind::InvalidInput, "too many arguments"));
    }
    if args.len() < 1 {
        // TODO implement to move home directory after supporting environment variables
        return Ok(());
    }
    let path = Path::new(args.get(0).unwrap());
    match env::set_current_dir(path) {
        Ok(()) => Ok(()),
        Err(err) => {
            // FIXME output more kind message
            eprintln!("failed to change directory: {} ({:?})", path.display(), err);
            Ok(())
        }
    }
}

pub fn create_builtins() -> HashMap<String, fn(&[String]) -> io::Result<()>> {
    let mut r: HashMap<String, fn(&[String]) -> io::Result<()>> = HashMap::new();
    r.insert("exit".to_string(), builtin_exit);
    r.insert("cd".to_string(), builtin_cd);
    r
}
