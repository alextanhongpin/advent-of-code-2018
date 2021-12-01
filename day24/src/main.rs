use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let mut system = System::new(input);
    assert_eq!(18280, system.fight());
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Group {
    id: i32,
    hit_points: i32,
    is_immune: bool,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
    attack_damage: i32,
    attack_type: String,
    initiative: i32,
    units: i32,
}

impl Group {
    fn effective_power(&self) -> i32 {
        self.attack_damage * self.units
    }

    fn effectiveness(&self, other: &Group) -> i32 {
        if other.immunities.contains(&self.attack_type) {
            0
        } else if other.weaknesses.contains(&self.attack_type) {
            2
        } else {
            1
        }
    }

    fn calculate_damage(&self, other: &Group) -> i32 {
        self.effective_power() * self.effectiveness(other)
    }
}

struct System {
    groups: Vec<Group>,
}

impl System {
    fn new(input: &str) -> System {
        let groups = parse(input);
        System { groups }
    }

    fn fight(&mut self) -> i32 {
        let mut unit_by_id = HashMap::new();
        for group in self.groups.clone() {
            unit_by_id.insert(group.id, group);
        }

        loop {
            let (immune_units, infection_units) =
                unit_by_id.iter().fold((0, 0), |groups, (_, group)| {
                    if group.is_immune {
                        (groups.0 + group.units, groups.1)
                    } else {
                        (groups.0, groups.1 + group.units)
                    }
                });
            if immune_units == 0 || infection_units == 0 {
                return immune_units.max(infection_units);
            }
            let mut targets =
                self.target_selection(unit_by_id.values().cloned().collect::<Vec<Group>>());
            targets.sort_by_key(|(id, _)| (!unit_by_id[id].initiative));
            for (attacker, defender) in targets.iter() {
                if !(unit_by_id.contains_key(attacker) && unit_by_id.contains_key(defender)) {
                    continue;
                }
                let attacker = unit_by_id[attacker].clone();
                let mut defender = unit_by_id[defender].clone();
                let damage_dealt = attacker.calculate_damage(&defender);
                let units_killed = damage_dealt / defender.hit_points;
                defender.units = (defender.units - units_killed).max(0);
                if defender.units > 0 {
                    unit_by_id.insert(defender.id, defender);
                } else {
                    unit_by_id.remove(&defender.id);
                }
            }
        }
    }

    fn target_selection(&self, groups: Vec<Group>) -> Vec<(i32, i32)> {
        let mut targets = Vec::new();
        let mut targeted = HashSet::new();

        let mut groups = groups;
        groups.sort_by_key(|u| (!u.effective_power(), !u.initiative));
        for atk in groups.iter() {
            if atk.units <= 0 {
                continue;
            }
            let defender = groups
                .clone()
                .into_iter()
                .filter(|def| {
                    def.is_immune != atk.is_immune
                        && def.units > 0
                        && atk.calculate_damage(def) > 0
                        && !targeted.contains(&def.id)
                })
                .max_by_key(|def| {
                    (
                        atk.calculate_damage(def),
                        def.effective_power(),
                        def.initiative,
                    )
                });
            if let Some(def) = defender {
                targets.push((atk.id, def.id));
                targeted.insert(def.id);
            }
        }
        targets
    }
}

fn parse(input: &str) -> Vec<Group> {
    let units_re = Regex::new(r"^(\d+) units each with (\d+) hit points").unwrap();
    let attack_re =
        Regex::new(r"with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
    let immune_re = Regex::new(r"\((.+?)\)").unwrap();
    let mut groups = Vec::new();
    let mut id = 0;

    let mut is_immune = true;
    for line in input.trim().lines() {
        if line.starts_with("Immune System:") {
            continue;
        }
        if line.starts_with("Infection:") {
            is_immune = false;
            continue;
        }
        if line.trim().is_empty() {
            continue;
        }

        id += 1;
        let mut group = Group {
            id,
            hit_points: 0,
            is_immune,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            attack_damage: 0,
            attack_type: String::new(),
            initiative: 0,
            units: 0,
        };

        let units_cap = units_re.captures(line).unwrap();
        let units = units_cap[1].parse::<i32>().unwrap();
        let hit_points = units_cap[2].parse::<i32>().unwrap();
        group.units = units;
        group.hit_points = hit_points;

        if let Some(attack_cap) = attack_re.captures(line) {
            group.attack_damage = attack_cap[1].parse::<i32>().unwrap();
            group.attack_type = attack_cap[2].to_string();
            group.initiative = attack_cap[3].parse::<i32>().unwrap();
        }

        let immune_cap = immune_re.captures(line);
        if immune_cap.is_none() {
            groups.push(group);
            continue;
        }
        let immune_cap = immune_cap.unwrap();
        let mut parts = immune_cap[1].split(';').map(str::trim).collect::<Vec<_>>();
        parts.sort_unstable();
        if parts.len() == 1 {
            if parts[0].starts_with("weak to") {
                let weaknesses = parts[0]
                    .replace("weak to ", "")
                    .trim()
                    .split(',')
                    .map(str::trim)
                    .map(ToString::to_string)
                    .collect();
                group.weaknesses = weaknesses;
            }
            if parts[0].starts_with("immune to") {
                let immunities = parts[0]
                    .replace("immune to ", "")
                    .trim()
                    .split(',')
                    .map(str::trim)
                    .map(ToString::to_string)
                    .collect();
                group.immunities = immunities;
            }
        } else if parts.len() == 2 {
            if parts[0].starts_with("immune to") {
                let immunities = parts[0]
                    .replace("immune to ", "")
                    .trim()
                    .split(',')
                    .map(str::trim)
                    .map(ToString::to_string)
                    .collect();
                group.immunities = immunities;
            }
            if parts[1].starts_with("weak to") {
                let weaknesses = parts[1]
                    .replace("weak to ", "")
                    .trim()
                    .split(',')
                    .map(str::trim)
                    .map(ToString::to_string)
                    .collect();
                group.weaknesses = weaknesses;
            }
        }
        groups.push(group);
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

        let mut system = System::new(input);
        assert_eq!(5216, system.fight());
    }
}
