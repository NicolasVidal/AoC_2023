use std::collections::HashMap;
use itertools::Itertools;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut seeds = Vec::new();

    let mut lines = s.lines();

    let mut seeds_line = lines.next().unwrap();
    for num_str in seeds_line.split(": ").nth(1).unwrap().split(' ') {
        if let Ok(num) = num_str.parse::<usize>() {
            seeds.push(num);
        }
    }
    lines.next().unwrap();
    lines.next().unwrap();


    let mut min = usize::MAX;
    for seed in seeds {
        let mut lines = lines.clone();
        let mut seed = seed;
        'map: while let Some(line) = lines.next() {
            if line.is_empty() {
                let _ = lines.next().unwrap();
                continue;
            }
            let mut num_split = line.split(' ');
            let destination = num_split.next().unwrap().parse::<usize>().unwrap();
            let source = num_split.next().unwrap().parse::<usize>().unwrap();
            let range = num_split.next().unwrap().parse::<usize>().unwrap();

            if (source..(source + range)).contains(&seed) {
                seed = destination + (seed - source);

                while let Some(line) = lines.next() {
                    if line.is_empty() {
                        let _ = lines.next().unwrap();
                        continue 'map;
                    }
                }
            }
        }
        min = min.min(seed);
    }
    min
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j5.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut seeds = Vec::new();

    let mut lines = s.lines();

    let mut seeds_line = lines.next().unwrap();
    for num_str in seeds_line.split(": ").nth(1).unwrap().split(' ') {
        if let Ok(num) = num_str.parse::<usize>() {
            seeds.push(num);
        }
    }
    lines.next().unwrap();
    lines.next().unwrap();



    let mut min = usize::MAX;
    for mut seed in &seeds.iter().chunks(2) {
        let mut start = *seed.next().unwrap();
        let mut range = *seed.next().unwrap();
        for seed in start..(start + range) {
            let mut lines = lines.clone();
            let mut seed = seed;
            'map: while let Some(line) = lines.next() {
                if line.is_empty() {
                    let _ = lines.next().unwrap();
                    continue;
                }
                let mut num_split = line.split(' ');
                let destination = num_split.next().unwrap().parse::<usize>().unwrap();
                let source = num_split.next().unwrap().parse::<usize>().unwrap();
                let range = num_split.next().unwrap().parse::<usize>().unwrap();

                if (source..(source + range)).contains(&seed) {
                    seed = destination + (seed - source);

                    while let Some(line) = lines.next() {
                        if line.is_empty() {
                            let _ = lines.next().unwrap();
                            continue 'map;
                        }
                    }
                }
            }
            min = min.min(seed);
        }
    }
    min
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j5.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j5_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(35, _p1(include_str!("j5_test.txt")));
        assert_eq!(177942185, _p1(include_str!("j5.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(46, _p2(include_str!("j5_test.txt")));
        assert_eq!(42, _p2(include_str!("j5.txt")));
    }
}