/*
--- Day 3: Mull It Over ---
"Our computers are having issues, so I have no idea if we have any Chief Historians in stock! You're welcome to check the warehouse, though," says the mildly flustered shopkeeper at the North Pole Toboggan Rental Shop. The Historians head out to take a look.

The shopkeeper turns to you. "Any chance you can see why our computers are having issues again?"

The computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted. All of the instructions have been jumbled up!

It seems like the goal of the program is just to multiply some numbers. It does that with instructions like mul(X,Y), where X and Y are each 1-3 digit numbers. For instance, mul(44,46) multiplies 44 by 46 to get a result of 2024. Similarly, mul(123,4) would multiply 123 by 4.

However, because the program's memory has been corrupted, there are also many invalid characters that should be ignored, even if they look like part of a mul instruction. Sequences like mul(4*, mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do nothing.

For example, consider the following section of corrupted memory:

xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
Only the four highlighted sections are real mul instructions. Adding up the result of each instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).

Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the results of the multiplications?

--- Part Two ---
As you scan through the corrupted memory, you notice that some of the conditional statements are also still intact. If you handle some of the uncorrupted conditional statements in the program, you might be able to get an even more accurate result.

There are two new instructions you'll need to handle:

The do() instruction enables future mul instructions.
The don't() instruction disables future mul instructions.
Only the most recent do() or don't() instruction applies. At the beginning of the program, mul instructions are enabled.

For example:

xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
This corrupted memory is similar to the example from before, but this time the mul(5,5) and mul(11,8) instructions are disabled because there is a don't() instruction before them. The other mul instructions function normally, including the one at the end that gets re-enabled by a do() instruction.

This time, the sum of the results is 48 (2*4 + 8*5).

Handle the new instructions; what do you get if you add up all of the results of just the enabled multiplications?
*/

#[derive(PartialEq)]
enum State {
    FindM,
    FindU,
    FindL,
    FindOpenedBracket,
    FindFirstDigit,
    FindLastDigit,
    FindO,
    FindNOrOpenedBracket,
    FindClosedBracket,
    FindSingleQuote,
    FindT,
}

fn reset_state(state: &mut State, first_number: &mut i64) {
    *state = State::FindM;
    *first_number = -1;
}

fn parse_number(number_string: &str) -> i64 {
    match number_string.parse::<i64>() {
        Ok(number) => number,
        Err(_) => panic!("Unable to parse number: {:?}", number_string),
    }
}

fn calculate_sum(input: &str, mul_instructions_deactivatable: bool) -> i64 {
    let mut total_sum = 0;
    let mut state = State::FindM;
    let mut index_of_first_digit = 0;
    let mut first_number = -1;
    let mut mul_instructions_enabled = true;

    for (index, character) in input.as_bytes().iter().enumerate() {
        match character {
            b'm' => {
                if state == State::FindM {
                    state = State::FindU
                }
            }
            b'u' => {
                if state == State::FindU {
                    state = State::FindL
                } else {
                    reset_state(&mut state, &mut first_number);
                }
            }
            b'l' => {
                if state == State::FindL {
                    state = State::FindOpenedBracket
                } else {
                    reset_state(&mut state, &mut first_number);
                }
            }
            b'(' => {
                if state == State::FindOpenedBracket {
                    state = State::FindFirstDigit
                } else if state == State::FindNOrOpenedBracket {
                    state = State::FindClosedBracket
                } else {
                    reset_state(&mut state, &mut first_number);
                }
            }
            b'0'..=b'9' => {
                if state == State::FindFirstDigit {
                    state = State::FindLastDigit;
                    index_of_first_digit = index;
                } else if state == State::FindLastDigit {
                } else {
                    reset_state(&mut state, &mut first_number);
                }
            }
            b',' => {
                if state == State::FindLastDigit {
                    state = State::FindFirstDigit;
                    first_number = parse_number(&input[index_of_first_digit..index]);
                } else {
                    reset_state(&mut state, &mut first_number);
                }
            }
            b')' => {
                if state == State::FindLastDigit && first_number > -1 {
                    state = State::FindM;
                    let second_number = parse_number(&input[index_of_first_digit..index]);

                    if mul_instructions_enabled {
                        total_sum += first_number * second_number;
                    }
                } else if state == State::FindClosedBracket {
                    mul_instructions_enabled = true;
                    state = State::FindM
                } else {
                    reset_state(&mut state, &mut first_number);
                }
            }
            b'd' => {
                reset_state(&mut state, &mut first_number);
                state = State::FindO;
            }
            b'o' => {
                if state == State::FindO {
                    state = State::FindNOrOpenedBracket;
                } else {
                    reset_state(&mut state, &mut first_number);
                }
            }
            b'n' => {
                if state == State::FindNOrOpenedBracket {
                    state = State::FindSingleQuote;
                } else {
                    reset_state(&mut state, &mut first_number);
                }
            }
            b'\'' => {
                if state == State::FindSingleQuote {
                    state = State::FindT;
                } else {
                    reset_state(&mut state, &mut first_number);
                }
            }
            b't' => {
                if state == State::FindT {
                    if mul_instructions_deactivatable {
                        mul_instructions_enabled = false;
                    }

                    state = State::FindM
                }
            }
            _ => state = State::FindM,
        }
    }

    total_sum
}

pub fn part1(input: &str) -> Result<String, String> {
    Ok(calculate_sum(input, false).to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    Ok(calculate_sum(input, true).to_string())
}

#[cfg(test)]
mod tests {
    use crate::day3::{part1, part2};

    static TEST_INPUT: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

    static TEST_INPUT_2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Ok(161.to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), Ok(48.to_string()));
    }
}
