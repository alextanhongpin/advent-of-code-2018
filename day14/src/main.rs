fn main() {
    assert_eq!("3811491411".to_string(), part1(vec![3, 7], 846601));

    assert_eq!(20408083, part2(vec![3, 7], "846601"));
}

fn part2(scoreboard: Vec<usize>, final_scores: &str) -> usize {
    let mut scoreboard = scoreboard;
    let mut i = 0;
    let mut j = 1;
    let mut k = 0;

    loop {
        let new_score = scoreboard[i] + scoreboard[j];
        let mut scores = new_score
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
        scoreboard.append(&mut scores);
        i = (i + scoreboard[i] + 1) % scoreboard.len();
        j = (j + scoreboard[j] + 1) % scoreboard.len();

        while k + final_scores.len() < scoreboard.len() {
            if scoreboard[k..k + final_scores.len()]
                .iter()
                .map(ToString::to_string)
                .collect::<String>()
                == final_scores
            {
                return k;
            }
            k += 1
        }
    }
}

fn part1(scoreboard: Vec<usize>, num_recipes: usize) -> String {
    let mut scoreboard = scoreboard;
    let mut i = 0;
    let mut j = 1;

    loop {
        let new_score = scoreboard[i] + scoreboard[j];
        let mut scores = new_score
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
        scoreboard.append(&mut scores);
        i = (i + scoreboard[i] + 1) % scoreboard.len();
        j = (j + scoreboard[j] + 1) % scoreboard.len();

        if scoreboard.len() >= num_recipes + 10 {
            break;
        }
    }

    scoreboard[num_recipes..num_recipes + 10]
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("5158916779".to_string(), part1(vec![3, 7], 9));
        assert_eq!("0124515891".to_string(), part1(vec![3, 7], 5));
        assert_eq!("9251071085".to_string(), part1(vec![3, 7], 18));
        assert_eq!("5941429882".to_string(), part1(vec![3, 7], 2018));
    }

    #[test]
    fn test_part2() {
        assert_eq!(9, part2(vec![3, 7], "51589"));
        assert_eq!(5, part2(vec![3, 7], "01245"));
        assert_eq!(18, part2(vec![3, 7], "92510"));
        assert_eq!(2018, part2(vec![3, 7], "59414"));
    }
}
