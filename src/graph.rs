use std::cell::RefCell;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use priority_queue::PriorityQueue;

pub struct GraphNode<T> {
    pub edges: RefCell<Vec<(T, i32)>>,
}

pub struct Graph<T> {
    pub nodes: HashMap<T, GraphNode<T>>,
}

impl<T> Graph<T>
where
    T: Eq + Hash + Copy + Clone + Ord,
{
    pub fn add_edge(&mut self, start: T, end: T, length: i32) {
        if self.nodes.contains_key(&start) {
            self.nodes
                .get(&start)
                .unwrap()
                .edges
                .borrow_mut()
                .push((end, length))
        } else {
            let node = GraphNode {
                edges: RefCell::new(vec![]),
            };
            node.edges.borrow_mut().push((end, length));
            self.nodes.insert(start, node);
        }
    }

    // does not work with negative length
    pub fn dijkstra(&self, start: T, end: T) -> u32 {
        let mut stack = PriorityQueue::new();
        stack.push(start, 0);

        let mut seen: HashMap<T, u32> = HashMap::new();
        seen.insert(start, 0);

        let mut min = u32::MAX;

        while let Some((current_pos, steps)) = stack.pop() {
            if current_pos == end {
                if steps < min {
                    min = steps
                }
                continue;
            }

            let possible_steps = self.nodes.get(&current_pos).unwrap().edges.borrow();
            for (move_pos, length) in possible_steps.iter() {
                if length < &0 {
                    panic!("negative length is not supported")
                }
                let dist_next_pos = *seen.get(&move_pos).unwrap_or(&u32::MAX);
                let l = *length as u32;
                if steps + l < dist_next_pos {
                    seen.insert(*move_pos, steps + l);
                    stack.push(*move_pos, steps + l);
                }
            }
        }
        min
    }

    // does not work with negative length
    pub fn astar(&self, start: T, end: T, heuristic: fn(T, T) -> u32) -> u32 {
        let mut visit_next = BinaryHeap::new();
        visit_next.push((heuristic(start, end), start));

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

            let possible_steps = self.nodes.get(&current_pos).unwrap().edges.borrow();
            for (move_pos, length) in possible_steps.iter() {
                if length < &0 {
                    panic!("negative length is not supported")
                }
                let l = *length as u32;
                let move_score = current_score + l;

                match scores.entry(*move_pos) {
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
                let next_estimate_score = move_score + heuristic(*move_pos, end);
                visit_next.push((next_estimate_score, *move_pos));
            }
        }
        min
    }
}