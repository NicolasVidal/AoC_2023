const MAX_GRID_WIDTH: usize = 100;
const MAX_GRID_HEIGHT: usize = 100;

const TOTAL: usize = MAX_GRID_WIDTH * MAX_GRID_HEIGHT;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut grid = heapless::Vec::<u8, TOTAL>::new();
    let (height, width) = parse_grid(s, &mut grid);

    let (d_row, d_col) = (-1, 0);
    tilt_panel(&mut grid, d_row, d_col, height, width);


    compute_total(&grid, height, width)
}

fn parse_grid(s: &str, grid: &mut heapless::Vec<u8, TOTAL>) -> (usize, usize) {
    let mut height = 0usize;
    let mut width = 0;
    for line in s.lines() {
        width = line.len();
        for c in line.bytes() {
            grid.push(c).unwrap();
        }
        height += 1;
    }
    (height, width)
}

fn tilt_panel(grid: &mut heapless::Vec<u8, TOTAL>, d_row: isize, d_col: isize, height: usize, width: usize) {
    for row in 0..height {
        for col in 0..width {
            let mut old_row = if d_row <= 0 { row as isize } else { height as isize - row as isize - 1 };
            let mut old_col = if d_col <= 0 { col as isize } else { width as isize - col as isize - 1 };
            match grid[old_row as usize * width + old_col as usize] {
                b'.' => {}
                b'O' => {
                    let mut current_row = old_row + d_row;
                    let mut current_col = old_col + d_col;

                    while current_row >= 0 && current_row < height as isize &&
                        current_col >= 0 && current_col < width as isize {
                        match grid[current_row as usize * width + current_col as usize] {
                            b'.' => {
                                grid[current_row as usize * width + current_col as usize] = b'O';
                                grid[old_row as usize * width + old_col as usize] = b'.';
                            }
                            b'O' => { break; }
                            b'#' => { break; }
                            _ => panic!("Invalid char"),
                        }
                        old_row = current_row;
                        old_col = current_col;
                        current_row += d_row;
                        current_col += d_col;
                    }
                }
                b'#' => {}
                _ => panic!("Invalid char"),
            }
        }
    }
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j14.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut grid = &mut heapless::Vec::<u8, TOTAL>::new();
    let (height, width) = parse_grid(s, &mut grid);

    let last_cycle = grid.clone();
    let mut past_grids = heapless::Vec::<(usize, u128), 124>::new();
    let mut counter = 0usize;
    loop {
        tilt_panel(&mut grid, -1, 0, height, width);
        tilt_panel(&mut grid, 0, -1, height, width);
        tilt_panel(&mut grid, 1, 0, height, width);
        tilt_panel(&mut grid, 0, 1, height, width);

        counter += 1;
        if let Some((j, old_grid)) = past_grids.iter().find(|(j, past_grid)| *past_grid == calculate_hash(&grid))
        {
            if (1_000_000_000 - j) % (counter - j) == 0 {
                break;
            }
        }
        past_grids.push((counter, calculate_hash(grid.as_slice())));
    }

    compute_total(&grid, height, width)
}

fn calculate_hash(t: &[u8]) -> u128 {
    fastmurmur3::hash(t)
}

fn compute_total(grid: &heapless::Vec<u8, TOTAL>, height: usize, width: usize) -> usize {
    let mut total = 0;
    for row in 0..height {
        for col in 0..width {
            match grid[row * width + col] {
                b'.' => {}
                b'O' => {
                    total += height - row;
                }
                b'#' => {}
                _ => panic!("Invalid char"),
            }
        }
    }

    total
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j14.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j14_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(136, _p1(include_str!("j14_test.txt")));
        assert_eq!(108935, _p1(include_str!("j14.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(64, _p2(include_str!("j14_test.txt")));
        assert_eq!(100876, _p2(include_str!("j14.txt")));
    }
}