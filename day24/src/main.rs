use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {}

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

    fn calculate_damage(&self, other: &Group) -> i32 {
        if other.immunities.contains(&self.attack_type) {
            0
        } else if other.weaknesses.contains(&self.attack_type) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }

    fn damage(&mut self, other: &mut Group) {
        let damage = self.calculate_damage(other);
        let groups = damage / other.hit_points;
        println!(
            "type({}: {}) deals {} {} damage to type({}: {}) {} groups (HP: {}) with weaknesses({:?}) immunities ({:?})",
            self.id,
            self.is_immune,
            damage,
            self.attack_type,
            other.id,
            other.is_immune,
            groups,
            other.hit_points,
            other.weaknesses,
            other.immunities
        );
        other.units -= groups;
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
            let has_attacker = unit_by_id.iter().any(|(_, group)| group.is_immune);
            let has_defender = unit_by_id.iter().any(|(_, group)| !group.is_immune);
            if has_attacker != has_defender {
                println!(
                    "{:?}",
                    unit_by_id
                        .iter()
                        .map(|(_, group)| group.units)
                        .collect::<Vec<_>>()
                );
                return unit_by_id.iter().map(|(_, group)| group.units).sum::<i32>();
            }
            let mut targets = self.target_selection();
            targets.sort_by_key(|(atk, _)| match unit_by_id.get(atk) {
                Some(group) => -group.initiative,
                None => 0,
            });
            for (attacker, defender) in targets.iter_mut() {
                //println!("{} attacks {:?}", attacker, unit_by_id.get(&attacker));
                if !unit_by_id.contains_key(attacker) || !unit_by_id.contains_key(defender) {
                    continue;
                }
                let mut attacker = unit_by_id[attacker].clone();
                let mut defender = unit_by_id[defender].clone();
                if attacker.units == 0 {
                    unit_by_id.remove(&attacker.id);
                    continue;
                }
                if defender.units == 0 {
                    unit_by_id.remove(&defender.id);
                    continue;
                }

                attacker.damage(&mut defender);
                if defender.units <= 0 {
                    unit_by_id.remove(&defender.id);
                } else {
                    unit_by_id.insert(defender.id, defender);
                }
            }
            println!();
        }
    }

    fn target_selection(&mut self) -> Vec<(i32, i32)> {
        let mut targets = Vec::new();
        let mut targetted = HashSet::new();
        self.groups
            .sort_by_key(|u| (-u.effective_power(), -u.initiative));
        for atk in &self.groups {
            let defenders = self
                .groups
                .clone()
                .into_iter()
                .filter(|def| def.is_immune != atk.is_immune && !targetted.contains(&def.id))
                .collect::<Vec<Group>>();
            let defender = defenders.iter().max_by_key(|def| {
                (
                    atk.calculate_damage(&def),
                    def.effective_power(),
                    def.initiative,
                )
            });
            match defender {
                Some(def) => {
                    targetted.insert(def.id);
                    targets.push((atk.id, def.id));
                }
                None => {
                    continue;
                }
            }
        }
        targets
    }
}

fn parse(input: &str) -> Vec<Group> {
    let units_re = Regex::new(r"^(\d+) groups each with (\d+) hit points").unwrap();
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

        match attack_re.captures(line) {
            Some(attack_cap) => {
                group.attack_damage = attack_cap[1].parse::<i32>().unwrap();
                group.attack_type = attack_cap[2].to_string();
                group.initiative = attack_cap[3].parse::<i32>().unwrap();
            }
            None => (),
        }

        let immune_cap = immune_re.captures(line).unwrap();
        let mut parts = immune_cap[1].split(';').map(str::trim).collect::<Vec<_>>();
        parts.sort();
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
17 groups each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 groups each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 groups each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 groups each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

        let mut system = System::new(input);
        assert_eq!(5216, system.fight());
    }
}
