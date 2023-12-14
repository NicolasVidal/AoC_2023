#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut sum = 0;
    'lines: for line in s.lines() {
        let mut first = None;
        let mut last = None;
        for c in line.chars() {
            if c.is_ascii_digit() {
                if first.is_none() {
                    first = Some(c.to_digit(10).unwrap() as usize);
                }
                last = Some(c.to_digit(10).unwrap() as usize);
            }
        }
        sum += first.unwrap() * 10 + last.unwrap();
    }
    sum
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j1.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut sum = 0;
    for line in s.lines() {
        let mut chars = line.chars();

        let numbers = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let digits = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

        let mut first = None;
        let mut last = 0;
        let mut chars_count = line.chars().count();
        for c in 0..chars_count {
            for (i, number) in numbers.iter().enumerate() {
                if line[c..].starts_with(number) || line[c..].starts_with(digits[i]) {
                    if first.is_none() {
                        first = Some(i + 1);
                    }
                    last = i + 1;
                }
            }
        }
        sum += first.unwrap() * 10 + last;
    }
    sum
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j1.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j1_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(142, _p1(include_str!("j1_test.txt")));
        assert_eq!(55002, _p1(include_str!("j1.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(281, _p2(include_str!("j1_p2_test.txt")));
        assert_eq!(55093, _p2(include_str!("j1.txt")));
    }
}