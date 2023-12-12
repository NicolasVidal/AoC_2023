use itertools::Itertools;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut total = 0;
    for line in s.lines() {
        let mut parts = line.split(" ");
        let mut rest_of_line = parts.next().unwrap();
        let mut rest_of_constraints = parts.next().unwrap().split(",")
            .map(|e| (false, e.parse::<usize>().unwrap())).collect_vec();

        count_arrangements_in_line(&mut total, rest_of_line, rest_of_constraints);
    }
    total
}

fn count_arrangements_in_line(total: &mut usize, rest_of_line: &str, rest_of_constraints: Vec<(bool, usize)>) {
    let mut all_constraints = vec![rest_of_constraints];

    let mut to_remove = Vec::new();

    for c in rest_of_line.chars() {
        // dbg!(&all_constraints);

        match c {
            '.' => {
                to_remove.clear();
                for (i, constraint_list) in all_constraints.iter_mut().enumerate() {
                    handle_dot(&mut to_remove, i, constraint_list);
                }
                for i in to_remove.iter().rev() {
                    all_constraints.swap_remove(*i);
                }
            }
            '#' => {
                to_remove.clear();
                for (i, constraint_list) in all_constraints.iter_mut().enumerate() {
                    handle_hashtag(&mut to_remove, i, constraint_list);
                }
                for i in to_remove.iter().rev() {
                    all_constraints.swap_remove(*i);
                }
            }
            '?' => {
                to_remove.clear();
                let mut cloned_all_constraints = all_constraints.clone();
                for (i, constraint_list) in cloned_all_constraints.iter_mut().enumerate() {
                    handle_dot(&mut to_remove, i, constraint_list);
                }
                for i in to_remove.iter().rev() {
                    cloned_all_constraints.swap_remove(*i);
                }

                to_remove.clear();
                for (i, constraint_list) in all_constraints.iter_mut().enumerate() {
                    handle_hashtag(&mut to_remove, i, constraint_list);
                }
                for i in to_remove.iter().rev() {
                    all_constraints.swap_remove(*i);
                }

                all_constraints.append(&mut cloned_all_constraints);
            }
            _ => panic!("Invalid char in rest_of_line"),
        }
    }
    let count = all_constraints.iter().filter(|e| e.len() == 0 || e.len() == 1 && e[0].1 == 0).count();
    *total += count;
}

fn handle_hashtag(to_remove: &mut Vec<usize>, i: usize, constraint_list: &mut Vec<(bool, usize)>) {
    if constraint_list.len() == 0 {
        to_remove.push(i);
        return;
    }

    let (started, constraint) = &mut constraint_list[0];

    *started = true;
    if *constraint == 0 {
        to_remove.push(i);
        return;
    }
    *constraint -= 1;
}

fn handle_dot(to_remove: &mut Vec<usize>, i: usize, constraint_list: &mut Vec<(bool, usize)>) {
    if constraint_list.len() == 0 {
        return;
    }

    let (started, constraint) = &mut constraint_list[0];
    if !*started {
        return;
    }
    if *constraint == 0 {
        *started = false;
        constraint_list.remove(0);
        return;
    }
    to_remove.push(i);
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
        let mut rest_of_constraints = parts.next().unwrap().split(",")
            .map(|e| (false, e.parse::<usize>().unwrap())).collect_vec();

        let mut new_str = String::new();
        let mut new_constraints = Vec::new();
        for i in 0..5 {
            new_str.push_str(rest_of_line);
            if i != 4 {
                new_str.push('?');
            }
            new_constraints.append(&mut rest_of_constraints.clone());
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
        assert_eq!(42, _p2(include_str!("j12.txt")));
    }
}