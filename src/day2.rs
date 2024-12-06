/*
--- Day 2: Red-Nosed Reports ---
Fortunately, the first location The Historians want to search isn't a long walk from the Chief Historian's office.

While the Red-Nosed Reindeer nuclear fusion/fission plant appears to contain no sign of the Chief Historian, the engineers there run up to you as soon as they see you. Apparently, they still talk about the time Rudolph was saved through molecular synthesis from a single electron.

They're quick to add that - since you're already here - they'd really appreciate your help analyzing some unusual data from the Red-Nosed reactor. You turn to check if The Historians are waiting for you, but they seem to have already divided into groups that are currently searching every corner of the facility. You offer to help with the unusual data.

The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers called levels that are separated by spaces. For example:

7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
This example data contains six reports each containing five levels.

The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:

The levels are either all increasing or all decreasing.
Any two adjacent levels differ by at least one and at most three.
In the example above, the reports can be found safe or unsafe by checking those rules:

7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
So, in this example, 2 reports are safe.

Analyze the unusual data from the engineers. How many reports are safe?

--- Part Two ---
The engineers are surprised by the low number of safe reports until they realize they forgot to tell you about the Problem Dampener.

The Problem Dampener is a reactor-mounted module that lets the reactor safety systems tolerate a single bad level in what would otherwise be a safe report. It's like the bad level never happened!

Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.

More of the above example's reports are now safe:

7 6 4 2 1: Safe without removing any level.
1 2 7 8 9: Unsafe regardless of which level is removed.
9 7 6 2 1: Unsafe regardless of which level is removed.
1 3 2 4 5: Safe by removing the second level, 3.
8 6 4 4 1: Safe by removing the third level, 4.
1 3 6 7 9: Safe without removing any level.
Thanks to the Problem Dampener, 4 reports are actually safe!

Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports. How many reports are now safe?
*/

const MAX_LEVEL_CHANGE: i64 = 3;

fn non_empty_lines(input: &str) -> impl Iterator<Item = &str> {
    input.split('\n').filter(|line| !line.is_empty())
}

#[derive(PartialEq)]
enum LevelState {
    Unset,
    Increasing,
    Decreasing,
}

fn parse_levels(level_string: &str) -> Vec<i64> {
    level_string
        .split(' ')
        .map(|level| match level.parse::<i64>() {
            Ok(value) => value,
            Err(_) => panic!("Unable to parse level: {:?}", level),
        })
        .collect()
}

fn is_report_safe(report: &[i64]) -> bool {
    let mut previous_level = 0;
    let mut level_state = LevelState::Unset;
    let report_length = report.len();

    for (index, current_level) in report.iter().enumerate() {
        if index > 0 {
            let level_change = current_level - previous_level;

            if level_change.abs() > MAX_LEVEL_CHANGE {
                return false;
            }

            match level_change {
                x if x > 0 => {
                    if level_state == LevelState::Unset {
                        level_state = LevelState::Decreasing;
                    } else if level_state == LevelState::Increasing {
                        return false;
                    }
                }
                x if x < 0 => {
                    if level_state == LevelState::Unset {
                        level_state = LevelState::Increasing;
                    } else if level_state == LevelState::Decreasing {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        previous_level = *current_level;

        if index == report_length - 1 {
            return true;
        }
    }

    false
}

pub fn part1(input: &str) -> Result<String, String> {
    let mut count_of_safe_reports = 0;

    for line in non_empty_lines(input) {
        let report: Vec<i64> = parse_levels(line);

        if is_report_safe(&report) {
            count_of_safe_reports += 1;
        }
    }

    Ok(count_of_safe_reports.to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let mut count_of_safe_reports = 0;

    for line in non_empty_lines(input) {
        let report: Vec<i64> = parse_levels(line);

        match is_report_safe(&report) {
            false => {
                for index in 0..report.len() {
                    let mut report_with_removed_level = report.clone();
                    report_with_removed_level.remove(index);

                    if is_report_safe(&report_with_removed_level) {
                        count_of_safe_reports += 1;
                        break;
                    }
                }
            }
            true => count_of_safe_reports += 1,
        }
    }

    Ok(count_of_safe_reports.to_string())
}

#[cfg(test)]
mod tests {
    use crate::day2::{part1, part2};

    static TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Ok(2.to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), Ok(4.to_string()));
    }
}
