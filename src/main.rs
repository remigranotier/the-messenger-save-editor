use std::path::PathBuf;

use clap::Parser;

/// CLI to encrypt and decrypt The Messenger save files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Output path
    #[arg(short, long, value_name = "PATH")]
    output: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let save_data = std::fs::read_to_string(args.input)?
        .replace("\r", "")
        .replace("\n", "");

    let new_data = save_data
        .chars()
        .map(|c: char| {
            let code = c as u8;
            let new_code = if code % 2 == 1 {
                (code ^ 128) - 1
            } else {
                (code ^ 128) + 1
            };
            new_code as char
        })
        .collect::<String>();

    let _result = std::fs::write(args.output, new_data);

    Ok(())
}
