#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut grid = heapless::Vec::<heapless::Vec<char, 256>, 256>::new();
    let mut grid_numbers = heapless::Vec::<heapless::Vec<bool, 256>, 256>::new();

    let mut start = None;

    for (row, line) in s.lines().enumerate() {
        let mut grid_row = heapless::Vec::<char, 256>::new();
        let mut grid_row_numbers = heapless::Vec::<bool, 256>::new();
        for (col, c) in line.chars().enumerate() {
            grid_row.push(c).unwrap();
            grid_row_numbers.push(false).unwrap();
            if c == 'S' {
                start = Some((row, col));
            }
        }
        grid.push(grid_row).unwrap();
        grid_numbers.push(grid_row_numbers).unwrap();
    }

    let start = start.unwrap();

    let mut start_possibilities = heapless::FnvIndexSet::<char, 8>::new();
    start_possibilities.insert('|').unwrap();
    start_possibilities.insert('-').unwrap();
    start_possibilities.insert('L').unwrap();
    start_possibilities.insert('J').unwrap();
    start_possibilities.insert('7').unwrap();
    start_possibilities.insert('F').unwrap();

    if start.1 > 0 && ['-', 'L', 'F'].contains(&grid[start.0][start.1 - 1]) {} else {
        start_possibilities.remove(&'-');
        start_possibilities.remove(&'7');
        start_possibilities.remove(&'J');
    }

    if start.0 > 0 && ['|', '7', 'F'].contains(&grid[start.0 - 1][start.1]) {} else {
        start_possibilities.remove(&'|');
        start_possibilities.remove(&'J');
        start_possibilities.remove(&'L');
    }

    if start.1 < grid[start.0].len() - 1 && ['-', 'J', '7'].contains(&grid[start.0][start.1 + 1]) {} else {
        start_possibilities.remove(&'-');
        start_possibilities.remove(&'L');
        start_possibilities.remove(&'F');
    }

    if start.0 < grid.len() - 1 && ['|', 'J', 'L'].contains(&grid[start.0 + 1][start.1]) {} else {
        start_possibilities.remove(&'|');
        start_possibilities.remove(&'7');
        start_possibilities.remove(&'F');
    }

    let start_possibility = start_possibilities.iter().next().unwrap();

    grid[start.0][start.1] = *start_possibility;
    grid_numbers[start.0][start.1] = true;


    let mut frontier = heapless::Vec::<(usize, usize, usize), 2>::new();
    let mut next_frontier = heapless::Vec::<(usize, usize, usize), 4>::new();
    frontier.push((start.0, start.1, 0)).unwrap();

    loop {
        next_frontier.clear();
        let mut max_distance = usize::MIN;
        for &(row, col, distance) in frontier.iter() {
            if distance > max_distance {
                max_distance = distance;
            }
            match grid[row][col] {
                '|' => {
                    next_frontier.push((row - 1, col, distance + 1));
                    next_frontier.push((row + 1, col, distance + 1));
                }
                '-' => {
                    next_frontier.push((row, col - 1, distance + 1));
                    next_frontier.push((row, col + 1, distance + 1));
                }
                'L' => {
                    next_frontier.push((row - 1, col, distance + 1));
                    next_frontier.push((row, col + 1, distance + 1));
                }
                'J' => {
                    next_frontier.push((row - 1, col, distance + 1));
                    next_frontier.push((row, col - 1, distance + 1));
                }
                '7' => {
                    next_frontier.push((row + 1, col, distance + 1));
                    next_frontier.push((row, col - 1, distance + 1));
                }
                'F' => {
                    next_frontier.push((row + 1, col, distance + 1));
                    next_frontier.push((row, col + 1, distance + 1));
                }
                _ => panic!("Unknown char {}", grid[row][col])
            }
        }

        next_frontier.retain(|&(row, col, _)| !grid_numbers[row][col]);

        if next_frontier.len() == 0 {
            return max_distance;
        }

        frontier.clear();
        for elt in next_frontier.iter() {
            grid_numbers[elt.0][elt.1] = true;
            frontier.push(*elt).unwrap();
        }
    }
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j10.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut grid = heapless::Vec::<heapless::Vec<char, 256>, 256>::new();
    let mut grid_numbers = heapless::Vec::<heapless::Vec<bool, 256>, 256>::new();

    let mut start = None;

    for (row, line) in s.lines().enumerate() {
        let mut grid_row = heapless::Vec::<char, 256>::new();
        let mut grid_row_numbers = heapless::Vec::<bool, 256>::new();
        for (col, c) in line.chars().enumerate() {
            grid_row.push(c).unwrap();
            grid_row_numbers.push(false).unwrap();
            if c == 'S' {
                start = Some((row, col));
            }
        }
        grid.push(grid_row).unwrap();
        grid_numbers.push(grid_row_numbers).unwrap();
    }

    let start = start.unwrap();

    let mut start_possibilities = heapless::FnvIndexSet::<char, 8>::new();
    start_possibilities.insert('|').unwrap();
    start_possibilities.insert('-').unwrap();
    start_possibilities.insert('L').unwrap();
    start_possibilities.insert('J').unwrap();
    start_possibilities.insert('7').unwrap();
    start_possibilities.insert('F').unwrap();

    if start.1 > 0 && ['-', 'L', 'F'].contains(&grid[start.0][start.1 - 1]) {} else {
        start_possibilities.remove(&'-');
        start_possibilities.remove(&'7');
        start_possibilities.remove(&'J');
    }

    if start.0 > 0 && ['|', '7', 'F'].contains(&grid[start.0 - 1][start.1]) {} else {
        start_possibilities.remove(&'|');
        start_possibilities.remove(&'J');
        start_possibilities.remove(&'L');
    }

    if start.1 < grid[start.0].len() - 1 && ['-', 'J', '7'].contains(&grid[start.0][start.1 + 1]) {} else {
        start_possibilities.remove(&'-');
        start_possibilities.remove(&'L');
        start_possibilities.remove(&'F');
    }

    if start.0 < grid.len() - 1 && ['|', 'J', 'L'].contains(&grid[start.0 + 1][start.1]) {} else {
        start_possibilities.remove(&'|');
        start_possibilities.remove(&'7');
        start_possibilities.remove(&'F');
    }

    let start_possibility = start_possibilities.iter().next().unwrap();

    grid[start.0][start.1] = *start_possibility;
    grid_numbers[start.0][start.1] = true;


    let mut frontier = heapless::Vec::<(usize, usize, usize), 2>::new();
    let mut next_frontier = heapless::Vec::<(usize, usize, usize), 4>::new();
    frontier.push((start.0, start.1, 0)).unwrap();

    loop {
        next_frontier.clear();
        let mut max_distance = usize::MIN;
        for &(row, col, distance) in frontier.iter() {
            if distance > max_distance {
                max_distance = distance;
            }
            match grid[row][col] {
                '|' => {
                    next_frontier.push((row - 1, col, distance + 1));
                    next_frontier.push((row + 1, col, distance + 1));
                }
                '-' => {
                    next_frontier.push((row, col - 1, distance + 1));
                    next_frontier.push((row, col + 1, distance + 1));
                }
                'L' => {
                    next_frontier.push((row - 1, col, distance + 1));
                    next_frontier.push((row, col + 1, distance + 1));
                }
                'J' => {
                    next_frontier.push((row - 1, col, distance + 1));
                    next_frontier.push((row, col - 1, distance + 1));
                }
                '7' => {
                    next_frontier.push((row + 1, col, distance + 1));
                    next_frontier.push((row, col - 1, distance + 1));
                }
                'F' => {
                    next_frontier.push((row + 1, col, distance + 1));
                    next_frontier.push((row, col + 1, distance + 1));
                }
                _ => panic!("Unknown char {}", grid[row][col])
            }
        }

        next_frontier.retain(|&(row, col, _)| !grid_numbers[row][col]);

        if next_frontier.len() == 0 {
            break;
        }

        frontier.clear();
        for elt in next_frontier.iter() {
            grid_numbers[elt.0][elt.1] = true;
            frontier.push(*elt).unwrap();
        }
    }

    let mut total_inside = 0;

    let mut previous_loop = 0;
    let mut inside = false;

    let mut border = None;
    'outer: for row in 0..grid_numbers.len() {
        for col in 0..grid_numbers[row].len() {
            if grid_numbers[row][col] {
                border = Some((row, col));
                break 'outer;
            }
        }
    }

    let border = border.unwrap();

    let mut cell = border;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let mut direction = Direction::Up;

    let mut total_inside = 0;

    loop {
        let (next_cell, next_direction, search_downwards) = match (direction, grid[cell.0][cell.1]) {
            (Direction::Up, '|') => ((cell.0 - 1, cell.1), Direction::Up, false),
            (Direction::Down, '|') => ((cell.0 + 1, cell.1), Direction::Down, false),
            (Direction::Left, '-') => ((cell.0, cell.1 - 1), Direction::Left, false),
            (Direction::Right, '-') => ((cell.0, cell.1 + 1), Direction::Right, true),
            (Direction::Left, 'L') => ((cell.0 - 1, cell.1), Direction::Up, false),
            (Direction::Down, 'L') => ((cell.0, cell.1 + 1), Direction::Right, true),
            (Direction::Right, 'J') => ((cell.0 - 1, cell.1), Direction::Up, true),
            (Direction::Down, 'J') => ((cell.0, cell.1 - 1), Direction::Left, false),
            (Direction::Right, '7') => ((cell.0 + 1, cell.1), Direction::Down, false),
            (Direction::Up, '7') => ((cell.0, cell.1 - 1), Direction::Left, false),
            (Direction::Left, 'F') => ((cell.0 + 1, cell.1), Direction::Down, false),
            (Direction::Up, 'F') => ((cell.0, cell.1 + 1), Direction::Right, false),
            _ => panic!("Unknown char {}", grid[cell.0][cell.1])
        };

        if search_downwards {
            let mut down_cell = (cell.0 + 1, cell.1);
            while down_cell.0 < grid.len() && !grid_numbers[down_cell.0][down_cell.1] {
                down_cell = (down_cell.0 + 1, down_cell.1);
                total_inside += 1;
            }
        }

        if next_cell == border {
            break;
        }
        cell = next_cell;
        direction = next_direction;
    }


    total_inside
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j10.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j10_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(8, _p1(include_str!("j10_test.txt")));
        assert_eq!(6846, _p1(include_str!("j10.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(4, _p2(include_str!("j10_test_p2_1.txt")));
        assert_eq!(8, _p2(include_str!("j10_test_p2_2.txt")));
        assert_eq!(10, _p2(include_str!("j10_test_p2_3.txt")));
        assert_eq!(325, _p2(include_str!("j10.txt")));
    }
}