
#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut total = 0;
    for line in s.lines() {
        let mut numbers = line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect::<heapless::Vec<i64, 128>>();

        let mut lasts = heapless::Vec::<i64, 256>::new();
        loop {

            lasts.push(numbers[numbers.len() - 1]);
            if numbers.iter().all(|v| *v == 0) {
                break;
            }
            for i in 0..(numbers.len() - 1) {
                numbers[i] = numbers[i + 1] - numbers[i];
            }
            numbers.pop();
        }
        let sub_total = lasts.into_iter().sum::<i64>();
        total += sub_total;
    }
    total as usize
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j9.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut total = 0;
    for line in s.lines() {
        let mut numbers = line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect::<heapless::Vec<i64, 128>>();

        let mut firsts = heapless::Vec::<i64, 256>::new();
        loop {

            firsts.push(numbers[0]);
            if numbers.iter().all(|v| *v == 0) {
                break;
            }
            for i in 0..(numbers.len() - 1) {
                numbers[i] = numbers[i + 1] - numbers[i];
            }
            numbers.pop();
        }
        let mut sub_total = 0;
        for i in (0..firsts.len()).rev() {
            sub_total = firsts[i] - sub_total;
        }
        total += sub_total;
    }
    total as usize
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j9.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j9_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(114, _p1(include_str!("j9_test.txt")));
        assert_eq!(1581679977, _p1(include_str!("j9.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(2, _p2(include_str!("j9_test.txt")));
        assert_eq!(889, _p2(include_str!("j9.txt")));
    }
}