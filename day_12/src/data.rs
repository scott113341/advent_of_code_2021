use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};

pub type CaveName = String;
pub type Path = Vec<Cave>;

#[derive(Eq, PartialEq, Debug)]
pub struct CaveSystem {
    pub caves: HashMap<CaveName, Cave>,
}

impl CaveSystem {
    pub fn new(lines: Vec<String>) -> CaveSystem {
        let mut caves = HashMap::new();

        for line in lines {
            let (c1, c2) = line.split_once('-').unwrap();

            let cave_1 = caves.entry(c1.into()).or_insert_with(|| Cave::new(c1));
            cave_1.connected_cave_names.insert(c2.into());

            let cave_2 = caves.entry(c2.into()).or_insert_with(|| Cave::new(c2));
            cave_2.connected_cave_names.insert(c1.into());
        }

        CaveSystem { caves }
    }

    pub fn cave<'a>(&self, name: impl Into<CaveName>) -> &Cave {
        self.caves.get(&name.into()).unwrap()
    }

    pub fn adjacent_caves<'a>(&self, name: impl Into<CaveName>) -> Vec<&Cave> {
        self.cave(name)
            .connected_cave_names
            .iter()
            .map(|cn| self.cave(cn))
            .collect()
    }

    pub fn path_is_complete(&self, path: &Path) -> bool {
        path.last() == Some(self.cave("end"))
    }

    // Small caves can only be visited once
    pub fn can_visit_next_part_1(&self, path: &Path, cave_name: &CaveName) -> bool {
        let cave = self.cave(cave_name);

        if cave.is_small && path.contains(cave) {
            false
        } else {
            true
        }
    }

    // At most, one small cave can be visited twice; all other small caves can only be visited once.
    // Also, the start can't be returned to, and once the end is reached, it's over.
    pub fn can_visit_next_part_2(&self, path: &Path, cave_name: &CaveName) -> bool {
        let cave = self.cave(cave_name);

        if cave.is_start {
            false
        } else if cave.is_small && path.contains(cave) {
            self.can_visit_a_small_cave_again(path)
        } else {
            true
        }
    }

    fn can_visit_a_small_cave_again(&self, path: &Path) -> bool {
        let mut small_caves = HashSet::new();

        for cave in path.iter().filter(|c| c.is_small) {
            if small_caves.contains(&cave.name) {
                return false;
            } else {
                small_caves.insert(&cave.name);
            }
        }

        true
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Cave {
    pub name: CaveName,
    pub is_big: bool,
    pub is_small: bool,
    pub is_start: bool,
    pub is_end: bool,
    pub connected_cave_names: BTreeSet<CaveName>,
}

impl Cave {
    pub fn new(name: &str) -> Cave {
        let is_big = name == name.to_ascii_uppercase();

        Cave {
            name: name.into(),
            is_big,
            is_small: !is_big,
            is_start: name == "start",
            is_end: name == "end",
            connected_cave_names: BTreeSet::new(),
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Cave({})", self.name))
    }
}

impl Debug for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}
