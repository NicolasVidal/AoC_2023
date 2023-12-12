use itertools::Itertools;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut total = 0;
    for line in s.lines() {
        let mut parts = line.split(" ");
        let mut rest_of_line = parts.next().unwrap();
        let mut rest_of_constraints: heapless::Vec<(bool, usize), 128> = parts.next().unwrap().split(",")
            .map(|e| (false, e.parse::<usize>().unwrap())).collect();

        count_arrangements_in_line(&mut total, rest_of_line, rest_of_constraints);
    }
    total
}

fn count_arrangements_in_line(total: &mut usize, rest_of_line: &str, rest_of_constraints: heapless::Vec<(bool, usize), 128>) {
    let mut all_constraints = vec![(0, rest_of_constraints)];

    while let Some((start, mut constraint_list)) = all_constraints.pop() {
        if rest_of_line.len() == start {
            if constraint_list.len() == 0 || constraint_list.len() == 1 && constraint_list[0].1 == 0 {
                *total += 1;
            }
            continue;
        }
        let c = rest_of_line.chars().nth(start).unwrap(); {
            match c {
                '.' => {
                    if handle_dot(&mut constraint_list) {
                        all_constraints.push((start + 1, constraint_list));
                    }
                }
                '#' => {
                    if handle_hashtag(&mut constraint_list) {
                        all_constraints.push((start + 1, constraint_list));
                    }
                }
                '?' => {
                    let mut cloned_constraint_list = constraint_list.clone();

                    if handle_dot(&mut cloned_constraint_list) {
                        all_constraints.push((start + 1, cloned_constraint_list));
                    }
                    if handle_hashtag(&mut constraint_list) {
                        all_constraints.push((start + 1, constraint_list));
                    }
                }
                _ => panic!("Invalid char in rest_of_line"),
            }
        }
    }
}

fn handle_hashtag(constraint_list: &mut heapless::Vec<(bool, usize), 128>) -> bool {
    if constraint_list.len() == 0 {
        return false;
    }

    let (started, constraint) = &mut constraint_list[0];

    *started = true;
    if *constraint == 0 {
        return false;
    }
    *constraint -= 1;
    true
}

fn handle_dot(constraint_list: &mut heapless::Vec<(bool, usize), 128>) -> bool {
    if constraint_list.len() == 0 {
        return true;
    }

    let (started, constraint) = &mut constraint_list[0];
    if !*started {
        return true;
    }
    if *constraint == 0 {
        *started = false;
        constraint_list.remove(0);
        return true;
    }
    false
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j12.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut total = 0;
    for (i, line) in s.lines().enumerate() {
        println!("{i}");
        let mut parts = line.split(" ");
        let mut rest_of_line = parts.next().unwrap();
        let mut rest_of_constraints: heapless::Vec<(bool, usize), 128> = parts.next().unwrap().split(",")
            .map(|e| (false, e.parse::<usize>().unwrap())).collect();

        let mut new_str = String::new();
        let mut new_constraints = heapless::Vec::<(bool, usize), 128>::new();
        for i in 0..5 {
            new_str.push_str(rest_of_line);
            if i != 4 {
                new_str.push('?');
            }
            for (started, constraint) in rest_of_constraints.iter() {
                new_constraints.push((*started, *constraint)).unwrap();
            }
        }
        count_arrangements_in_line(&mut total, new_str.as_str(), new_constraints);
    }
    total
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j12.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j12_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(21, _p1(include_str!("j12_test.txt")));
        assert_eq!(6958, _p1(include_str!("j12.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(525152, _p2(include_str!("j12_test.txt")));
        // assert_eq!(42, _p2(include_str!("j12.txt")));
    }
}