const TOTAL_MAX_SIZE: usize = 110 * 110;

#[derive(Clone, Debug)]
struct EnergizedCell {
    right: bool,
    down: bool,
    left: bool,
    up: bool,
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let (grid, mut energized, width, height) = parse_grid(s);

    let start_beam = ((0, -1), (0, 1));
    run_beam(&grid, &mut energized, width, height, start_beam)
}

fn parse_grid(s: &str) -> (heapless::Vec<u8, TOTAL_MAX_SIZE>, heapless::Vec<Option<EnergizedCell>, TOTAL_MAX_SIZE>, usize, usize) {
    let mut grid = heapless::Vec::<u8, TOTAL_MAX_SIZE>::new();
    let mut energized = heapless::Vec::<_, TOTAL_MAX_SIZE>::new();
    for line in s.lines() {
        for c in line.bytes() {
            grid.push(c).unwrap();
            energized.push(None).unwrap();
        }
    }
    let width = s.lines().next().unwrap().len();
    let height = grid.len() / width;

    (grid, energized, width, height)
}

fn run_beam(grid: &heapless::Vec<u8, TOTAL_MAX_SIZE>, energized: &mut heapless::Vec<Option<EnergizedCell>, TOTAL_MAX_SIZE>, width: usize, height: usize, start_beam: ((isize, isize), (isize, isize))) -> usize {
    let mut beams = heapless::Vec::<_, 128>::new();
    let mut beams_to_remove = heapless::Vec::<_, 128>::new();
    let mut beams_to_add = heapless::Vec::<_, 128>::new();
    beams.push(start_beam).unwrap();
    let mut total = 0;
    while !beams.is_empty() {
        beams_to_remove.clear();
        beams_to_add.clear();
        for (i, ((row, col), (d_row, d_col))) in beams.iter_mut().enumerate() {
            *row += *d_row;
            *col += *d_col;

            if *row < 0 || *row >= height as isize ||
                *col < 0 || *col >= width as isize {
                beams_to_remove.push(i).unwrap();
                continue;
            }

            let new_cell_index = (*row) * width as isize + (*col);

            if energized[new_cell_index as usize].is_none() {
                energized[new_cell_index as usize] = Some(EnergizedCell {
                    left: false,
                    right: false,
                    up: false,
                    down: false,
                });
                total += 1;
            }

            match &mut energized[new_cell_index as usize] {
                Some(EnergizedCell { left, right, up, down }) => {
                    if *d_col == 1 {
                        if *right {
                            beams_to_remove.push(i).unwrap();
                            continue;
                        }
                        *right = true;
                    }
                    if *d_col == -1 {
                        if *left {
                            beams_to_remove.push(i).unwrap();
                            continue;
                        }
                        *left = true;
                    }
                    if *d_row == 1 {
                        if *down {
                            beams_to_remove.push(i).unwrap();
                            continue;
                        }
                        *down = true;
                    }
                    if *d_row == -1 {
                        if *up {
                            beams_to_remove.push(i).unwrap();
                            continue;
                        }
                        *up = true;
                    }
                }
                _ => panic!("Invalid cell"),
            }

            match grid[new_cell_index as usize] {
                b'.' => {}
                b'\\' => {
                    match (*d_row, *d_col) {
                        (0, -1) => {
                            *d_row = -1;
                            *d_col = 0;
                        }
                        (0, 1) => {
                            *d_row = 1;
                            *d_col = 0;
                        }
                        (-1, 0) => {
                            *d_row = 0;
                            *d_col = -1;
                        }
                        (1, 0) => {
                            *d_row = 0;
                            *d_col = 1;
                        }
                        _ => panic!("Invalid direction"),
                    }
                }
                b'/' => {
                    match (*d_row, *d_col) {
                        (0, -1) => {
                            *d_row = 1;
                            *d_col = 0;
                        }
                        (0, 1) => {
                            *d_row = -1;
                            *d_col = 0;
                        }
                        (-1, 0) => {
                            *d_row = 0;
                            *d_col = 1;
                        }
                        (1, 0) => {
                            *d_row = 0;
                            *d_col = -1;
                        }
                        _ => panic!("Invalid direction"),
                    }
                }
                b'-' => {
                    match (*d_row, *d_col) {
                        (0, -1) => {}
                        (0, 1) => {}
                        (-1, 0) | (1, 0) => {
                            beams_to_add.push(((*row, *col), (0, -1))).unwrap();
                            *d_row = 0;
                            *d_col = 1;
                        }
                        _ => panic!("Invalid direction"),
                    }
                }
                b'|' => {
                    match (*d_row, *d_col) {
                        (-1, 0) => {}
                        (1, 0) => {}
                        (0, -1) | (0, 1) => {
                            beams_to_add.push(((*row, *col), (-1, 0))).unwrap();
                            *d_row = 1;
                            *d_col = 0;
                        }
                        _ => panic!("Invalid direction"),
                    }
                }
                c => panic!("Invalid char {}", c as char)
            }
        }
        for i in beams_to_remove.iter().rev() {
            beams.swap_remove(*i);
        }
        for beam in beams_to_add.iter() {
            beams.push(*beam).unwrap();
        }
    }

    total
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j16.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let (grid, mut energized, width, height) = parse_grid(s);

    let mut max = 0;

    for row in 0..height {
        max = max.max(run_beam(&grid, &mut energized.clone(), width, height, ((row as isize, -1), (0, 1))));
        max = max.max(run_beam(&grid, &mut energized.clone(), width, height, ((row as isize, width as isize), (0, -1))));
    }
    for col in 0..width {
        max = max.max(run_beam(&grid, &mut energized.clone(), width, height, ((-1, col as isize), (1, 0))));
        max = max.max(run_beam(&grid, &mut energized.clone(), width, height, ((height as isize, col as isize), (-1, 0))));
    }
    max
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j16.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j16_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(46, _p1(include_str!("j16_test.txt")));
        assert_eq!(6622, _p1(include_str!("j16.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(51, _p2(include_str!("j16_test.txt")));
        assert_eq!(7130, _p2(include_str!("j16.txt")));
    }
}