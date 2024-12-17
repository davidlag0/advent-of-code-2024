/*
--- Day 4: Ceres Search ---
"Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!

As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.

This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:


..X...
.SAMX.
.A..A.
XMAS.S
.X....

The actual word search will be full of letters instead. For example:

MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX

In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:

....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX

Take a look at the little Elf's word search. How many times does XMAS appear?
*/

/*
00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10
11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21
22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54,
55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65,
66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76,
77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87,
88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98,
99, 00, 01, 02, 03, 04, 05, 06, 07, 08, 09,

total_length: 110
row_length = 10 + '\n' = 11

13 + ? = 25
? = (x: 1, y: -1)

13 + x + y * row_length = 13 + 1 + (-1*11) = 14 - 11 = 3

15 + ? = 25
? = (x: -1, y: 1)
15 + (-1) + (1 * 11) = 14 + 11 = 25

15 + ? = 5
? = (x: 1, y: -1)
15 + 1 + (-1)*11 = 16 - 11 = 5

64 + ? = 54
? = (x: 1, y: -1)
64 + 1 + (-1)*11 = 65 - 11 = 54
valid? 54 mod 11 = 10
invalid if : index mod row_length = row_length - 1
*/

struct XMASGrid<'a> {
    pub input_bytes: &'a [u8],
    row_length: usize,
    total_length: usize,
}

impl<'a> XMASGrid<'a> {
    pub fn new(input: &'a str) -> Self {
        let input_bytes = input.as_bytes();
        let row_length = match input.find('\n') {
            Some(length) => length,
            None => panic!("Unable to determine row length"),
        };
        let total_length = input.len();

        Self {
            input_bytes,
            row_length,
            total_length,
        }
    }

    fn number_of_xmas_at_index(&self, index: usize) -> i32 {
        let mut count_of_xmas = 0;

        if self.input_bytes[index] != b'S' {
            return 0;
        }

        for direction in DIRECTIONS {
            let mut current_index = index;
            let mut state = State::FindA;

            loop {
                let new_index = get_next_index_in_direction(
                    self.row_length,
                    self.total_length,
                    current_index,
                    &direction,
                );

                match new_index {
                    Some(index) => {
                        if update_state(&mut state, &self.input_bytes[index]) {
                            count_of_xmas += 1
                        }
                        current_index = index;
                    }
                    None => break,
                }

                if state == State::FindS {
                    break;
                }
            }
        }

        count_of_xmas
    }

    pub fn count_xmas(&self) -> i32 {
        let mut count_of_xmas = 0;

        for (index, character) in self.input_bytes.iter().enumerate() {
            match character {
                b'S' => {
                    count_of_xmas += self.number_of_xmas_at_index(index);
                }
                _ => continue,
            }
        }

        count_of_xmas
    }

    fn number_of_x_mas_at_index(&self, index: usize) -> i32 {
        let top_left = match get_next_index_in_direction(
            self.row_length,
            self.total_length,
            index,
            &Direction { x: -1, y: -1 },
        ) {
            Some(value) => value,
            None => return 0,
        };

        let bottom_left = match get_next_index_in_direction(
            self.row_length,
            self.total_length,
            index,
            &Direction { x: -1, y: 1 },
        ) {
            Some(value) => value,
            None => return 0,
        };

        let top_right = match get_next_index_in_direction(
            self.row_length,
            self.total_length,
            index,
            &Direction { x: 1, y: -1 },
        ) {
            Some(value) => value,
            None => return 0,
        };

        let bottom_right = match get_next_index_in_direction(
            self.row_length,
            self.total_length,
            index,
            &Direction { x: 1, y: 1 },
        ) {
            Some(value) => value,
            None => return 0,
        };

        if ((self.input_bytes[bottom_left] == b'M' && self.input_bytes[top_right] == b'S')
            || (self.input_bytes[bottom_left] == b'S' && self.input_bytes[top_right] == b'M'))
            && ((self.input_bytes[top_left] == b'M' && self.input_bytes[bottom_right] == b'S')
                || (self.input_bytes[top_left] == b'S' && self.input_bytes[bottom_right] == b'M'))
        {
            1
        } else {
            0
        }
    }

    pub fn count_x_mas(&self) -> i32 {
        let mut count_of_x_mas = 0;

        for (index, character) in self.input_bytes.iter().enumerate() {
            match character {
                b'A' => {
                    count_of_x_mas += self.number_of_x_mas_at_index(index);
                }
                _ => continue,
            }
        }

        count_of_x_mas
    }
}

#[derive(Debug)]
struct Direction {
    pub x: i32,
    pub y: i32,
}

const DIRECTIONS: [Direction; 8] = [
    Direction { x: -1, y: 1 },
    Direction { x: 0, y: 1 },
    Direction { x: 1, y: 1 },
    Direction { x: 1, y: 0 },
    Direction { x: 1, y: -1 },
    Direction { x: 0, y: -1 },
    Direction { x: -1, y: -1 },
    Direction { x: -1, y: 0 },
];

#[derive(Debug, PartialEq)]
enum State {
    FindS,
    FindA,
    FindM,
    FindX,
}

fn is_index_valid(index: usize, row_length: usize, total_length: usize) -> bool {
    index < total_length && (index % (row_length + 1) != row_length)
}

fn get_next_index_in_direction(
    row_length: usize,
    total_length: usize,
    current_index: usize,
    direction: &Direction,
) -> Option<usize> {
    let new_index =
        (current_index as i32 + direction.x + direction.y * (row_length as i32 + 1)) as usize;

    if !is_index_valid(new_index, row_length, total_length) {
        return None;
    }

    Some(new_index)
}

fn update_state(current_state: &mut State, character: &u8) -> bool {
    match character {
        b'A' => {
            if *current_state == State::FindA {
                *current_state = State::FindM
            } else {
                *current_state = State::FindS
            }
        }
        b'M' => {
            if *current_state == State::FindM {
                *current_state = State::FindX
            } else {
                *current_state = State::FindS
            }
        }
        b'X' => {
            if *current_state == State::FindX {
                *current_state = State::FindS;
                return true;
            } else {
                *current_state = State::FindS
            }
        }
        _ => *current_state = State::FindS,
    }

    false
}

pub fn part1(input: &str) -> Result<String, String> {
    let xmas_grid = XMASGrid::new(input);
    Ok(xmas_grid.count_xmas().to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let xmas_grid = XMASGrid::new(input);
    Ok(xmas_grid.count_x_mas().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_get_next_index_in_direction() {
        assert_eq!(
            get_next_index_in_direction(10, 110, 0, &Direction { x: -1, y: 0 }),
            None
        );
        assert_eq!(
            get_next_index_in_direction(10, 110, 1, &Direction { x: -1, y: 0 }),
            Some(0)
        );
        assert_eq!(
            get_next_index_in_direction(10, 110, 0, &Direction { x: 0, y: -1 }),
            None
        );
        assert_eq!(
            get_next_index_in_direction(10, 110, 1, &Direction { x: 0, y: 1 }),
            Some(12)
        );
        assert_eq!(
            get_next_index_in_direction(10, 110, 13, &Direction { x: 1, y: 1 }),
            Some(25)
        );
        assert_eq!(
            get_next_index_in_direction(10, 110, 64, &Direction { x: 1, y: -1 }),
            None
        );
        // This is starting from a wrong position ('\n') on purpose.
        assert_eq!(
            get_next_index_in_direction(10, 110, 21, &Direction { x: 1, y: 1 }),
            Some(33)
        );
    }

    #[test]
    fn test_number_of_xmas_at_index() {
        let xmas_grid = XMASGrid::new(TEST_INPUT);

        assert_eq!(xmas_grid.number_of_xmas_at_index(3), 0);
        assert_eq!(xmas_grid.number_of_xmas_at_index(8), 1);
        assert_eq!(xmas_grid.number_of_xmas_at_index(12), 1);
        assert_eq!(xmas_grid.number_of_xmas_at_index(17), 1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Ok(18.to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), Ok(9.to_string()));
    }
}
