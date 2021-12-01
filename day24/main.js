const example = `Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4`;

const input = `Immune System:
554 units each with 8034 hit points (weak to cold; immune to slashing) with an attack that does 124 bludgeoning damage at initiative 2
285 units each with 3942 hit points (weak to cold) with an attack that does 107 bludgeoning damage at initiative 6
4470 units each with 7895 hit points (immune to bludgeoning; weak to radiation) with an attack that does 17 bludgeoning damage at initiative 1
4705 units each with 8128 hit points (weak to slashing) with an attack that does 14 bludgeoning damage at initiative 8
3788 units each with 7504 hit points (weak to cold, slashing) with an attack that does 17 cold damage at initiative 3
7087 units each with 2733 hit points (weak to bludgeoning) with an attack that does 3 slashing damage at initiative 14
23 units each with 7234 hit points with an attack that does 3132 fire damage at initiative 15
818 units each with 7188 hit points (weak to fire; immune to radiation, slashing) with an attack that does 80 fire damage at initiative 19
3233 units each with 3713 hit points (immune to cold; weak to radiation) with an attack that does 10 radiation damage at initiative 20
1011 units each with 8135 hit points (immune to slashing, cold, bludgeoning; weak to fire) with an attack that does 75 radiation damage at initiative 12

Infection:
136 units each with 37513 hit points (weak to radiation) with an attack that does 492 cold damage at initiative 18
4811 units each with 5863 hit points (weak to radiation, cold; immune to slashing) with an attack that does 2 radiation damage at initiative 17
4057 units each with 9812 hit points (weak to slashing) with an attack that does 4 bludgeoning damage at initiative 11
2828 units each with 30926 hit points (immune to bludgeoning; weak to cold) with an attack that does 19 cold damage at initiative 7
2311 units each with 20627 hit points (immune to slashing) with an attack that does 17 slashing damage at initiative 5
1622 units each with 30824 hit points (weak to slashing, bludgeoning) with an attack that does 34 bludgeoning damage at initiative 4
108 units each with 8628 hit points with an attack that does 139 slashing damage at initiative 13
1256 units each with 51819 hit points (immune to cold, slashing) with an attack that does 63 radiation damage at initiative 16
3681 units each with 21469 hit points (weak to slashing; immune to cold, bludgeoning) with an attack that does 11 cold damage at initiative 9
7289 units each with 6935 hit points (weak to slashing, bludgeoning) with an attack that does 1 fire damage at initiative 10`;

function parseRow(line) {
  const matches = [
    ...line.matchAll(
      /(?<units>\d+) units each with (?<hitPoints>\d+) hit points/gi
    ),
  ];
  const units = Number(matches?.[0]?.groups?.units);
  const hitPoints = Number(matches?.[0]?.groups?.hitPoints);

  const strengthWeaknessesMatches = [
    ...line.matchAll(/\((?<strengthWeaknesses>.+)\)/gi),
  ];
  const strengthWeaknesses =
    strengthWeaknessesMatches?.[0]?.groups?.strengthWeaknesses ?? "";
  const parts = strengthWeaknesses
    .split(";")
    .map((s) => s.trim())
    .sort();
  const immuneTo = [];
  const weakAgainst = [];
  for (let part of parts) {
    if (part.startsWith("weak to")) {
      weakAgainst.push(
        ...part
          .replaceAll("weak to", "")
          .split(",")
          .map((s) => s.trim())
      );
    }
    if (part.startsWith("immune to")) {
      immuneTo.push(
        ...part
          .replaceAll("immune to", "")
          .split(",")
          .map((s) => s.trim())
      );
    }
  }
  const attackMatches = [
    ...line.matchAll(
      /with an attack that does (?<damage>\d+) (?<attackType>\w+) damage at initiative (?<initiative>\d+)/gi
    ),
  ];
  const damage = Number(attackMatches?.[0]?.groups?.damage);
  const attackType = attackMatches?.[0]?.groups?.attackType;
  const initiative = Number(attackMatches?.[0]?.groups?.initiative);

  return {
    units,
    hitPoints,
    immuneTo,
    weakAgainst,
    damage,
    attackType,
    initiative,
  };
}

function parse(input) {
  const lines = input.trim().split("\n");
  let group = "IS";
  const groups = [];
  let id = 0;
  for (const line of lines) {
    if (line.startsWith("Immune System:")) {
      group = "IS";
      continue;
    }

    if (line.startsWith("Infection:")) {
      group = "I";
      continue;
    }

    if (line.trim() === "") {
      continue;
    }
    id++;
    const units = parseRow(line);
    units.group = group;
    units.id = id;
    groups.push(units);
  }
  return groups;
}

function effectivePower({ units, damage }) {
  return units * damage;
}

function damage(attacker, defender) {
  if (defender.weakAgainst.includes(attacker.attackType)) {
    return effectivePower(attacker) * 2;
  }
  if (defender.immuneTo.includes(attacker.attackType)) {
    return 0;
  }
  return effectivePower(attacker);
}

function targetSelection(groups = []) {
  // Target selection.
  groups.sort((a, b) => {
    const aep = effectivePower(a);
    const bep = effectivePower(b);
    const sameEffectivePower = bep === aep;
    const initiativeDesc = b.initiative - a.initiative;
    const effectivePowerDesc = bep - aep;
    return sameEffectivePower ? initiativeDesc : effectivePowerDesc;
  });

  const pairs = {};
  const selected = new Set();
  for (const group of groups) {
    if (group.units <= 0) {
      continue;
    }
    const others = groups.filter((other) => {
      return (
        other.group != group.group &&
        other.units > 0 &&
        damage(group, other) > 0 &&
        !selected.has(other.id)
      );
    });
    others.sort((a, b) => {
      const admg = damage(group, a);
      const bdmg = damage(group, b);

      const aep = effectivePower(a);
      const bep = effectivePower(b);

      return bdmg === admg
        ? bep === aep
          ? b.initiative - a.initiative
          : bep - aep
        : bdmg - admg;
    });
    if (others.length) {
      pairs[group.id] = others[0].id;
      selected.add(others[0].id);
    }
  }

  return Object.entries(pairs).map((kv) => kv.map(Number));
}

function fight(groupById = {}, pairs = []) {
  pairs.sort(([a], [b]) => groupById[b].initiative - groupById[a].initiative);

  for (const [attackerId, defenderId] of pairs) {
    const attacker = groupById[attackerId];
    const defender = groupById[defenderId];
    if (!(attacker && defender)) {
      continue;
    }
    const damageDealt = damage(attacker, defender);
    const unitsDie = Math.floor(damageDealt / defender.hitPoints);
    defender.units -= unitsDie;
    defender.units = Math.max(0, defender.units);
    if (!defender.units) {
      delete groupById[defender.id];
    } else {
      groupById[defender.id] = defender;
    }
  }

  return Object.values(groupById);
}

function hasGroup(groups = []) {
  const unitsByGroup = groups.reduce((acc, group) => {
    if (!acc[group.group]) {
      acc[group.group] = 0;
    }
    acc[group.group] += group.units;
    return acc;
  }, {});
  return Object.keys(unitsByGroup).length > 1;
}

function boostDamage(groups = [], boost = 0) {
  return groups.map((group) => {
    if (group.group === "IS") {
      group.damage += boost;
    }
    return group;
  });
}

function execute(input, boost = 0) {
  let groups = boostDamage(parse(input), boost);
  while (hasGroup(groups)) {
    const unitsBefore = groups.reduce((acc, group) => acc + group.units, 0);
    const pairs = targetSelection(groups);
    const groupById = groups.reduce((acc, group) => {
      acc[group.id] = group;
      return acc;
    }, {});
    groups = fight(groupById, pairs);
    const unitsAfter = groups.reduce((acc, group) => acc + group.units, 0);
    if (unitsBefore === unitsAfter) {
      return groups;
    }
  }
  return groups;
}

function part1(input) {
  const groups = execute(input);
  const unitsRemaining = groups.reduce((acc, group) => acc + group.units, 0);
  return unitsRemaining;
}

function immuneSystemWin(groups = []) {
  return groups.every((group) => group.group === "IS");
}

function part2(input) {
  let boost = 0;
  groups = execute(input, boost);
  while (!immuneSystemWin(groups)) {
    boost += 1;
    groups = execute(input, boost);
  }
  const unitsRemaining = groups.reduce((acc, group) => acc + group.units, 0);
  console.log("boost: " + boost, "units", unitsRemaining);
  return unitsRemaining;
}

(function main() {
  if (part1(example) !== 5216) {
    throw new Error("part1Error: got " + part1(example));
  }
  if (part1(input) !== 18280) {
    throw new Error("part1Error: got " + part1(input));
  }

  if (part2(example) !== 51) {
    throw new Error("part2Error: got " + part2(example));
  }

  if (part2(input) !== 4573) {
    throw new Error("part2Error: got " + part2(input));
  }
})();
