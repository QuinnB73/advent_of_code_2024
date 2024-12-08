use crate::error::PuzzleError;
use crate::io::read_lines;

pub fn main(file_path: String) -> Result<(), PuzzleError> {
    let reports = get_reports(file_path)?;

    let mut num_safe = 0;
    let mut num_safe_with_dampener = 0;
    for report in reports.into_iter() {
        if is_safe(&report) {
            num_safe += 1;
        }

        for i in 0..report.len() {
            let mut report_copy = report.to_vec();
            report_copy.remove(i);

            if is_safe(&report_copy) {
                num_safe_with_dampener += 1;
                break;
            }
        }
    }

    println!("Part one: {} safe reports", num_safe);
    println!("Part two: {} safe reports", num_safe_with_dampener);

    Ok(())
}

fn is_safe(report: &Vec<u32>) -> bool {
    let mut report_iter = report.into_iter();
    let mut prev_level: u32 = if let Some(level) = report_iter.next() {
        *level
    } else {
        return false;
    };

    let mut checked_initial_levels = false;
    let mut is_ascending = false;

    loop {
        let next_level = if let Some(level) = report_iter.next() {
            *level
        } else {
            break;
        };

        if !checked_initial_levels {
            is_ascending = prev_level < next_level;
            checked_initial_levels = true;
        }

        let diff = prev_level.abs_diff(next_level);
        let is_unsafe_level = (is_ascending && prev_level > next_level)
            || (!is_ascending && prev_level < next_level)
            || diff < 1
            || diff > 3;

        if is_unsafe_level {
            return false;
        }

        prev_level = next_level;
    }

    true
}

fn get_reports(file_path: String) -> Result<Vec<Vec<u32>>, PuzzleError> {
    let mut reports: Vec<Vec<u32>> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let parts = line.split(" ");
            let report_nums: Result<Vec<u32>, PuzzleError> = parts
                .into_iter()
                .map(|x| {
                    x.parse::<u32>().map_err(|e| PuzzleError {
                        msg: format!("Unable to parse input {}", e).to_string(),
                    })
                })
                .collect();

            reports.push(report_nums?);
        }
    } else {
        return Err(PuzzleError {
            msg: "Failed to fetch reports".to_string(),
        });
    }

    Ok(reports)
}
