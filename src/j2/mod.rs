
#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut total_possible = 0usize;

    const TOTAL_RED: usize = 12;
    const TOTAL_GREEN: usize = 13;
    const TOTAL_BLUE: usize = 14;

    'line: for line in s.lines() {
        let mut split = line.split(':');
        let game = split.next().unwrap();
        let game_num = game.split(' ').nth(1).unwrap().parse::<usize>().unwrap();
        let sets = split.next().unwrap();
        let mut sets = sets.split(';');
        for set in sets {
            let mut blue_count = 0usize;
            let mut green_count = 0usize;
            let mut red_count = 0usize;
            for draw in set.split(',') {
                let mut draw = draw.split(' ');
                draw.next().unwrap();
                let count = draw.next().unwrap().parse::<usize>().unwrap();
                let color = draw.next().unwrap();
                match color {
                    "blue" => blue_count += count,
                    "green" => green_count += count,
                    "red" => red_count += count,
                    _ => panic!("Unknown color"),
                }
                if blue_count > TOTAL_BLUE || green_count > TOTAL_GREEN || red_count > TOTAL_RED {
                    continue 'line;
                }
            }
        }
        total_possible += game_num;
    }
    total_possible
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j2.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {

    let mut total_possible = 0usize;

    const TOTAL_RED: usize = 12;
    const TOTAL_GREEN: usize = 13;
    const TOTAL_BLUE: usize = 14;

    'line: for line in s.lines() {
        let mut split = line.split(':');
        let game = split.next().unwrap();
        let game_num = game.split(' ').nth(1).unwrap().parse::<usize>().unwrap();
        let sets = split.next().unwrap();
        let mut sets = sets.split(';');
        let mut min_blue = 0usize;
        let mut min_green = 0usize;
        let mut min_red = 0usize;
        for set in sets {
            let mut blue_count = 0usize;
            let mut green_count = 0usize;
            let mut red_count = 0usize;
            for draw in set.split(',') {
                let mut draw = draw.split(' ');
                draw.next().unwrap();
                let count = draw.next().unwrap().parse::<usize>().unwrap();
                let color = draw.next().unwrap();
                match color {
                    "blue" => blue_count += count,
                    "green" => green_count += count,
                    "red" => red_count += count,
                    _ => panic!("Unknown color"),
                }
                min_blue = min_blue.max(blue_count);
                min_green = min_green.max(green_count);
                min_red = min_red.max(red_count);
            }
        }
        let power = min_blue * min_green * min_red;
        total_possible += power;
    }
    total_possible
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j2.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j2_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(8, _p1(include_str!("j2_test.txt")));
        assert_eq!(2331, _p1(include_str!("j2.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(2286, _p2(include_str!("j2_test.txt")));
        assert_eq!(71585, _p2(include_str!("j2.txt")));
    }
}