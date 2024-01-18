use std::{io::ErrorKind, process};

use hexdumper::{parse_args, run};

const USAGE: &str = "hexdumper filename";

fn main() {
    let args = match parse_args() {
        Some(a) => a,
        None => {
            println!("{}", USAGE);
            process::exit(1);
        }
    };
    if let Err(e) = run(&args) {
        if e.kind() == ErrorKind::NotFound {
            eprintln!("file: {} doesn't exits!", &args.fname);
            process::exit(1);
        } else {
            eprintln!("An application error occured!");
            process::exit(1)
        }
    };
}
