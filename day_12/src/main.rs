mod data;
mod input;

use data::*;
use input::*;
use std::collections::BTreeSet;

fn main() {
    println!("day: 12");
    println!("  part 1: {}", part_1(get_input()));
    println!("  part 2: {}", part_2(get_input()));
}

// Find the number of distinct paths that start at start, end at end, and don't visit small caves
// more than once
fn part_1(lines: Vec<String>) -> usize {
    let cave_system = CaveSystem::new(lines);
    let start = cave_system.cave("start");
    part_1_paths(&cave_system, &start, vec![start.clone()]).len()
}

fn part_2(lines: Vec<String>) -> usize {
    let cave_system = CaveSystem::new(lines);
    let start = cave_system.cave("start");
    part_2_paths(&cave_system, &start, vec![start.clone()]).len()
}

fn part_1_paths(cave_system: &CaveSystem, cave: &Cave, path: Path) -> BTreeSet<Path> {
    let mut paths = BTreeSet::new();

    // If this Path is complete, return it early (no need to search backwards)
    if cave_system.path_is_complete(&path) {
        paths.insert(path);
        return paths;
    }

    for adj_cave in cave_system.adjacent_caves(&cave.name) {
        if cave_system.can_visit_next_part_1(&path, &adj_cave.name) {
            let mut new_path = path.clone();
            new_path.push((*adj_cave).clone());
            paths.append(&mut part_1_paths(cave_system, adj_cave, new_path));
        }
    }

    paths
}

fn part_2_paths(cave_system: &CaveSystem, cave: &Cave, path: Path) -> BTreeSet<Path> {
    let mut paths = BTreeSet::new();

    // If this Path is complete, return it early (no need to search backwards)
    if cave_system.path_is_complete(&path) {
        paths.insert(path);
        return paths;
    }

    for adj_cave in cave_system.adjacent_caves(&cave.name) {
        if cave_system.can_visit_next_part_2(&path, &adj_cave.name) {
            let mut new_path = path.clone();
            new_path.push((*adj_cave).clone());
            paths.append(&mut part_2_paths(cave_system, adj_cave, new_path));
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_test_input(1)), 10);
        assert_eq!(part_1(get_test_input(2)), 19);
        assert_eq!(part_1(get_test_input(3)), 226);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_test_input(1)), 36);
        assert_eq!(part_2(get_test_input(2)), 103);
        assert_eq!(part_2(get_test_input(3)), 3509);
    }
}
