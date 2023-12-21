use std::collections::HashSet;

const GRID_CELLS_COUNT: usize = 132*132;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut grid = heapless::Vec::<u8, GRID_CELLS_COUNT>::new();
    let mut current_pos = heapless::Vec::<u8, GRID_CELLS_COUNT>::new();

    let mut rows = s.lines().count();
    let mut cols = s.lines().next().unwrap().bytes().count();

    for line in s.lines() {
        for c in line.bytes() {
            if c == b'S' {
                current_pos.push(b'O').unwrap();
                grid.push(b'.').unwrap();
            } else {
                current_pos.push(b'.').unwrap();
                grid.push(c).unwrap();
            }
        }
    }

    for _ in 0..64 {
        let mut new_pos = (0..current_pos.len()).map(|_| b'.').collect::<heapless::Vec<u8, GRID_CELLS_COUNT>>();

        for row in 0..rows {
            for col in 0..cols {
                if current_pos[row * cols + col] == b'O' {
                    if row > 0 && grid[(row - 1) * cols + col] == b'.' {
                        new_pos[(row - 1) * cols + col] = b'O';
                    }
                    if row < rows - 1 && grid[(row + 1) * cols + col] == b'.' {
                        new_pos[(row + 1) * cols + col] = b'O';
                    }
                    if col > 0 && grid[row * cols + col - 1] == b'.' {
                        new_pos[row * cols + col - 1] = b'O';
                    }
                    if col < cols - 1 && grid[row * cols + col + 1] == b'.' {
                        new_pos[row * cols + col + 1] = b'O';
                    }
                }
            }
        }

        current_pos = new_pos;
    }

    // for row in 0..rows {
    //     for col in 0..cols {
    //         if current_pos[row * cols + col] == b'O' {
    //             grid[row * cols + col] = b'O';
    //         }
    //         print!("{}", grid[row * cols + col] as char);
    //     }
    //     println!();
    // }
    // println!();

    current_pos.iter().filter(|c| **c == b'O').count()
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j21.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut grid = heapless::Vec::<u8, GRID_CELLS_COUNT>::new();

    let mut rows = s.lines().count();
    let mut cols = s.lines().next().unwrap().bytes().count();
    let mut start_pos = 0;

    for line in s.lines() {
        for c in line.bytes() {
            if c == b'S' {
                grid.push(b'.').unwrap();
                start_pos = grid.len() - 1;
            } else {
                grid.push(c).unwrap();
            }
        }
    }

    let start_row = start_pos / cols;
    let start_col = start_pos % cols;

    let mut total_steps = 265013isize;

    let mut total = 0;

    let mut explored = HashSet::<(isize, isize, bool)>::new();
    let mut to_explore = Vec::<(isize, isize)>::new();
    let mut next_to_explore = Vec::<(isize, isize)>::new();

    to_explore.push((start_row as isize, start_col as isize));

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut points = Vec::new();
    let mut filter_even = false;
    for i in 0..=5000{
        let even = (i) % 2 == 0;
        filter_even = !filter_even;
        next_to_explore.clear();
        while let Some((row, col)) = to_explore.pop() {
            if explored.contains(&(row, col, even)) {
                continue;
            }
            explored.insert((row, col, even));

            for (dr, dc) in directions.iter() {
                let mut grid_row = row + dr;
                while grid_row < 0 {
                    grid_row += rows as isize;
                }

                let mut grid_col = col + dc;
                while grid_col < 0 {
                    grid_col += cols as isize;
                }

                let grid_row = grid_row as usize % rows;
                let grid_col = grid_col as usize % cols;

                if grid[grid_row * cols + grid_col] == b'.' {
                    next_to_explore.push(( row + dr, col + dc));
                }
            }
        }
        to_explore.append(&mut next_to_explore);
        points.push((i, explored.iter().filter(|(row, col, even)| *even == filter_even).count()));
    }

    for (x, y) in points {
        println!("{{{x}, {y}}},");
    }

    explored.iter().filter(|(row, col, even)| *even).count()
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j21.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j21_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(16, _p1(include_str!("j21_test.txt")));
        assert_eq!(3617, _p1(include_str!("j21.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        // assert_eq!(42, _p2(include_str!("j21_test.txt")));
        assert_eq!(42, _p2(include_str!("j21.txt")));
    }
}