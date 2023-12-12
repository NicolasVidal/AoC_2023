use itertools::Itertools;
use rayon::prelude::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CharType {
    Dot,
    Hashtag,
    QuestionMark,
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut total = 0;
    for line in s.lines() {
        let mut parts = line.split(" ");
        let mut rest_of_line = parts.next().unwrap().chars().map(|c| match c {
            '.' => CharType::Dot,
            '#' => CharType::Hashtag,
            '?' => CharType::QuestionMark,
            _ => panic!("Invalid char in rest_of_line"),
        }).collect::<Vec<CharType>>();
        let mut rest_of_constraints: heapless::Vec<(bool, usize), 128> = parts.next().unwrap().split(",")
            .map(|e| (false, e.parse::<usize>().unwrap())).collect();

        count_arrangements_in_line(&mut total, rest_of_line.as_slice(), rest_of_constraints);
    }
    total
}

fn count_arrangements_in_line(total: &mut usize, rest_of_line: &[CharType], rest_of_constraints: heapless::Vec<(bool, usize), 128>) {
    let mut all_constraints = vec![(0, rest_of_constraints)];

    while let Some((mut start, mut constraint_list)) = all_constraints.pop() {
        if rest_of_line.len() <= start {
            if constraint_list.len() == 0 || constraint_list.len() == 1 && constraint_list[0].1 == 0 {
                *total += 1;
            }
            continue;
        }

        let c = rest_of_line[start];
        {
            match c {
                CharType::Dot => {
                    if handle_dot(&mut start, &mut constraint_list) {
                        all_constraints.push((start, constraint_list));
                    }
                }
                CharType::Hashtag => {
                    if handle_hashtag(rest_of_line, &mut start, &mut constraint_list) {
                        all_constraints.push((start, constraint_list));
                    }
                }
                CharType::QuestionMark => {
                    if constraint_list.len() > 0 && constraint_list[0].1 > 0 {
                        if rest_of_line.len() - start < constraint_list[0].1 {
                            continue;
                        }
                    }

                    let mut cloned_constraint_list = constraint_list.clone();
                    let mut cloned_start = start;

                    if handle_dot(&mut cloned_start, &mut cloned_constraint_list) {
                        all_constraints.push((cloned_start, cloned_constraint_list));
                    }
                    if handle_hashtag(rest_of_line, &mut start, &mut constraint_list) {
                        all_constraints.push((start, constraint_list));
                    }
                }
                _ => panic!("Invalid char in rest_of_line"),
            }
        }
    }
}

fn handle_hashtag(line: &[CharType], start: &mut usize, constraint_list: &mut heapless::Vec<(bool, usize), 128>) -> bool {
    if constraint_list.len() == 0 {
        return false;
    }

    let (started, constraint) = &mut constraint_list[0];

    *started = true;
    if *constraint == 0 {
        return false;
    }
    if line.len() - *start < *constraint {
        return false;
    }

    let slice = &line[*start..(*start + *constraint)];
    if !slice.iter().all(|c| *c == CharType::Hashtag || *c == CharType::QuestionMark) {
        return false;
    }

    if line.len() > *start + *constraint {
        if line[*start + *constraint] == CharType::Hashtag {
            return false;
        }
    }

    *start += *constraint + 1;
    constraint_list.remove(0);
    true
}

fn handle_dot(i: &mut usize, constraint_list: &mut heapless::Vec<(bool, usize), 128>) -> bool {
    if constraint_list.len() == 0 {
        *i += 1;
        return true;
    }

    let (started, constraint) = &mut constraint_list[0];
    if !*started {
        *i += 1;
        return true;
    }
    if *constraint == 0 {
        *started = false;
        constraint_list.remove(0);
        *i += 1;
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

    let mut lines = s.lines().enumerate().map(|(i, line)| (i, line)).collect::<Vec<(usize, &str)>>();

    lines.par_iter().map(|(i, line)| {
        let mut total = 0;
        println!("{i}");
        let mut parts = line.split(" ");
        let mut rest_of_line = parts.next().unwrap().chars().map(|c| match c {
            '.' => CharType::Dot,
            '#' => CharType::Hashtag,
            '?' => CharType::QuestionMark,
            _ => panic!("Invalid char in rest_of_line"),
        }).collect::<Vec<CharType>>();
        let mut rest_of_constraints: heapless::Vec<(bool, usize), 128> = parts.next().unwrap().split(",")
            .map(|e| (false, e.parse::<usize>().unwrap())).collect();

        let mut new_str = Vec::new();
        let mut new_constraints = heapless::Vec::<(bool, usize), 128>::new();
        for i in 0..5 {
            new_str.append(&mut rest_of_line.clone());
            if i != 4 {
                new_str.push(CharType::QuestionMark);
            }
            for (started, constraint) in rest_of_constraints.iter() {
                new_constraints.push((*started, *constraint)).unwrap();
            }
        }
        count_arrangements_in_line(&mut total, new_str.as_slice(), new_constraints);
        total
    }).sum()
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