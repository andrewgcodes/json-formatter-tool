use clap::Parser;
use serde_json::{Value, from_str, to_string_pretty};
use std::fs;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "devin")]
#[command(about = "A simple JSON formatter", long_about = None)]
struct Cli {
    /// JSON file to format (reads from stdin if not provided)
    file: Option<String>,
    
    /// Number of spaces for indentation
    #[arg(short, long, default_value = "2")]
    indent: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Read JSON content
    let json_content = if let Some(file_path) = cli.file {
        fs::read_to_string(file_path)?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };
    
    // Parse and pretty-print JSON
    let parsed: Value = from_str(&json_content)?;
    let pretty = to_string_pretty(&parsed)?;
    
    println!("{}", pretty);
    
    Ok(())
}
