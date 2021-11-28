use std::collections::{HashMap, HashSet};

const DEPTH: i32 = 5913;
const TARGET: (i32, i32) = (8, 701);

fn main() {
    let map = erosion_level(TARGET, DEPTH, TARGET);
    assert_eq!(6256, erosion_level_modulo_3(&map));

    // We multiply by 7, because at worse we have to switch equipment at every tile.
    let bigger_map = erosion_level(TARGET, DEPTH, (TARGET.0 * 7, TARGET.1 * 7));
    assert_eq!(979, shortest_path(&bigger_map, TARGET));
}

fn erosion_level_modulo_3(map: &HashMap<(i32, i32), i32>) -> i32 {
    map.values().map(|n| n % 3).sum()
}

fn erosion_level(tgt: (i32, i32), depth: i32, max: (i32, i32)) -> HashMap<(i32, i32), i32> {
    let mut map = HashMap::new();
    for y in 0..=max.1 {
        for x in 0..=max.0 {
            let geological_index = match (x, y) {
                _ if (x, y) == tgt => 0,
                (0, 0) => 0,
                (x, 0) => x * 16807,
                (0, y) => y * 48271,
                (x, y) => map.get(&(x - 1, y)).unwrap() * map.get(&(x, y - 1)).unwrap(),
            };
            map.insert((x, y), (geological_index + depth) % 20183);
        }
    }
    map
}

type Minutes = i32;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

fn shortest_path(map: &HashMap<(i32, i32), i32>, tgt: (i32, i32)) -> i32 {
    let mut allowed_tools: HashMap<i32, HashSet<Tool>> = HashMap::new();
    allowed_tools.insert(0, HashSet::from_iter([Tool::ClimbingGear, Tool::Torch]));
    allowed_tools.insert(1, HashSet::from_iter([Tool::ClimbingGear, Tool::Neither]));
    allowed_tools.insert(2, HashSet::from_iter([Tool::Torch, Tool::Neither]));

    let mut queue: Vec<((i32, i32), Tool, Minutes)> = vec![((0, 0), Tool::Torch, 0)];
    let mut seen = HashMap::new();

    let mut distances = vec![];

    while !queue.is_empty() {
        let to_move = queue.clone();
        queue.clear();

        for (pos, tool, minutes) in to_move {
            if minutes > *distances.iter().min().unwrap_or(&i32::MAX) {
                continue;
            }
            if pos == tgt {
                if tool == Tool::Torch {
                    distances.push(minutes);
                }
                continue;
            }
            if seen.contains_key(&(pos, tool)) && seen[&(pos, tool)] <= minutes {
                continue;
            }
            seen.insert((pos, tool), minutes);

            let curr_tools = allowed_tools.get(&(map.get(&pos).unwrap() % 3)).unwrap();
            for (x, y) in vec![(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let new_pos = (pos.0 + x, pos.1 + y);
                if map.get(&new_pos).is_none() {
                    continue;
                }
                let next_tools = allowed_tools
                    .get(&(map.get(&new_pos).unwrap() % 3))
                    .unwrap();
                let allowed_tools: HashSet<Tool> =
                    curr_tools.intersection(&next_tools).cloned().collect();
                for new_tool in allowed_tools {
                    if new_tool == tool {
                        queue.push((new_pos, tool, minutes + 1));
                    } else {
                        queue.push((pos, new_tool, minutes + 7));
                    }
                }
            }
        }
    }
    *distances.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let map = erosion_level((10, 10), 510, (10, 10));
        assert_eq!(114, erosion_level_modulo_3(&map));
        let map = erosion_level((10, 10), 510, (12, 12));
        assert_eq!(45, shortest_path(&map, (10, 10)));
    }
}
