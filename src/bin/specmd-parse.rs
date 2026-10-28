use clap::Parser;

use specmd::prelude::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    path: String,
}

fn main() -> CutResult<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(args.path).expect("should open file path passed");

    let parsed_cut_file = parse(&content)?;

    let json = serde_json::to_string_pretty(&parsed_cut_file).expect("should serialize to JSON");

    println!("{json}");

    Ok(())
}
