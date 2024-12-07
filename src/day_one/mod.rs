use crate::error::PuzzleError;
use crate::io::read_lines;
use std::collections::HashMap;

pub fn main(file_path: String) -> Result<(), PuzzleError> {
    let (mut list_one, mut list_two) = get_input(file_path)?;

    list_one.sort();
    list_two.sort();

    part_one(&list_one, &list_two);
    part_two(&list_one, &list_two);

    Ok(())
}

fn part_one(list_one: &[u32], list_two: &[u32]) {
    let mut total_distance = 0;
    for (num_one, num_two) in list_one.iter().zip(list_two.iter()) {
        total_distance += num_one.abs_diff(*num_two);
    }

    println!("Part one: {}", total_distance);
}

fn part_two(list_one: &[u32], list_two: &[u32]) {
    let mut occurrences: HashMap<u32, u32> = HashMap::new();
    for num in list_two.iter() {
        occurrences
            .entry(*num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut similarity_score = 0;
    for num in list_one.iter() {
        let frequency = occurrences.get(num).unwrap_or(&0);

        similarity_score += num * frequency;
    }

    println!("Part two: {}", similarity_score);
}

fn get_input(file_path: String) -> Result<(Vec<u32>, Vec<u32>), PuzzleError> {
    let mut list_one: Vec<u32> = Vec::new();
    let mut list_two: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let parts: Vec<&str> = line.split("   ").collect();
            if parts.len() != 2 {
                return Err(PuzzleError {
                    msg: format!("Unexpected number of parts in {}", line).to_string(),
                });
            }

            let num_one = parse(parts[0].to_string())?;
            let num_two = parse(parts[1].to_string())?;

            list_one.push(num_one);
            list_two.push(num_two);
        }
    } else {
        return Err(PuzzleError {
            msg: "Unable to read file".to_string(),
        });
    }

    Ok((list_one, list_two))
}

fn parse(num_str: String) -> Result<u32, PuzzleError> {
    num_str.parse::<u32>().map_err(|_| PuzzleError {
        msg: "Unable to parse {num_str}".to_string(),
    })
}
