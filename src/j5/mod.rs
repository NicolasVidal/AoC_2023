use std::collections::HashMap;
use std::str::Lines;
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

fn check_range(mut start: usize, mut count: usize, mut lines: Lines) -> Vec<(usize, usize)> {
    let mut output_ranges = Vec::new();

    let mut self_ranges = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            let _ = lines.next().unwrap();
            break;
        }
        let mut num_split = line.split(' ');
        let destination = num_split.next().unwrap().parse::<usize>().unwrap();
        let source = num_split.next().unwrap().parse::<usize>().unwrap();
        let range = num_split.next().unwrap().parse::<usize>().unwrap();

        self_ranges.push((source, range, destination));
    }

    self_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    for (source, range, destination) in self_ranges {
        if start < source {
            if start + count <= source {
                output_ranges.push((start, count));
                return output_ranges;
            }
            output_ranges.push((start, source - start));
            count = count - (source - start);
            start = source;
        }
        if start < source + range {
            if start + count <= source + range {
                output_ranges.push((destination + start - source, count));
                return output_ranges;
            }
            output_ranges.push((destination + start - source, range - (start - source)));
            count = count - (source + range - start);
            start = source + range;
        }
    }
    output_ranges.push((start, count));
    output_ranges
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

    let mut ranges = Vec::new();

    for mut seed in &seeds.iter().chunks(2) {
        let mut start = *seed.next().unwrap();
        let mut range = *seed.next().unwrap();

        ranges.push((start, range));
    }

    for _ in 0..7 {
        let mut new_ranges = Vec::new();
        for (start, count) in ranges {
            let mut lines = lines.clone();
            for r in check_range(start, count, lines) {
                new_ranges.push(r);
            }
        }
        ranges = new_ranges;

        while let Some(line) = lines.next() {
            if line.is_empty() {
                let _ = lines.next().unwrap();
                break;
            }
        }
    }

    for (start, count) in ranges {
        min = min.min(start);
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
        assert_eq!(69841803, _p2(include_str!("j5.txt")));
    }
}