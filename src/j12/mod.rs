type SuperMap<'a> = heapless::FnvIndexMap<(&'a [CharType], heapless::Vec<(bool, u8), 128>), usize, 2048>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CharType {
    Dot,
    Hashtag,
    QuestionMark,
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut total = 0;
    for line in s.lines() {
        let mut parts = line.split(' ');
        let mut rest_of_line = parts.next().unwrap().chars().map(|c| match c {
            '.' => CharType::Dot,
            '#' => CharType::Hashtag,
            '?' => CharType::QuestionMark,
            _ => panic!("Invalid char in rest_of_line"),
        }).collect::<heapless::Vec<CharType, 128>>();
        let mut rest_of_constraints: heapless::Vec<(bool, u8), 128> = parts.next().unwrap().split(',')
            .map(|e| (false, e.parse::<u8>().unwrap())).collect();

        let mut known_patterns = heapless::FnvIndexMap::new();
        total += count_arrangements_in_line(rest_of_line.as_slice(),
                                            rest_of_constraints,
                                            &mut known_patterns,
        );
    }
    total
}

fn count_arrangements_in_line<'a>(rest_of_line: &'a[CharType], mut rest_of_constraints: heapless::Vec<(bool, u8), 128>,
                              known_patterns: &mut SuperMap<'a>) -> usize {
    let packed = (rest_of_line, rest_of_constraints.clone());
    if let Some(v) = known_patterns.get(&packed) {
        return *v;
    }

    let mut finish = 0;

    if rest_of_line.is_empty() {
        if rest_of_constraints.is_empty() || rest_of_constraints.len() == 1 && rest_of_constraints[0].1 == 0 {
            finish = 1;
        }
    } else {
        let c = rest_of_line[0];
        match c {
            CharType::Dot => {
                let mut start = 0;
                if handle_dot(&mut start, &mut rest_of_constraints) {
                    finish = count_arrangements_in_line(&rest_of_line[start..], rest_of_constraints, known_patterns)
                }
            }
            CharType::Hashtag => {
                let mut start = 0;
                if handle_hashtag(rest_of_line, &mut start, &mut rest_of_constraints) {
                    start = start.min(rest_of_line.len());
                    finish = count_arrangements_in_line(&rest_of_line[start..], rest_of_constraints, known_patterns)
                }
            }
            CharType::QuestionMark => {
                let mut start = 0;
                if !rest_of_constraints.is_empty() && rest_of_constraints[0].1 > 0 && rest_of_line.len() - start < rest_of_constraints[0].1 as usize {
                } else {
                    let mut cloned_constraint_list = rest_of_constraints.clone();
                    let mut cloned_start = start;

                    if handle_dot(&mut cloned_start, &mut cloned_constraint_list) {
                        cloned_start = cloned_start.min(rest_of_line.len());
                        finish += count_arrangements_in_line(&rest_of_line[cloned_start..], cloned_constraint_list, known_patterns)
                    }
                    if handle_hashtag(rest_of_line, &mut start, &mut rest_of_constraints) {
                        start = start.min(rest_of_line.len());
                        finish += count_arrangements_in_line(&rest_of_line[start..], rest_of_constraints, known_patterns)
                    }
                }
            }
        }
    }

    known_patterns.insert(packed, finish).unwrap();
    finish
}

fn handle_hashtag(line: &[CharType], start: &mut usize, constraint_list: &mut heapless::Vec<(bool, u8), 128>) -> bool {
    if constraint_list.is_empty() {
        return false;
    }

    let (started, constraint) = &mut constraint_list[0];

    *started = true;
    if *constraint == 0 {
        return false;
    }
    if line.len() < *constraint as usize {
        return false;
    }

    let slice = &line[0..(*constraint as usize)];
    if !slice.iter().all(|c| *c == CharType::Hashtag || *c == CharType::QuestionMark) {
        return false;
    }

    if line.len() > *constraint as usize && line[*constraint as usize] == CharType::Hashtag {
        return false;
    }

    *start = (*constraint + 1) as usize;
    constraint_list.remove(0);
    true
}

fn handle_dot(i: &mut usize, constraint_list: &mut heapless::Vec<(bool, u8), 128>) -> bool {
    if constraint_list.is_empty() {
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
    s.lines().enumerate().map(|(i, line)| {
        let mut total = 0;
        let mut parts = line.split(' ');
        let mut rest_of_line = parts.next().unwrap().chars().map(|c| match c {
            '.' => CharType::Dot,
            '#' => CharType::Hashtag,
            '?' => CharType::QuestionMark,
            _ => panic!("Invalid char in rest_of_line"),
        }).collect::<heapless::Vec<CharType, 128>>();
        let mut rest_of_constraints: heapless::Vec<(bool, u8), 128> = parts.next().unwrap().split(',')
            .map(|e| (false, e.parse::<u8>().unwrap())).collect();

        let mut new_str = heapless::Vec::<CharType, 256>::new();
        let mut new_constraints = heapless::Vec::<(bool, u8), 128>::new();
        for i in 0..5 {
            for elt in rest_of_line.clone() {
                new_str.push(elt);
            }
            if i != 4 {
                new_str.push(CharType::QuestionMark);
            }
            for (started, constraint) in rest_of_constraints.iter() {
                new_constraints.push((*started, *constraint)).unwrap();
            }
        }

        let mut known_patterns = heapless::FnvIndexMap::<_, _, 2048>::new();
        total += count_arrangements_in_line(new_str.as_slice(),
                                            new_constraints,
                                            &mut known_patterns);
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
        assert_eq!(6555315065024, _p2(include_str!("j12.txt")));
    }
}