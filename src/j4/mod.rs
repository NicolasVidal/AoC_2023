use std::str::Lines;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut total = 0;
    for line in s.lines() {
        let mut count = None;
        let numbers = line.split(':').nth(1).unwrap();
        let mut groups = numbers.split('|');
        let winning = groups.next().unwrap();
        let mine = groups.next().unwrap();

        'mine: for mine_number_str in mine.split(' ') {
            if let Ok(mine_number) = mine_number_str.parse::<usize>() {
                let mut winning_numbers = winning.split(' ');
                for winning_number_str in winning_numbers {
                    if let Ok(winning_number) = winning_number_str.parse::<usize>() {
                        if mine_number == winning_number {
                            if let Some(c) = count {
                                count = Some(c * 2);
                            } else {
                                count = Some(1);
                            }
                            continue 'mine;
                        }
                    }
                }
            }
        }
        if let Some(c) = count {
            total += c;
        }
    }
    total
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j4.txt"))
}

pub fn count_card_match(card: &str) -> Option<usize> {
    let mut count = None;
    let numbers = card.split(':').nth(1).unwrap();
    let mut groups = numbers.split('|');
    let winning = groups.next().unwrap();
    let mine = groups.next().unwrap();

    'mine: for mine_number_str in mine.split(' ') {
        if let Ok(mine_number) = mine_number_str.parse::<usize>() {
            let winning_numbers = winning.split(' ');
            for winning_number_str in winning_numbers {
                if let Ok(winning_number) = winning_number_str.parse::<usize>() {
                    if mine_number == winning_number {
                        if let Some(c) = count {
                            count = Some(c + 1);
                        } else {
                            count = Some(1);
                        }
                        continue 'mine;
                    }
                }
            }
        }
    }
    count
}

pub fn process_card(card: &str, mut lines: Lines) -> usize {
    let mut total = 1;
    if let Some(count) = count_card_match(card) {
        for _ in 0..count {
            total += process_card(lines.next().unwrap(), lines.clone());
        }
    }
    total
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut lines = s.lines();
    let mut total = 0;
    for (i, line) in lines.clone().enumerate() {
        total += process_card(lines.next().unwrap(), lines.clone());
    }
    total
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j4.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j4_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(13, _p1(include_str!("j4_test.txt")));
        assert_eq!(32609, _p1(include_str!("j4.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(30, _p2(include_str!("j4_test.txt")));
        assert_eq!(14624680, _p2(include_str!("j4.txt")));
    }
}