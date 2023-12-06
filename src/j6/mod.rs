use std::str::Lines;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut lines = s.lines();

    let parse_ints: fn(&mut Lines) -> heapless::Vec<usize, 128> = (|lines: &mut Lines| {
        lines.next().unwrap()
            .split(':').nth(1).unwrap()
            .split(' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect()
    });

    let times = parse_ints(&mut lines);

    let distances: heapless::Vec<usize, 128> = parse_ints(&mut lines);

    let mut super_total = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let mut total = 0;
        for t in 0..*time {
            let d = (time - t) * t;
            if d > *distance {
                total += 1;
            }
        }
        super_total *= total;
    }

    super_total
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j6.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut lines = s.lines();
    let mut lines = s.lines();

    let mut get_split_int: fn(&mut Lines) -> usize = (|lines: &mut Lines| {
        lines.next().unwrap()
            .split(':').nth(1).unwrap()
            .split(' ')
            .flat_map(|s| s.chars())
            .filter_map(|s| if s.is_digit(10) { Some(s.to_digit(10).unwrap() as usize) } else { None })
            .fold(0, |acc, x| acc * 10 + x)
    });

    let time = get_split_int(&mut lines);

    let distance = get_split_int(&mut lines);

    let mut total = 0;
    let mut min_time = 0;
    let mut max_time = time - 1;
    for t in 0..time {
        let d = (time - t) * t;
        if d > distance {
            min_time = t;
            break
        }
    }

    for t in (0..time).rev() {
        let d = (time - t) * t;
        if d > distance {
            max_time = t;
            break
        }
    }

    max_time - min_time + 1
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j6.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j6_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(288, _p1(include_str!("j6_test.txt")));
        assert_eq!(1083852, _p1(include_str!("j6.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(71503, _p2(include_str!("j6_test.txt")));
        assert_eq!(23501589, _p2(include_str!("j6.txt")));
    }
}