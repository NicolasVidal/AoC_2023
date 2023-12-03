use std::collections::HashMap;
use itertools::Itertools;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut total_number = 0usize;
    for (row, line) in s.lines().enumerate() {
        let mut number = 0;
        let mut validated_number = false;
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                number = c.to_digit(10).unwrap() as usize + number * 10;

                if !validated_number {
                    'validation: for i in 0..3 {
                        let line_id = (row + i).max(1) - 1;
                        match s.lines().nth(line_id) {
                            None => {}
                            Some(line_to_check) => {
                                for j in 0..3 {
                                    let col_id = (col + j).max(1) - 1;
                                    match line_to_check.chars().nth(col_id) {
                                        None => {}
                                        Some(c) => {
                                            if !c.is_ascii_digit() && c != '.' {
                                                validated_number = true;
                                                break 'validation;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if validated_number {
                    total_number += number;
                }
                number = 0;
                validated_number = false;
            }
        }
        if validated_number {
            total_number += number;
        }
    }
    total_number
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j3.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut numbers = HashMap::new();
    for (row, line) in s.lines().enumerate() {
        let mut number = 0;
        let mut row_and_cols = vec![];
        let mut validated_number = false;
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                number = c.to_digit(10).unwrap() as usize + number * 10;

                if !validated_number {
                    'validation: for i in 0..3 {
                        let line_id = (row + i).max(1) - 1;
                        match s.lines().nth(line_id) {
                            None => {}
                            Some(line_to_check) => {
                                for j in 0..3 {
                                    let col_id = (col + j).max(1) - 1;
                                    match line_to_check.chars().nth(col_id) {
                                        None => {}
                                        Some(c) => {
                                            if c == '*' {
                                                validated_number = true;
                                                row_and_cols.push((line_id, col_id));
                                                break 'validation;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if validated_number {
                    for (star_row, star_col) in row_and_cols.iter() {
                        let en = numbers.entry((*star_row, *star_col)).or_insert(vec![]);
                        en.push(number);
                    }
                }
                number = 0;
                row_and_cols.clear();
                validated_number = false;
            }
        }
        if validated_number {
            for (star_row, star_col) in row_and_cols.iter() {
                let en = numbers.entry((*star_row, *star_col)).or_insert(vec![]);
                en.push(number);
            }
        }
    }

    numbers.iter().filter(|(_, v)| v.len() == 2)
        .sorted_by(|((r0, _), _), ((r1, _), _)| r0.cmp(&r1))
        .map(|(_, v)| v[0] * v[1])
        .sum()
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j3.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j3_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(4361, _p1(include_str!("j3_test.txt")));
        assert_eq!(512794, _p1(include_str!("j3.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(467835, _p2(include_str!("j3_test.txt")));
        assert_eq!(67779080, _p2(include_str!("j3.txt")));
    }
}