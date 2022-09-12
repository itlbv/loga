use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use clap::Parser;

#[derive(Parser)]
struct Args {
    path: String,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let file = File::open(args.path)?;
    let buffered = BufReader::new(file);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
