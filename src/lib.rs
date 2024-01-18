use std::{
    env,
    fs::File,
    io::{self, Read},
};

const BYTES_PER_LINE: usize = 16;

type HexdumperResult = io::Result<()>;

pub struct Args {
    pub fname: String,
}

pub fn parse_args() -> Option<Args> {
    let fname = match env::args().nth(1) {
        Some(f) => f,
        None => return None,
    };

    Some(Args { fname })
}

pub fn run(args: &Args) -> HexdumperResult {
    let mut file = File::open(&args.fname)?;

    let mut buffer = [0; BYTES_PER_LINE];

    let mut curr_pos = 0;

    while file.read_exact(&mut buffer).is_ok() {
        print!("0x{:08x} ", curr_pos);
        for byte in &buffer {
            print!("0x{:02x} ", byte)
        }
        println!();
        curr_pos += BYTES_PER_LINE;
    }
    Ok(())
}
