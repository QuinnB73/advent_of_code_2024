use std::str::FromStr;

use crate::error::PuzzleError;
use crate::io::read_lines;
use regex::{Captures, Regex};

pub fn main(file_path: String) -> Result<(), PuzzleError> {
    let instructions = get_instructions(file_path)?;

    let mut final_answer = 0;
    for instruction_set in &instructions {
        let local_answer = execute_instruction_set(instruction_set)?;
        final_answer += local_answer;
    }

    println!("Part one: {}", final_answer);

    let mut final_answer_with_enablement_instructions = 0;
    let mut enabled = true;
    for instruction_set in &instructions {
        let result = process_with_enablement_instructions(instruction_set, enabled)?;
        let local_answer = result.0;
        enabled = result.1;

        final_answer_with_enablement_instructions += local_answer;
    }

    println!("Part two: {}", final_answer_with_enablement_instructions);

    Ok(())
}

fn execute_instruction_set(instruction: &str) -> Result<u32, PuzzleError> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let nums: Vec<(Result<u32, PuzzleError>, Result<u32, PuzzleError>)> = re
        .captures_iter(&instruction)
        .map(|caps| {
            let (_, [lnum_str, rnum_str]) = caps.extract();

            let lnum = lnum_str.puzzle_parse::<u32>();
            let rnum = rnum_str.puzzle_parse::<u32>();

            (lnum, rnum)
        })
        .into_iter()
        .collect();

    let mut answer = 0;
    for (lnum, rnum) in nums {
        let local_result = lnum? * rnum?;
        answer += local_result;
    }

    Ok(answer)
}

fn process_with_enablement_instructions(
    instruction: &str,
    mut enabled: bool,
) -> Result<(u32, bool), PuzzleError> {
    let re = Regex::new(r"(?:mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();
    let captures: Vec<Captures> = re.captures_iter(&instruction).collect();
    let mut final_answer = 0;

    for capture in captures {
        let lnum_str = capture.get(1);
        let rnum_str = capture.get(2);
        let do_str = capture.get(3);
        let dont_str = capture.get(4);

        if dont_str.is_some() {
            enabled = false;
        }

        if do_str.is_some() {
            enabled = true;
        }

        if enabled && lnum_str.is_some() && rnum_str.is_some() {
            let lnum = lnum_str
                .ok_or(PuzzleError {
                    msg: "Unable to capture left operand".to_string(),
                })?
                .as_str()
                .puzzle_parse::<u32>()?;
            let rnum = rnum_str
                .ok_or(PuzzleError {
                    msg: "Unable to capture right operand".to_string(),
                })?
                .as_str()
                .puzzle_parse::<u32>()?;

            let local_answer = lnum * rnum;
            final_answer += local_answer;
        }
    }

    Ok((final_answer, enabled))
}

fn get_instructions(file_path: String) -> Result<Vec<String>, PuzzleError> {
    if let Ok(lines) = read_lines(file_path) {
        return Ok(lines.flatten().collect());
    } else {
        return Err(PuzzleError {
            msg: "Failed to read input".to_string(),
        });
    }
}

trait PuzzleParse {
    fn puzzle_parse<T>(&self) -> Result<T, PuzzleError>
    where
        T: FromStr;
}

impl PuzzleParse for str {
    fn puzzle_parse<T>(&self) -> Result<T, PuzzleError>
    where
        T: FromStr,
    {
        self.parse::<T>().map_err(|_| PuzzleError {
            msg: format!("Unable to parse {}", self),
        })
    }
}
