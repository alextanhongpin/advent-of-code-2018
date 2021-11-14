const SIZE: usize = 300;

fn main() {
    let serial_number = 7989;

    let table = sum_area_table(serial_number);
    assert_eq!((29, (19, 17)), max_power(&table, 3));
    assert_eq!(((233, 288), 12), max_power_grid(serial_number));
}

fn max_power_grid(serial_number: i32) -> ((usize, usize), usize) {
    let mut max = 0;
    let mut coord_and_size = ((0, 0), 0);
    let table = sum_area_table(serial_number);

    for size in 1..=300 {
        let (power, (x, y)) = max_power(&table, size);
        if power > max {
            max = power;
            coord_and_size = ((x, y), size);
        }
    }

    coord_and_size
}

fn sum_area(v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut v = v;
    let get_value = |v: &Vec<Vec<i32>>, x: i32, y: i32| {
        v.get(y as usize)
            .cloned()
            .unwrap_or_default()
            .get(x as usize)
            .unwrap_or(&0)
            .clone()
    };

    let row = v.len();
    let col = v[0].len();
    for y in 0..row {
        for x in 0..col {
            v[y][x] = get_value(&v, x as i32, y as i32)
                + get_value(&v, x as i32, y as i32 - 1)
                + get_value(&v, x as i32 - 1, y as i32)
                - get_value(&v, x as i32 - 1, y as i32 - 1);
        }
    }
    v
}

fn sum_area_table(serial_number: i32) -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0; SIZE + 1]; SIZE + 1];
    for y in 1..=SIZE {
        for x in 1..=SIZE {
            grid[y][x] = power_level(serial_number, (x as i32, y as i32));
        }
    }

    sum_area(grid)
}

fn max_power(table: &[Vec<i32>], size: usize) -> (i32, (usize, usize)) {
    let mut max_power = 0;
    let mut coordinate = (0, 0);
    let get_value = |x: i32, y: i32| {
        let x = x - 1;
        let y = y - 1;
        *table
            .get(y as usize)
            .cloned()
            .unwrap_or_default()
            .get(x as usize)
            .unwrap_or(&0)
    };
    for y in 1..=SIZE - size {
        for x in 1..=SIZE - size {
            let top_left = get_value(x as i32, y as i32);
            let top_right = get_value(x as i32 + size as i32, y as i32);
            let bottom_left = get_value(x as i32, y as i32 + size as i32);
            let bottom_right = get_value(x as i32 + size as i32, y as i32 + size as i32);
            let power = bottom_right + top_left - bottom_left - top_right;
            if power > max_power {
                max_power = power;
                coordinate = (x, y);
            }
        }
    }

    (max_power, coordinate)
}

fn power_level(serial_number: i32, (x, y): (i32, i32)) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_number;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10;
    power_level -= 5;
    power_level
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(4, power_level(8, (3, 5)));
        assert_eq!(-5, power_level(57, (122, 79)));
        assert_eq!(0, power_level(39, (217, 196)));
        assert_eq!(4, power_level(71, (101, 153)));
    }

    #[test]
    fn test_max_power() {
        let table = sum_area_table(18);
        assert_eq!((29, (33, 45)), max_power(&table, 3));

        let table = sum_area_table(42);
        assert_eq!((30, (21, 61)), max_power(&table, 3));
    }

    #[test]
    fn test_max_power_grid() {
        assert_eq!(((90, 269), 16), max_power_grid(18));
        assert_eq!(((232, 251), 12), max_power_grid(42));
    }
}
