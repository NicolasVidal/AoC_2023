use std::collections::HashMap;
use itertools::Itertools;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut lines = s.lines();
    let pattern = lines.next().unwrap();

    lines.next().unwrap();
    let mut map = HashMap::new();
    for line in lines {
        let mut splits = line.split(" = ");
        let key = splits.next().unwrap();
        let choices = splits.next().unwrap();
        let mut choices = choices.split(", ");
        let left = choices.next().unwrap()[1..].to_string();
        let right = choices.next().unwrap()[..3].to_string();

        map.insert(key, (left, right));
    }

    let mut node = map.get("AAA").unwrap();
    let mut counter = 0;
    loop {
        for (_, c) in pattern.chars().enumerate() {
            let next_node = match c {
                'L' => node.0.as_str(),
                'R' => node.1.as_str(),
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

    let mut starts = vec![];
    lines.next().unwrap();
    let mut map = HashMap::new();
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

    let mut stats = vec![];
    for start in starts.iter() {
        let mut current = (start, 0);
        let mut chars = pattern.chars();

        let mut final_initial_position = None;
        let mut final_loop_step = None;

        loop {
            if current.0.ends_with('Z') {
                if let Some(final_initial_position) = final_initial_position {
                    final_loop_step = Some(current.1 - final_initial_position);
                    break;
                } else {
                    final_initial_position = Some(current.1);
                }
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

        stats.push((start, final_initial_position.unwrap(), final_loop_step.unwrap()));
    }

    let mut cnt = stats.iter().map(|v| v.1).collect::<Vec<_>>();
    let steps = stats.iter().map(|v| v.2).collect::<Vec<_>>();


    loop {
        if cnt.iter().all_equal() {
            return cnt[0];
        }
        let mut smallest = usize::MAX;
        let mut smallest_cnt_ref = 0;

        for (i, cnt_ref) in cnt.iter().enumerate() {
            if *cnt_ref < smallest {
                smallest = cnt[i];
                smallest_cnt_ref = i;
            }
        }

        cnt[smallest_cnt_ref] += steps[smallest_cnt_ref];
    }
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