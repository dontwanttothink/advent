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
    fn turned_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    fn turned_counter_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
    fn move_from(&self, position: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (position.0, position.1 - 1),
            Direction::South => (position.0, position.1 + 1),
            Direction::East => (position.0 + 1, position.1),
            Direction::West => (position.0 - 1, position.1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
    fn get_neighbors(&self, matrix: &[Vec<char>]) -> [Option<Edge>; 4] {
        let possible = [
            Edge {
                cost: TURN_COST,
                to: Node {
                    position: self.position,
                    direction: self.direction.turned_clockwise(),
                },
            },
            Edge {
                cost: TURN_COST,
                to: Node {
                    position: self.position,
                    direction: self.direction.turned_counter_clockwise(),
                },
            },
            Edge {
                cost: MOVE_COST,
                to: Node {
                    position: self.direction.move_from(self.position),
                    direction: self.direction,
                },
            },
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

fn get_distances(from: Node, matrix: &[Vec<char>]) -> HashMap<Node, usize> {
    let mut distances = HashMap::new();
    let mut confirmed = HashSet::new();
    let mut candidates = BTreeSet::new();

    candidates.insert((0, from));
    distances.insert(from, 0);

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

    distances
}

fn get_positions_count(start: (usize, usize), goal: (usize, usize), matrix: &[Vec<char>]) -> usize {
    /*
    First, get the optimal path length */
    let distances_from_start = get_distances(
        Node {
            position: start,
            direction: Direction::East,
        },
        matrix,
    );

    // We keep track of the ending states
    let mut ending_nodes = vec![];

    let mut out = None;
    for direction in [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ] {
        let ending_node = Node {
            position: goal,
            direction,
        };

        let dist = distances_from_start.get(&ending_node);

        if dist.is_none() {
            continue;
        }
        let &dist = dist.unwrap();

        if let Some(old) = out {
            #[allow(clippy::comparison_chain)]
            if dist < old {
                out = Some(dist);

                ending_nodes.clear();
                ending_nodes.push(ending_node);
            } else if dist == old {
                ending_nodes.push(ending_node);
            }
        } else {
            out = Some(dist);
            ending_nodes.push(ending_node);
        }
    }

    let min_length = out.expect("No path could be found!");

    /*
    We perform the search from the perspective of the path's end! */
    let distances_from_goals: Vec<HashMap<Node, usize>> = ending_nodes
        .into_iter()
        .map(|n| {
            get_distances(
                Node {
                    direction: n.direction.opposite(), // We're going backwards!
                    ..n
                },
                matrix,
            )
        })
        .collect();

    /*
    Now, for each node, check whether it's a part of an optimal path. */

    let mut count = 0;

    for (y, row) in matrix.iter().enumerate() {
        'position: for (x, &ch) in row.iter().enumerate() {
            if ch != '.' {
                continue;
            }

            for direction in [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                let node_forward = Node {
                    position: (x, y),
                    direction,
                };
                let node_backward = Node {
                    position: (x, y),
                    direction: direction.opposite(),
                };

                for distances_from_goal in &distances_from_goals {
                    let distance = distances_from_start.get(&node_forward).unwrap()
                        + distances_from_goal.get(&node_backward).unwrap();
                    if distance == min_length {
                        count += 1;
                        continue 'position;
                    }
                }
            }
        }
    }

    count
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

    let positions_count = get_positions_count(starting_pos, goal_pos, &matrix);

    println!("{} best viewing positions", positions_count);
}
