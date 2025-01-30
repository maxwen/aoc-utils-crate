use priority_queue::PriorityQueue;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use aoc_utils_crate::file::read_lines_as_vec;
use aoc_utils_crate::graph::Graph;

type Point = (i32, i32);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Path,
    Forrest,
    SlopeRight,
    SlopeLeft,
    SlopeDown,
    SlopeUp,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Tile::Path),
            '#' => Ok(Tile::Forrest),
            '^' => Ok(Tile::SlopeUp),
            'v' => Ok(Tile::SlopeDown),
            '<' => Ok(Tile::SlopeLeft),
            '>' => Ok(Tile::SlopeRight),
            _ => Err(()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Path => {
                write!(f, ".")
            }
            Tile::Forrest => {
                write!(f, "#")
            }
            Tile::SlopeRight => {
                write!(f, ">")
            }
            Tile::SlopeLeft => {
                write!(f, "<")
            }
            Tile::SlopeDown => {
                write!(f, "v")
            }
            Tile::SlopeUp => {
                write!(f, "^")
            }
        }
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<Tile>>) {
    for (_y, line) in grid.iter().enumerate() {
        for (_x, tile) in line.iter().enumerate() {
            print!("{}", tile);
        }
        println!();
    }
    println!();
}

fn get_position_for_slope(pos: &Point, tile: &Tile) -> Point {
    match tile {
        Tile::SlopeRight => (pos.0 + 1, pos.1),
        Tile::SlopeLeft => (pos.0 - 1, pos.1),
        Tile::SlopeDown => (pos.0, pos.1 + 1),
        Tile::SlopeUp => (pos.0, pos.1 - 1),
        _ => *pos,
    }
}
fn get_neighbours(grid: &Vec<Vec<Tile>>, pos: &Point) -> Vec<Point> {
    let grid_lines = grid.len() as i32;
    let grid_cols = grid.first().unwrap().len() as i32;

    [
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
    ]
    .iter()
    .filter(|pos| {
        pos.0 >= 0
            && pos.0 < grid_cols
            && pos.1 >= 0
            && pos.1 < grid_lines
            && grid[pos.1 as usize][pos.0 as usize] != Tile::Forrest
    })
    .map(|(r, c)| (*r, *c))
    .collect::<Vec<_>>()
}

fn get_neighbours_without(grid: &Vec<Vec<Tile>>, pos: &Point, filter_pos: &Point) -> Vec<Point> {
    let grid_lines = grid.len() as i32;
    let grid_cols = grid.first().unwrap().len() as i32;

    [
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0 + 1, pos.1),
    ]
        .iter()
        .filter(|pos| {
            pos.0 >= 0
                && pos.0 < grid_cols
                && pos.1 >= 0
                && pos.1 < grid_lines
                && grid[pos.1 as usize][pos.0 as usize] != Tile::Forrest
                && *pos != filter_pos
        })
        .map(|(r, c)| (*r, *c))
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: Point,
    steps: i32,
}

fn bfs(grid: &Vec<Vec<Tile>>, start: Point, end: Point) -> i32 {
    let mut stack: VecDeque<State> = VecDeque::new();
    let s = State {
        pos: start,
        steps: 0,
    };
    stack.push_back(s.clone());

    let mut seen: HashSet<State> = HashSet::new();

    let min = i32::MAX;

    while let Some(current) = stack.pop_front() {
        let steps = current.steps;
        let current_pos = current.pos;

        if current_pos == end {
            // shortest path ONLY because we count steps and all steps have same length
            return steps;
        }

        // just so simple
        if seen.contains(&current) {
            continue;
        }
        seen.insert(current.clone());

        let mut possible_steps = get_neighbours(grid, &current_pos);
        let t = grid[current_pos.1 as usize][current_pos.0 as usize];

        match t {
            Tile::SlopeRight | Tile::SlopeLeft | Tile::SlopeDown | Tile::SlopeUp => {
                possible_steps.clear();
                possible_steps.push(get_position_for_slope(&current_pos, &t))
            }
            _ => {}
        }

        for move_pos in possible_steps {
            let s_new = State {
                pos: move_pos,
                steps: steps + 1,
            };
            stack.push_back(s_new);
        }
    }
    min
}

fn dijkstra(grid: &Vec<Vec<Tile>>, start: Point, end: Point) -> i32 {
    let mut stack = PriorityQueue::new();
    stack.push(start, 0);

    let mut seen: HashMap<Point, i32> = HashMap::new();
    seen.insert(start, 0);

    let mut min = i32::MAX;

    while let Some((current_pos, steps)) = stack.pop() {
        if current_pos == end {
            if steps < min {
                min = steps
            }
        }

        let mut possible_steps = get_neighbours(grid, &current_pos);
        let t = grid[current_pos.1 as usize][current_pos.0 as usize];

        match t {
            Tile::SlopeRight | Tile::SlopeLeft | Tile::SlopeDown | Tile::SlopeUp => {
                possible_steps.clear();
                possible_steps.push(get_position_for_slope(&current_pos, &t))
            }
            _ => {}
        }

        for move_pos in possible_steps {
            let dist_next_pos = seen.get(&move_pos).unwrap_or(&i32::MAX);
            if steps + 1 < *dist_next_pos {
                seen.insert(move_pos, steps + 1);
                stack.push(move_pos, steps + 1);
            }
        }
    }
    min
}

fn dfs_long(
    grid: &Vec<Vec<Tile>>,
    graph: &Graph<Point>,
    current_pos: Point,
    end: Point,
    steps: i32,
    current_max: i32,
    seen: &mut HashSet<Point>,
) -> i32 {
    let mut max = current_max;
    if current_pos == end {
        if steps > current_max {
            max = steps
        }
        return max;
    }

    let possible_steps = graph.nodes.get(&current_pos).unwrap().edges.borrow();

    for (move_pos, length) in possible_steps.iter() {
        if !seen.contains(&move_pos) {
            seen.insert(*move_pos);
            max = dfs_long(grid, graph, *move_pos, end, steps + length, max, seen);
            seen.remove(&move_pos);
        }
    }
    max
}

fn manhatten_distance(pos1: Point, pos2: Point) -> u32 {
    pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1)
}

fn astar(grid: &Vec<Vec<Tile>>, start: Point, end: Point) -> u32 {
    let mut visit_next = BinaryHeap::new();
    visit_next.push((manhatten_distance(start, end), start));

    let mut scores = HashMap::new(); // g-values, cost to reach the node
    scores.insert(start, 0u32);

    let mut estimate_scores = HashMap::new(); // f-values, cost to reach + estimate cost to goal
    let mut min = u32::MAX;

    while let Some((estimate_score, current_pos)) = visit_next.pop() {
        if current_pos == end {
            let cost = scores[&current_pos];
            if cost < min {
                min = cost
            }
        }
        let current_score = scores[&current_pos];

        match estimate_scores.entry(current_pos) {
            Occupied(mut entry) => {
                // If the node has already been visited with an equal or lower score than now, then
                // we do not need to re-visit it.
                if *entry.get() <= estimate_score {
                    continue;
                }
                entry.insert(estimate_score);
            }
            Vacant(entry) => {
                entry.insert(estimate_score);
            }
        }

        let mut possible_steps = get_neighbours(grid, &current_pos);
        let t = grid[current_pos.1 as usize][current_pos.0 as usize];

        match t {
            Tile::SlopeRight | Tile::SlopeLeft | Tile::SlopeDown | Tile::SlopeUp => {
                possible_steps.clear();
                possible_steps.push(get_position_for_slope(&current_pos, &t))
            }
            _ => {}
        }

        for move_pos in possible_steps {
            let move_score = current_score + 1;

            match scores.entry(move_pos) {
                Occupied(mut entry) => {
                    // No need to add neighbors that we have already reached through a shorter path
                    // than now.
                    if *entry.get() <= move_score {
                        continue;
                    }
                    entry.insert(move_score);
                }
                Vacant(entry) => {
                    entry.insert(move_score);
                }
            }
            let next_estimate_score = move_score + manhatten_distance(move_pos, end);
            visit_next.push((next_estimate_score, move_pos));
        }
    }
    min
}

fn build_graph(grid: &Vec<Vec<Tile>>, graph: &mut Graph<Point>, start: Point, end: Point) {
    let mut crossroad_pos_list = HashSet::new();

    crossroad_pos_list.insert(start);
    for (y, line) in grid.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let pos: Point = (x as i32, y as i32);
            let n = get_neighbours(grid, &pos);
            if n.len() > 2 {
                crossroad_pos_list.insert(pos);
            }
        }
    }
    crossroad_pos_list.insert(end);

    for crossroad_pos in crossroad_pos_list.iter() {
        for direction in get_neighbours(grid, crossroad_pos).iter() {
            let mut previous = *crossroad_pos;
            let mut current = *direction;
            let mut length = 1;

            while !crossroad_pos_list.contains(&current) {
                // walk this way until we find next pos that has more then 2 possible ways
                // (all of which we put in bifurcations before)
                // HORRIBLE - we just want the other one thats not previous
                for c in get_neighbours_without(grid, &current, &previous).iter() {
                    // MUST be size 1
                    previous = current;
                    current = *c;
                    break;
                }
                length += 1;
                // next step
            }
            graph.add_edge(*crossroad_pos, current, length);
        }
    }
}

fn main() {
    let lines = read_lines_as_vec("input_test/input_day23.txt").unwrap();

    let mut grid = vec![];
    let mut start: Point = (0, 0);
    let mut end: Point = (0, 0);

    for (y, line) in lines.iter().enumerate() {
        let mut l = vec![];
        for (x, c) in line.chars().enumerate() {
            let pos: Point = (x as i32, y as i32);
            let t = Tile::try_from(c).unwrap();
            if y == 0 && t == Tile::Path {
                start = pos;
            }
            if y == lines.len() - 1 && t == Tile::Path {
                end = pos;
            }
            l.push(Tile::try_from(c).unwrap());
        }
        grid.push(l);
    }

    // print_grid(&grid);
    // println!("{:?} {:?}", start, end);

    println!("bfs = {:?}", bfs(&grid, start, end));
    println!("dijkstra = {:?}", dijkstra(&grid, start, end));
    // println!("astar = {:?}", astar(&grid, start, end));

    // let mut seen = HashSet::new();
    let mut graph = Graph {
        nodes: HashMap::new()
    };

    // let now = Instant::now();
    build_graph(&grid, &mut graph, start, end);
    // println!(
    //     "dfs long = {:?}",
    //     dfs_long(&grid, &graph, start, end, 0, 0, &mut seen)
    // );
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
    println!("dijkstra2 = {:?}", graph.dijkstra(start, end));
    println!("astar2 = {:?}", graph.astar(start, end, manhatten_distance));

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
