use clap::Parser;

#[derive(Parser)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.path);
}
