use regex::Regex;
use std::collections::HashMap;

type GuardID = u32;
type MMDD = (i32, i32);

#[derive(Debug, Eq, PartialEq)]
enum Action {
    BeginShift(GuardID),
    FallAsleep,
    WakeUp,
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        if s.contains("wakes up") {
            Action::WakeUp
        } else if s.contains("falls asleep") {
            Action::FallAsleep
        } else {
            let re = Regex::new(r"#(\d+)").unwrap();
            let id = re.captures(s).unwrap()[1].parse::<u32>().unwrap();
            Action::BeginShift(id)
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Shift {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
}

impl From<&str> for Shift {
    fn from(input: &str) -> Self {
        let input = input.trim();
        let re = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\]").unwrap();
        let cap = re.captures(input).unwrap();
        let year = &cap[1];
        let month = &cap[2];
        let day = &cap[3];
        let hour = &cap[4];
        let minute = &cap[5];
        Shift {
            year: year.parse::<i32>().unwrap(),
            month: month.parse::<i32>().unwrap(),
            day: day.parse::<i32>().unwrap(),
            hour: hour.parse::<i32>().unwrap(),
            minute: minute.parse::<i32>().unwrap(),
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let (part1, part2) = suggest_working_time(input);
    assert_eq!(20859, part1);
    assert_eq!(76576, part2);
}

fn suggest_working_time(input: &str) -> (i32, i32) {
    let input = input.trim();

    let mut lines = input
        .lines()
        .map(|line| (Shift::from(line), Action::from(line)))
        .collect::<Vec<_>>();
    // Sort by month, then date.
    lines.sort_by_key(|k| (k.0.month, k.0.day, k.0.hour, k.0.minute));

    use Action::*;

    let mut last_guard: Option<GuardID> = None;
    let mut timeline: HashMap<MMDD, (GuardID, Vec<i32>)> = HashMap::new();
    for (shift, action) in lines {
        match action {
            BeginShift(id) => {
                last_guard = Some(id);
                timeline
                    .entry((shift.month, shift.day))
                    .or_insert((id, vec![0; 24 * 60]));
            }
            FallAsleep => match last_guard {
                Some(id) => {
                    let (_, minutes) = timeline
                        .entry((shift.month, shift.day))
                        .or_insert((id, vec![0; 24 * 60]));
                    minutes[(shift.hour * 60 + shift.minute) as usize] = 1;
                }
                None => panic!("where's da guard?!"),
            },
            WakeUp => {
                let (_, minutes) = timeline.get_mut(&(shift.month, shift.day)).unwrap();
                for i in (0..(shift.hour * 60 + shift.minute)).rev() {
                    let i = i as usize;
                    if minutes[i] == 1 {
                        break;
                    }
                    minutes[i] = 1;
                }
            }
        }
    }

    // Find the numbers of minutes slept by each guard.
    let mut cumulative_sleep: HashMap<GuardID, i32> = HashMap::new();
    for (_, (guard_id, minutes)) in timeline.clone().into_iter() {
        let minutes_slept = cumulative_sleep.entry(guard_id).or_insert(0);
        *minutes_slept += minutes.iter().sum::<i32>();
    }

    let (sleepy_guard_id, minutes_slept) = cumulative_sleep.iter().max_by_key(|&(_, v)| v).unwrap();
    println!(
        "Guard #{:?} slept {} minutes",
        sleepy_guard_id, minutes_slept
    );

    let mut overlapping_minutes: HashMap<(GuardID, usize), i32> = HashMap::new();
    // Find the minute that the guard was most often asleep.
    for (_, (guard_id, minutes)) in timeline {
        for (i, &min) in minutes.iter().enumerate() {
            if min == 0 {
                continue;
            }
            let minutes_slept = overlapping_minutes.entry((guard_id, i)).or_insert(0);
            *minutes_slept += 1;
        }
    }
    let minute = overlapping_minutes
        .iter()
        .filter(|&((guard_id, _), _)| guard_id == sleepy_guard_id)
        .max_by_key(|&(_, v)| v)
        .unwrap()
        .0
         .1;

    //println!("{:?}", overlapping_minutes);

    let strategy_one = (*sleepy_guard_id as i32) * (minute as i32);

    let (guard_id, minute) = overlapping_minutes
        .iter()
        .max_by_key(|&(_, v)| v)
        .unwrap()
        .0;
    let strategy_two = (*guard_id as i32) * (*minute as i32);

    (strategy_one, strategy_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_shift() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift";
        let shift = Shift::from(input);
        assert_eq!(
            Shift {
                year: 1518,
                month: 11,
                day: 1,
                hour: 0,
                minute: 0,
            },
            shift
        );
    }

    #[test]
    fn test_new_action() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift";
        let action = Action::from(input);
        assert_eq!(Action::BeginShift(10), action);
    }

    #[test]
    fn test_suggest_working_time() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
";
        let (part1, part2) = suggest_working_time(&input);
        assert_eq!(240, part1);
        assert_eq!(4455, part2);
    }
}
