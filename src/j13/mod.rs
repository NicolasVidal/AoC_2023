#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    compute_reflections(s, false)
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j13.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    compute_reflections(s, true)
}

fn compute_reflections(s: &str, use_smudge: bool) -> usize {
    let mut grids = heapless::Vec::<heapless::Vec<heapless::Vec<char, 32>, 32>, 128>::new();

    let mut grid = heapless::Vec::new();
    for line in s.lines() {
        if line.is_empty() {
            grids.push(grid.clone()).unwrap();
            grid.clear();
            continue;
        }
        let mut row = heapless::Vec::<char, 32>::new();
        for c in line.chars() {
            row.push(c).unwrap();
        }
        grid.push(row).unwrap();
    }
    grids.push(grid).unwrap();

    let mut total = 0;
    // search horizontally
    for grid in grids.iter() {
        let mut best_row = usize::MAX;
        let mut best_col = usize::MAX;
        'rows: {
            'row: for row in 0..(grid.len() - 1) {
                let mut smudge = None;
                for (upper_row, lower_row) in (0..=row).rev().zip((row + 1)..grid.len()) {
                    for col in 0..grid[0].len() {
                        if grid[upper_row][col] != grid[lower_row][col] {
                            if smudge.is_none() && use_smudge {
                                smudge = Some(());
                            } else {
                                continue 'row;
                            }
                        }
                    }
                }
                if smudge.is_none() && use_smudge {
                    continue 'row;
                }

                best_row = row + 1;
                break 'rows;
            }
        }

        'cols: {
            'col: for col in 0..(grid[0].len() - 1) {
                let mut smudge = None;
                for (left_col, right_col) in (0..=col).rev().zip((col + 1)..grid[0].len()) {
                    for row in 0..grid.len() {
                        if grid[row][left_col] != grid[row][right_col] {
                            if smudge.is_none() && use_smudge {
                                smudge = Some(());
                            } else {
                                continue 'col;
                            }
                        }
                    }
                }
                if smudge.is_none() && use_smudge {
                    continue 'col;
                }

                best_col = col + 1;
                break 'cols;
            }
        }

        if best_row <= best_col {
            total += 100 * best_row;
        } else {
            total += best_col;
        }
    }

    total
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j13.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j13_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(405, _p1(include_str!("j13_test.txt")));
        assert_eq!(30158, _p1(include_str!("j13.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(400, _p2(include_str!("j13_test.txt")));
        assert_eq!(36474, _p2(include_str!("j13.txt")));
    }
}