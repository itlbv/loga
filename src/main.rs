use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Args {
    path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(&args.path)
        .with_context(|| format!("Failed to read {}", &args.path))?;
    let buffered = BufReader::new(file);

    let mut result_table = HashMap::<String, (usize, usize)>::new();

    for line in buffered.lines() {
        let line_str = line.as_ref().unwrap();

        let parsed = json::parse(line_str)
            .with_context(|| format!("Failed to parse line [{}]", line_str))?;
        let typ = parsed["type"].as_str().unwrap(); // TODO handle unwrap
        let size = std::mem::size_of_val(&line_str); // TODO wrong size here

        if result_table.contains_key(&typ.to_string()) {
            let (count, curr_size) = result_table[&typ.to_string()];
            result_table.insert(typ.to_string(), (count + 1, curr_size + size));
        } else {
            result_table.insert(typ.to_string(), (1, size));
        }
    }

    let mut longest_type = 10;
    for (typ, _) in &result_table {
        if typ.len() > longest_type {
            longest_type = typ.len();
        }
    }

    println!("| {:width$} | {:<10} | {:<10} |", "type name", "count", "size", width = longest_type);
    for (typ, (count, size)) in &result_table {
        println!("| {:width$} | {:<10} | {:<10} |", typ, count, size, width = longest_type);
    }

    Ok(())
}
