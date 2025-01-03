/*
--- Day 6: Guard Gallivant ---
The Historians use their fancy device again, this time to whisk you all away to the North Pole prototype suit manufacturing lab... in the year 1518! It turns out that having direct access to history is very convenient for a group of historians.

You still have to be careful of time paradoxes, and so it will be important to avoid anyone from 1518 while The Historians search for the Chief. Unfortunately, a single guard is patrolling this part of the lab.

Maybe you can work out where the guard will go ahead of time so that The Historians can search safely?

You start by making a map (your puzzle input) of the situation. For example:

....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...

The map shows the current position of the guard with ^ (to indicate the guard is currently facing up from the perspective of the map). Any obstructions - crates, desks, alchemical reactors, etc. - are shown as #.

Lab guards in 1518 follow a very strict patrol protocol which involves repeatedly following these steps:

- If there is something directly in front of you, turn right 90 degrees.
- Otherwise, take a step forward.

Following the above protocol, the guard moves up several times until she reaches an obstacle (in this case, a pile of failed suit prototypes):

....#.....
....^....#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...

Because there is now an obstacle in front of the guard, she turns right before continuing straight in her new facing direction:

....#.....
........>#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...

Reaching another obstacle (a spool of several very long polymers), she turns right again and continues downward:

....#.....
.........#
..........
..#.......
.......#..
..........
.#......v.
........#.
#.........
......#...

This process continues for a while, but the guard eventually leaves the mapped area (after walking past a tank of universal solvent):

....#.....
.........#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#v..

By predicting the guard's route, you can determine which specific positions in the lab will be in the patrol path. Including the guard's starting position, the positions visited by the guard before leaving the area are marked with an X:

....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X..

In this example, the guard will visit 41 distinct positions on your map.

Predict the path of the guard. How many distinct positions will the guard visit before leaving the mapped area?

--- Part Two ---

While The Historians begin working around the guard's patrol route, you borrow their fancy device and step outside the lab. From the safety of a supply closet, you time travel through the last few months and record the nightly status of the lab's guard post on the walls of the closet.

Returning after what seems like only a few seconds to The Historians, they explain that the guard's patrol area is simply too large for them to safely search the lab without getting caught.

Fortunately, they are pretty sure that adding a single new obstruction won't cause a time paradox. They'd like to place the new obstruction in such a way that the guard will get stuck in a loop, making the rest of the lab safe to search.

To have the lowest chance of creating a time paradox, The Historians would like to know all of the possible positions for such an obstruction. The new obstruction can't be placed at the guard's starting position - the guard is there right now and would notice.

In the above example, there are only 6 different positions where a new obstruction would cause the guard to get stuck in a loop. The diagrams of these six situations use O to mark the new obstruction, | to show a position where the guard moves up/down, - to show a position where the guard moves left/right, and + to show a position where the guard moves both up/down and left/right.

Option one, put a printing press next to the guard's starting position:

....#.....
....+---+#
....|...|.
..#.|...|.
....|..#|.
....|...|.
.#.O^---+.
........#.
#.........
......#...

Option two, put a stack of failed suit prototypes in the bottom right quadrant of the mapped area:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
......O.#.
#.........
......#...

Option three, put a crate of chimney-squeeze prototype fabric next to the standing desk in the bottom right quadrant:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
.+----+O#.
#+----+...
......#...

Option four, put an alchemical retroencabulator near the bottom left corner:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
..|...|.#.
#O+---+...
......#...

Option five, put the alchemical retroencabulator a bit to the right instead:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
....|.|.#.
#..O+-+...
......#...

Option six, put a tank of sovereign glue right next to the tank of universal solvent:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
.+----++#.
#+----++..
......#O..

It doesn't really matter what you choose to use as an obstacle so long as you and The Historians can put it into position without the guard noticing. The important thing is having enough options that you can find one that minimizes time paradoxes, and in this example, there are 6 different positions you could choose.

You need to get the guard stuck in a loop by adding a single new obstruction. How many different positions could you choose for this obstruction?
*/

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Direction {
    x: i32,
    y: i32,
}

impl Direction {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn turn(&self) -> Self {
        match (self.x, self.y) {
            (0, -1) => Self { x: 1, y: 0 },
            (1, 0) => Self { x: 0, y: 1 },
            (0, 1) => Self { x: -1, y: 0 },
            (-1, 0) => Self { x: 0, y: -1 },
            _ => panic!("Unsupported direction!"),
        }
    }
}

struct Map {
    visited_positions: HashSet<Position>,
    obstacles: HashMap<Position, i32>,
    position: Position,
    x_max: i32,
    y_max: i32,
    direction: Direction,
    loop_detected: bool,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let visited_positions: HashSet<Position> = HashSet::new();
        let mut obstacles: HashMap<Position, i32> = HashMap::new();
        let mut position: Position = Position { x: -1, y: -1 };
        let y_max = non_empty_lines(input).collect::<Vec<&str>>().len() as i32 - 1;
        let x_max = non_empty_lines(input).collect::<Vec<&str>>()[0].len() as i32 - 1;
        let direction = Direction::new(0, -1);

        for (y, line) in non_empty_lines(input).enumerate() {
            for (x, position_item) in line.chars().enumerate() {
                match position_item {
                    '#' => {
                        obstacles.insert(
                            Position {
                                x: x as i32,
                                y: y as i32,
                            },
                            0,
                        );
                    }
                    '^' => {
                        position = Position {
                            x: x as i32,
                            y: y as i32,
                        };
                    }
                    _ => {}
                }
            }
        }

        if position == (Position { x: -1, y: -1 }) {
            panic!("No starting position found!");
        }

        Self {
            visited_positions,
            obstacles,
            position,
            x_max,
            y_max,
            direction,
            loop_detected: false,
        }
    }

    pub fn visited_positions(&mut self) -> &HashSet<Position> {
        while self.position.x >= 0
            && self.position.x <= self.x_max
            && self.position.y >= 0
            && self.position.y <= self.y_max
        {
            let obstacle_times_visited = self.obstacles.get_mut(&Position {
                x: self.position.x + self.direction.x,
                y: self.position.y + self.direction.y,
            });

            match obstacle_times_visited {
                Some(times_visited) => {
                    self.direction = self.direction.turn();

                    if *times_visited == 2 {
                        self.loop_detected = true;
                        break;
                    } else {
                        *times_visited += 1;
                    }
                }
                None => {
                    self.visited_positions.insert(self.position);

                    self.position = Position {
                        x: self.position.x + self.direction.x,
                        y: self.position.y + self.direction.y,
                    };
                }
            }
        }

        &self.visited_positions
    }

    pub fn add_obstacle(&mut self, x: i32, y: i32) -> bool {
        if self.position == (Position { x, y }) {
            return false;
        }

        if let std::collections::hash_map::Entry::Vacant(e) =
            self.obstacles.entry(Position { x, y })
        {
            e.insert(0);

            true
        } else {
            false
        }
    }
}

fn non_empty_lines(input: &str) -> impl Iterator<Item = &str> {
    input.split('\n').filter(|line| !line.is_empty())
}

pub fn part1(input: &str) -> Result<String, String> {
    let mut map = Map::new(input);

    Ok(map.visited_positions().len().to_string())
}

pub fn part2(input: &str) -> Result<String, String> {
    let map = Map::new(input);
    let mut number_of_positions_for_obstructions = 0;

    for y in 0..=map.y_max {
        for x in 0..=map.x_max {
            let mut temp_map = Map::new(input);

            if !temp_map.add_obstacle(x, y) {
                continue;
            };

            temp_map.visited_positions();

            if temp_map.loop_detected {
                number_of_positions_for_obstructions += 1;
            }
        }
    }

    Ok(number_of_positions_for_obstructions.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Ok(41.to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), Ok(6.to_string()));
    }
}
