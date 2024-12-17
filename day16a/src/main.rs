// I think this might be buggy? I don't know, but, even though it produced the right answer for my
// input data, I saw some weird behavior when I tried to adapt this solution exactly for part B,
// which led me to rewriting the way the nodes that are visited by the Dijkstra are calculated.
// So!

use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

const TURN_COST: usize = 1_000;
const MOVE_COST: usize = 1;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

fn turns_needed(from: Direction, to: Direction) -> usize {
    if from.opposite() == to {
        2
    } else if from == to {
        0
    } else {
        1
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    position: (usize, usize),
    direction: Direction,
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct Edge {
    cost: usize,
    to: Node,
}
impl Node {
    fn moved_to(&self, going: Direction) -> Edge {
        let Node {
            position: from,
            direction: facing,
        } = *self;

        let node = match going {
            Direction::North => Node {
                position: (from.0, from.1 + 1),
                direction: going,
            },
            Direction::South => Node {
                position: (from.0, from.1 - 1),
                direction: going,
            },
            Direction::East => Node {
                position: (from.0 + 1, from.1),
                direction: going,
            },
            Direction::West => Node {
                position: (from.0 - 1, from.1),
                direction: going,
            },
        };

        Edge {
            to: node,
            cost: TURN_COST * turns_needed(facing, going) + MOVE_COST,
        }
    }

    fn get_neighbors(&self, matrix: &[Vec<char>]) -> [Option<Edge>; 4] {
        let possible = [
            self.moved_to(Direction::North),
            self.moved_to(Direction::South),
            self.moved_to(Direction::East),
            self.moved_to(Direction::West),
        ];

        let mut out = [None; 4];

        for (i, e) in possible.into_iter().enumerate() {
            let (x, y) = e.to.position;

            if matrix.get(y).is_some_and(|row| row.get(x).is_some()) && matrix[y][x] == '.' {
                out[i] = Some(e);
            }
        }

        out
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position
            .cmp(&other.position)
            .then_with(|| self.direction.cmp(&other.direction))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_min_score(from: (usize, usize), to: (usize, usize), matrix: &[Vec<char>]) -> Option<usize> {
    let mut distances = HashMap::new();
    let mut confirmed = HashSet::new();
    let mut candidates = BTreeSet::new();

    let from_node = Node {
        position: from,
        direction: Direction::East,
    };

    candidates.insert((0, from_node));
    distances.insert(from_node, 0);

    while let Some(&(distance, node)) = candidates.iter().next() {
        confirmed.insert(node);

        candidates.remove(&(distance, node));

        for neighbor in node.get_neighbors(matrix) {
            if neighbor.is_none() {
                continue;
            }
            let neighbor = neighbor.unwrap();
            if confirmed.contains(&neighbor.to) {
                continue;
            }

            let new_distance = distance + neighbor.cost;

            if let Some(&old_distance) = distances.get(&neighbor.to) {
                if old_distance > new_distance {
                    candidates.remove(&(old_distance, neighbor.to));

                    candidates.insert((new_distance, neighbor.to));
                    distances.insert(neighbor.to, new_distance);
                }
            } else {
                candidates.insert((new_distance, neighbor.to));
                distances.insert(neighbor.to, new_distance);
            }
        }
    }

    let mut out = None;
    for &dist in [
        distances.get(&Node {
            position: to,
            direction: Direction::North,
        }),
        distances.get(&Node {
            position: to,
            direction: Direction::South,
        }),
        distances.get(&Node {
            position: to,
            direction: Direction::East,
        }),
        distances.get(&Node {
            position: to,
            direction: Direction::West,
        }),
    ]
    .into_iter()
    .flatten()
    {
        if let Some(old) = out {
            if old > dist {
                out = Some(dist);
            }
        } else {
            out = Some(dist);
        }
    }

    out
}
fn main() {
    let input = fs::read_to_string("inputs/16.txt").expect("Failed to read file");
    let mut matrix: Vec<Vec<char>> = vec![];

    let mut starting_pos: Option<(usize, usize)> = None;
    let mut goal_pos: Option<(usize, usize)> = None;

    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];

        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                starting_pos = Some((x, y));
                row.push('.');
                continue;
            }
            if ch == 'E' {
                goal_pos = Some((x, y));
                row.push('.');
                continue;
            }
            row.push(ch);
        }
        matrix.push(row);
    }

    let starting_pos = starting_pos.expect("Failed to find starting position.");
    let goal_pos = goal_pos.expect("Failed to find goal position.");

    let min_score = get_min_score(starting_pos, goal_pos, &matrix);

    println!(
        "A minimum score of {}",
        min_score.expect("No path could be found!")
    );
}
