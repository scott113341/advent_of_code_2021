use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct Coord {
    pub y: isize,
    pub x: isize,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Cave {
    pub nodes: BTreeMap<Coord, usize>,
    pub width: usize,
}

impl Cave {
    pub fn new(lines: &Vec<String>) -> Cave {
        let mut nodes = BTreeMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coord = Coord {
                    x: x as isize,
                    y: y as isize,
                };
                nodes.insert(coord, c.to_digit(10).unwrap() as usize);
            }
        }

        Cave {
            nodes,
            width: lines[0].len(),
        }
    }

    pub fn get(&self, coord: &Coord) -> Option<usize> {
        self.nodes.get(coord).cloned()
    }

    pub fn adjacent(&self, coord: &Coord) -> Vec<(Coord, usize)> {
        let up = Coord {
            x: coord.x,
            y: coord.y - 1,
        };
        let down = Coord {
            x: coord.x,
            y: coord.y + 1,
        };
        let left = Coord {
            x: coord.x - 1,
            y: coord.y,
        };
        let right = Coord {
            x: coord.x + 1,
            y: coord.y,
        };

        [
            self.get(&up).and_then(|risk| Some((up, risk))),
            self.get(&down).and_then(|risk| Some((down, risk))),
            self.get(&left).and_then(|risk| Some((left, risk))),
            self.get(&right).and_then(|risk| Some((right, risk))),
        ]
        .into_iter()
        .filter_map(|e| e)
        .collect()
    }

    // Dijkstra
    pub fn lowest_risk_path(&self) -> usize {
        let start_node = Coord { x: 0, y: 0 };
        let end_node = self.nodes.iter().last().unwrap().0;

        let mut visited_nodes = BTreeMap::new();
        visited_nodes.insert(start_node.clone(), 0);

        // Use Reverse<T> because we want a min-heap, so that ::pop returns the lowest-risk coord
        let mut nodes_to_visit: BinaryHeap<(Reverse<usize>, Coord)> = self
            .adjacent(&start_node)
            .into_iter()
            .map(|(coord, risk)| (Reverse(risk), coord))
            .collect();

        while let Some((Reverse(current_risk), current_node)) = nodes_to_visit.pop() {
            if visited_nodes.contains_key(&end_node) {
                break;
            }

            for (adj_node, adj_risk) in self.adjacent(&current_node) {
                // For not-yet-visited adjacent nodes, compute risk and queue for visiting
                visited_nodes.entry(adj_node.clone()).or_insert_with(|| {
                    let this_risk = current_risk + adj_risk;
                    nodes_to_visit.push((Reverse(this_risk), adj_node));
                    this_risk
                });
            }
        }

        *visited_nodes.get(end_node).unwrap()
    }
}
