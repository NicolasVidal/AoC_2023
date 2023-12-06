#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut lines = s.lines();

    let times = lines.next().unwrap()
        .split(':').nth(1).unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let distances = lines.next().unwrap()
        .split(':').nth(1).unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();

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

    let time = lines.next().unwrap()
        .split(':').nth(1).unwrap()
        .replace(" ", "")
        .parse::<usize>().unwrap();

    let distance = lines.next().unwrap()
        .split(':').nth(1).unwrap()
        .replace(" ", "")
        .parse::<usize>().unwrap();

    let mut total = 0;
    for t in 0..time {
        let d = (time - t) * t;
        if d > distance {
            total += 1;
        }
    }

    total
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