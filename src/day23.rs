use std::{
    collections::{HashMap, VecDeque},
    sync::atomic::{AtomicU32, Ordering},
    thread,
};

use crate::{
    libs::{Coordinate, Grid, DOWN, LEFT, OFFSETS, RIGHT, UP},
    problem::Solver,
};
pub struct Day {}

impl Solver for Day {
    fn pt1(&self, input: &str) -> String {
        let input = Input::from(input);
        let mut cost = [0; 36];

        let mut todo = VecDeque::new();
        todo.push_back(input.start);

        while let Some(from) = todo.pop_front() {
            let mut nodes = input.directed[from];

            while nodes > 0 {
                let to = nodes.trailing_zeros() as usize;
                let mask = 1 << to;
                nodes ^= mask;

                cost[to] = cost[to].max(cost[from] + input.weight[from][to]);
                todo.push_back(to);
            }
        }

        let distance = cost[input.end] + input.extra;
        format!("{distance}")
    }
    fn pt2(&self, input: &str) -> String {
        let input = Input::from(input);
        let shared = AtomicU32::new(0);
        let threads = thread::available_parallelism().unwrap().get();

        // Seed each worker thread with a starting state
        let mut seeds = VecDeque::new();
        seeds.push_back((input.start, 1 << input.start, 0));

        while seeds.len() < threads {
            let Some((from, seen, cost)) = seeds.pop_front() else {
                break;
            };

            if from == input.end {
                shared.fetch_max(cost, Ordering::Relaxed);
                continue;
            }

            let mut nodes = input.undirected[from] & !seen;

            while nodes > 0 {
                let to = nodes.trailing_zeros() as usize;
                let mask = 1 << to;
                nodes ^= mask;

                seeds.push_back((to, seen | mask, cost + input.weight[from][to]));
            }
        }

        // Use as many cores as possible to parallelize the remaining search.
        thread::scope(|scope| {
            for start in &seeds {
                scope.spawn(|| worker(&input, &shared, start));
            }
        });

        let distance = shared.load(Ordering::Relaxed) + input.extra;

        format!("{distance}")
    }
}

fn worker(input: &Input, shared: &AtomicU32, start: &(usize, u64, u32)) {
    let (from, seen, cost) = *start;
    let result = dfs(input, from, seen);
    shared.fetch_max(result + cost, Ordering::Relaxed);
}

fn dfs(input: &Input, from: usize, seen: u64) -> u32 {
    if from == input.end {
        return 0;
    }

    let mut nodes = input.undirected[from] & !seen;
    let mut result = 0;

    while nodes > 0 {
        let to = nodes.trailing_zeros() as usize;
        let mask = 1 << to;
        nodes ^= mask;

        result = result.max(input.weight[from][to] + dfs(input, to, seen | mask));
    }

    result
}

pub(crate) fn input() -> &'static str {
    include_str!("day23-input.txt").trim()
}

pub struct Input {
    start: usize,
    end: usize,
    extra: u32,
    directed: [u64; 36],
    undirected: [u64; 36],
    weight: [[u32; 36]; 36],
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let mut grid = Grid::from(value);
        let width = grid.width;
        let height = grid.height;

        // Modify edge of grid to remove the need for boundary checks.
        let start = grid.points[..grid.width]
            .iter()
            .enumerate()
            .find(|(i, _p)| grid.points[*i] != b'#')
            .map(|(i, _)| grid.coord(i))
            .unwrap();
        let end = grid
            .points
            .iter()
            .enumerate()
            .skip(grid.points.len() - grid.width)
            .find(|(i, _p)| grid.points[*i] != b'#')
            .map(|(i, _)| grid.coord(i))
            .unwrap();

        // Modify edge of grid to remove the need for boundary checks.
        grid[&start] = b'#';
        grid[&end] = b'#';

        // Move start and end away from edge.
        let start = Coordinate(start.0, start.1 + 1);
        let end = Coordinate(end.0, end.1 - 1);

        // Points of interest are start, end and junctions.
        grid[&start] = b'P';
        grid[&end] = b'P';

        let mut poi: HashMap<Coordinate<usize>, usize> = HashMap::new();

        poi.insert(start, 0);
        poi.insert(end, 1);

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let position = Coordinate(x, y);

                if grid[&position] != b'#' {
                    let neighbors = OFFSETS
                        .iter()
                        .map(|&o| position + o)
                        .filter(|n| grid[n] != b'#')
                        .count();
                    if neighbors > 2 {
                        grid[&position] = b'P';
                        poi.insert(position, poi.len());
                    }
                }
            }
        }

        // BFS to find distances between POIs.
        let mut todo = VecDeque::new();
        let mut directed: [u64; 36] = [0; 36];
        let mut undirected: [u64; 36] = [0; 36];
        let mut weight = [[0; 36]; 36];

        for (&start, &from) in &poi {
            todo.push_back((start, 0, true));
            grid[&start] = b'#';

            while let Some((position, cost, forward)) = todo.pop_front() {
                for direction in OFFSETS {
                    let next = position + direction;
                    match grid[&next] {
                        b'#' => (),
                        b'P' => {
                            let to = poi[&next];

                            if forward {
                                directed[from] |= 1 << to;
                            } else {
                                directed[to] |= 1 << from;
                            }

                            undirected[from] |= 1 << to;
                            undirected[to] |= 1 << from;

                            weight[from][to] = cost + 1;
                            weight[to][from] = cost + 1;
                        }
                        b'.' => {
                            todo.push_back((next, cost + 1, forward));
                            grid[&next] = b'#';
                        }
                        _ => {
                            let terrain = grid[&next];
                            let same = direction
                                == match terrain {
                                    b'^' => UP,
                                    b'>' => RIGHT,
                                    b'v' => DOWN,
                                    b'<' => LEFT,
                                    _ => panic!("Unknown terrain: '{terrain}'"),
                                };
                            todo.push_back((next, cost + 1, forward && same));
                            grid[&next] = b'#';
                        }
                    }
                }
            }
        }

        // Compress
        let start = undirected[0].trailing_zeros() as usize;
        let end = undirected[1].trailing_zeros() as usize;
        let extra = 2 + weight[0][start] + weight[1][end];

        // Heuristic
        let mut mask = 0;

        for (i, edges) in undirected.iter().enumerate() {
            if edges.count_ones() < 4 {
                mask |= 1 << i;
            }
        }

        for (i, edges) in undirected.iter_mut().enumerate() {
            if edges.count_ones() < 4 {
                *edges = (*edges & !mask) | directed[i];
            }
        }

        Input {
            start,
            end,
            extra,
            directed,
            undirected,
            weight,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
    }

    #[test]
    fn test_start() {
        let input = Input::from(example_input());
        assert_eq!(3, input.start);
    }

    #[test]
    fn test_goal() {
        let input = Input::from(example_input());
        assert_eq!(8, input.end);
    }

    #[test]
    fn test_pt1_example() {
        assert_eq!(String::from("94"), Day {}.pt1(example_input()));
    }

    #[test]
    fn test_pt1() {
        assert_eq!(String::from("2018"), Day {}.pt1(input()));
    }

    #[test]
    fn test_pt2_example() {
        assert_eq!(String::from("154"), Day {}.pt2(example_input()));
    }

    #[test]
    fn test_pt2() {
        assert_eq!(String::from("6406"), Day {}.pt2(input()));
    }
}
