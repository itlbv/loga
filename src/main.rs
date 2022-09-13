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
    let mut failed_lines = vec![];

    for line in buffered.lines() {
        let line_str = line.as_ref()
            .expect("Unexpected error processing the input file.");

        if line_str.is_empty() {
            continue;
        }

        let parsed = json::parse(line_str)
            .with_context(|| format!("Failed to parse line [{}]", line_str))?;

        match parsed["type"].as_str() {
            None => failed_lines.push(String::from(line_str)),
            Some(typ) => {
                let size = line_str.capacity(); // TODO wrong size here

                if result_table.contains_key(&typ.to_string()) {
                    let (count, curr_size) = result_table[&typ.to_string()];
                    result_table.insert(typ.to_string(), (count + 1, curr_size + size));
                } else {
                    result_table.insert(typ.to_string(), (1, size));
                }
            }
        }
    }

    let mut longest_type = 10;
    for (typ, _) in &result_table {
        let char_length = typ.chars().count();
        if char_length > longest_type {
            longest_type = char_length;
        }
    }

    print_table_row(longest_type, &"type name".to_string(), &"count".to_string(), &"size".to_string());
    for (typ, (count, size)) in &result_table {
        print_table_row(longest_type, typ, &count.to_string(), &size.to_string());
    }

    if failed_lines.is_empty() {
        println!("All lines successfully processed.")
    } else {
        println!("Field [type] was not found in {} lines.", failed_lines.len());
        println!("Failed lines:");
        for failed_line in failed_lines {
            println!("{}", failed_line);
        }
    }

    Ok(())
}

fn print_table_row(longest_type: usize, typ: &String, count: &String, size: &String) {
    println!("| {:width$} | {:<10} | {:<10} |", typ, count, size, width = longest_type);
}
