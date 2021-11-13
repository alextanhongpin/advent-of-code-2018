use std::collections::VecDeque;

fn main() {
    // 424 players; last marble is worth 71482 points
    let marble_points = 71_482;
    let num_players = 424;
    assert_eq!(408_679, simulate(num_players, marble_points));
    assert_eq!(3443939356, simulate(num_players, marble_points * 100));
}

fn simulate(players: usize, marbles: usize) -> i64 {
    let mut circle: VecDeque<i64> = VecDeque::new();
    circle.push_back(0);
    let mut scores: Vec<i64> = vec![0; players];

    for i in 1..=marbles {
        let n = circle.len();
        if i % 23 == 0 {
            // Take the last 7 values and put it to the front.
            circle.rotate_right(7);
            scores[i % players] += i as i64;
            // Remove the front value.
            scores[i % players] += circle.pop_front().unwrap();
        } else {
            // Shif to the left by 2, aka moving right by 2.
            circle.rotate_left(2 % n);
            circle.push_front(i as i64);
        }
    }

    scores.iter().cloned().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(32, simulate(9, 25));
        assert_eq!(8_317, simulate(10, 1_618));
        assert_eq!(146_373, simulate(13, 7_999));
        assert_eq!(2_764, simulate(17, 1_104));
        assert_eq!(54_718, simulate(21, 6_111));
        assert_eq!(37_305, simulate(30, 5_807));
    }
}
