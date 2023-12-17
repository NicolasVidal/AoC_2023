#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut lines = s.lines();
    let pattern = lines.next().unwrap();

    lines.next().unwrap();
    let mut map = heapless::FnvIndexMap::<&str, (&str, &str), 2048>::new();
    for line in lines {
        let mut splits = line.split(" = ");
        let key = splits.next().unwrap();
        let choices = splits.next().unwrap();
        let mut choices = choices.split(", ");
        let left = &choices.next().unwrap()[1..];
        let right = &choices.next().unwrap()[..3];

        map.insert(key, (left, right));
    }

    let mut node = map.get("AAA").unwrap();
    let mut counter = 0;
    loop {
        for c in pattern.chars() {
            let next_node = match c {
                'L' => node.0,
                'R' => node.1,
                _ => panic!("Unknown char {}", c),
            };
            counter += 1;
            if next_node == "ZZZ" {
                return counter;
            }
            node = map.get(next_node).unwrap();
        }
    }
    panic!()
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j8.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut lines = s.lines();
    let pattern = lines.next().unwrap();

    let mut starts = heapless::Vec::<&str, 16>::new();
    lines.next().unwrap();
    let mut map = heapless::FnvIndexMap::<&str, (&str, &str), 2048>::new();
    for line in lines {
        let mut splits = line.split(" = ");
        let key = splits.next().unwrap();
        if key.ends_with('A') {
            starts.push(key);
        }
        let choices = splits.next().unwrap();
        let mut choices = choices.split(", ");
        let left = &choices.next().unwrap()[1..];
        let right = &choices.next().unwrap()[..3];

        map.insert(key, (left, right));
    }

    let mut stats = heapless::Vec::<usize, 16>::new();
    for start in starts.iter() {
        let mut current = (start, 0);
        let mut chars = pattern.chars();

        let mut final_initial_position = None;

        loop {
            if current.0.ends_with('Z') {
                final_initial_position = Some(current.1);
                break;
            }
            loop {
                match chars.next() {
                    Some('L') => {
                        current.0 = &map.get(current.0).unwrap().0;
                        current.1 += 1;
                        break;
                    }
                    Some('R') => {
                        current.0 = &map.get(current.0).unwrap().1;
                        current.1 += 1;
                        break;
                    }
                    _ => chars = pattern.chars(),
                }
            }
        };

        stats.push(final_initial_position.unwrap());
    }

    let gcd_func = |a: usize, b: usize| -> usize {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    };

    let lcm_func = |list: &[usize]| -> usize {
        let mut lcm = list[0];
        for item in list.iter().skip(1) {
            lcm = lcm * *item / gcd_func(lcm, *item);
        }
        lcm
    };

    lcm_func(&stats)
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j8.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j8_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(2, _p1(include_str!("j8_test.txt")));
        assert_eq!(15517, _p1(include_str!("j8.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(6, _p2(include_str!("j8_test_p2.txt")));
        assert_eq!(14935034899483, _p2(include_str!("j8.txt")));
    }
}