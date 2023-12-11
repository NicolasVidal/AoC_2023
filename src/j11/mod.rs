#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    compute_total_paris_dist(s,  1)
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j11.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    compute_total_paris_dist(s,  1000000 - 1)
}

fn compute_total_paris_dist(s: &str, offset: usize) -> usize {
    let mut galaxy_coordinates = heapless::Vec::<(usize, usize), 512>::new();
    let mut row_offset = 0;

    let mut non_empty_columns = heapless::FnvIndexSet::<usize, 512>::new();
    let mut line_size = 0;

    for (row_idx, line) in s.lines().enumerate() {
        line_size = line.len();

        let mut all_empty = true;
        for (column_idx, c) in line.chars().enumerate() {
            match c {
                '.' => continue,
                '#' => {
                    non_empty_columns.insert(column_idx).unwrap();
                    galaxy_coordinates.push((row_idx + row_offset, column_idx)).unwrap();
                    all_empty = false;
                }
                _ => panic!("Unknown char {}", c),
            }
        }
        if all_empty {
            row_offset += offset;
        }
    }

    for col in (0..line_size).rev() {
        if non_empty_columns.contains(&col) {
            continue;
        }

        for coord in galaxy_coordinates.iter_mut() {
            if coord.1 >= col {
                coord.1 += offset;
            }
        }
    }

    let mut total_dist = 0;
    for i in 0..galaxy_coordinates.len() {
        for j in (i + 1)..galaxy_coordinates.len() {
            let dist = (galaxy_coordinates[i].0 as isize - galaxy_coordinates[j].0 as isize).abs()
                + (galaxy_coordinates[i].1 as isize - galaxy_coordinates[j].1 as isize).abs();
            total_dist += dist as usize;
        }
    }

    total_dist
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j11.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j11_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(374, _p1(include_str!("j11_test.txt")));
        assert_eq!(9403026, _p1(include_str!("j11.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(82000210, _p2(include_str!("j11_test.txt")));
        assert_eq!(543018317006, _p2(include_str!("j11.txt")));
    }
}