mod day_one;
mod error;
mod io;

use crate::day_one::main as main_one;
use crate::error::PuzzleError;
use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Puzzle number
    #[arg(short, long)]
    puzzle_num: u8,

    // Path to input file
    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();

    let functions = HashMap::from([(1, main_one)]);

    let result = match functions.get(&args.puzzle_num) {
        Some(function) => function(args.file_path),
        None => Err(PuzzleError {
            msg: "Unregistered puzzle num".to_string(),
        }),
    };

    match result {
        Ok(()) => (),
        Err(error) => {
            println!("{}", error);
            ()
        }
    }
}
