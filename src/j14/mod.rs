#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Cell {
    Empty,
    Rock,
    Cube,
}

const MAX_GRID_WIDTH: usize = 101;
const MAX_GRID_HEIGHT: usize = 101;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut grid = heapless::Vec::<heapless::Vec<Cell, MAX_GRID_WIDTH>, MAX_GRID_HEIGHT>::new();
    parse_grid(s, &mut grid);

    let (d_row, d_col) = (-1, 0);
    tilt_panel(&mut grid, d_row, d_col);


    compute_total(&grid)
}

fn parse_grid(s: &str, grid: &mut heapless::Vec<heapless::Vec<Cell, MAX_GRID_WIDTH>, MAX_GRID_HEIGHT>) {
    for line in s.lines() {
        let mut row = heapless::Vec::<Cell, MAX_GRID_WIDTH>::new();
        for c in line.chars() {
            row.push(match c {
                '.' => Cell::Empty,
                '#' => Cell::Cube,
                'O' => Cell::Rock,
                _ => panic!(),
            }).unwrap();
        }
        grid.push(row).unwrap();
    }
}

fn tilt_panel(grid: &mut heapless::Vec<heapless::Vec<Cell, MAX_GRID_WIDTH>, MAX_GRID_HEIGHT>, d_row: isize, d_col: isize) {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let mut old_row = if d_row <= 0 { row as isize } else { grid.len() as isize - row as isize - 1 };
            let mut old_col = if d_col <= 0 { col as isize } else { grid[0].len() as isize - col as isize - 1 };
            match grid[old_row as usize][old_col as usize] {
                Cell::Empty => {}
                Cell::Rock => {
                    let mut current_row = old_row + d_row;
                    let mut current_col = old_col + d_col;

                    while current_row >= 0 && current_row < grid.len() as isize &&
                        current_col >= 0 && current_col < grid[0].len() as isize {

                        match grid[current_row as usize][current_col as usize] {
                            Cell::Empty => {
                                grid[current_row as usize][current_col as usize] = Cell::Rock;
                                grid[old_row as usize][old_col as usize] = Cell::Empty;
                            }
                            Cell::Rock => { break; }
                            Cell::Cube => { break; }
                        }
                        old_row = current_row;
                        old_col = current_col;
                        current_row += d_row;
                        current_col += d_col;
                    }
                }
                Cell::Cube => {}
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
    let mut grid = heapless::Vec::<heapless::Vec<Cell, MAX_GRID_WIDTH>, MAX_GRID_HEIGHT>::new();
    parse_grid(s, &mut grid);

    let last_cycle = grid.clone();
    let mut past_grids = heapless::Vec::<_, 128>::new();
    let mut counter = 0usize;
    loop {
        tilt_panel(&mut grid, -1, 0);
        tilt_panel(&mut grid, 0, -1);
        tilt_panel(&mut grid, 1, 0);
        tilt_panel(&mut grid, 0, 1);

        counter += 1;
        if let Some((j, old_grid)) = past_grids.iter().rev().find(|(j, past_grid)| *past_grid == grid)
        {
            if (1_000_000_000 - j) % (counter - j) == 0 {
                break
            }
        }
        past_grids.push((counter, grid.clone()));
    }

    compute_total(&mut grid)
}

fn compute_total(grid: &heapless::Vec<heapless::Vec<Cell, MAX_GRID_WIDTH>, MAX_GRID_HEIGHT>) -> usize {
    let mut total = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            match grid[row][col] {
                Cell::Empty => {}
                Cell::Rock => {
                    total += grid.len() - row;
                }
                Cell::Cube => {}
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