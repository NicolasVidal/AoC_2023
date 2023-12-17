use std::collections::{HashMap, HashSet};

type Cell = (isize, isize, isize, isize, isize, isize);

fn neighbours_p1(cell: Cell, height: usize, width: usize) -> Vec<Cell> {
    let (row, col, left, right, up, down) = cell;
    let mut neighbours = Vec::new();
    if row > 0 && up < 3 && down == 0 {
        neighbours.push((row - 1, col, 0, 0, up + 1, 0));
    }
    if col > 0 && left < 3 && right == 0 {
        neighbours.push((row, col - 1, left + 1, 0, 0, 0));
    }
    if row < height as isize - 1 && down < 3 && up == 0 {
        neighbours.push((row + 1, col, 0, 0, 0, down + 1));
    }
    if col < width as isize - 1 && right < 3 && left == 0 {
        neighbours.push((row, col + 1, 0, right + 1, 0, 0));
    }

    neighbours
}


fn neighbours_p2(cell: Cell, height: usize, width: usize) -> Vec<Cell> {
    let (row, col, left, right, up, down) = cell;
    let mut neighbours = Vec::new();
    if row > 0 && up < 10 && down == 0 && (left == 0 || left >= 4) && (right == 0 || right >= 4) {
        neighbours.push((row - 1, col, 0, 0, up + 1, 0));
    }
    if col > 0 && left < 10 && right == 0 && (up == 0 || up >= 4) && (down == 0 || down >= 4) {
        neighbours.push((row, col - 1, left + 1, 0, 0, 0));
    }
    if row < height as isize - 1 && down < 10 && up == 0 && (left == 0 || left >= 4) && (right == 0 || right >= 4) {
        neighbours.push((row + 1, col, 0, 0, 0, down + 1));
    }
    if col < width as isize - 1 && right < 10 && left == 0 && (up == 0 || up >= 4) && (down == 0 || down >= 4) {
        neighbours.push((row, col + 1, 0, right + 1, 0, 0));
    }

    neighbours
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    find_min_cost_path(s, neighbours_p1)
}

fn find_min_cost_path(s: &str, neighbours_func: fn(Cell, height: usize, width: usize) -> Vec<Cell>) -> usize {
    let mut costs = Vec::new();
    let width = s.lines().next().unwrap().len();
    for line in s.lines() {
        for c in line.chars() {
            costs.push(c.to_digit(10).unwrap() as isize);
        }
    }
    let height = costs.len() / width;

    let mut explored_cells = HashSet::new();
    let mut graded_cells = HashMap::new();

    let target = (height as isize - 1, width as isize - 1);
    let mut current = (0, 0, 0, 0, 0, 0);
    let mut froms = HashMap::new();

    graded_cells.insert(current, 0);

    loop {
        let mut min_cost = isize::MAX;
        let mut min_cell = (0isize, 0isize, 0, 0, 0, 0);
        for (cell, cost) in &graded_cells {
            if *cost < min_cost {
                min_cost = *cost;
                min_cell = *cell;
            }
        }
        current = min_cell;
        if current.0 == target.0 && current.1 == target.1 {
            let mut optimal_path = vec!();
            loop {
                optimal_path.push(current);
                let from: &Cell = froms.get(&current).unwrap();
                if from.0 == 0 && from.1 == 0 && from.2 == 0 && from.3 == 0 && from.4 == 0 && from.5 == 0 {
                    break;
                }
                current = *from;
            }

            let mut grid = vec![vec!['.'; width]; height];

            for cell in optimal_path.iter().rev() {
                // println!("{},{}", cell.0, cell.1);
                grid[cell.0 as usize][cell.1 as usize] = match cell {
                    (0, 0, 0, 0, 0, 0) => 'S',
                    (_, _, i, 0, 0, 0) if *i > 0 => '<',
                    (_, _, 0, i, 0, 0) if *i > 0 => '>',
                    (_, _, 0, 0, i, 0) if *i > 0 => '^',
                    (_, _, 0, 0, 0, i) if *i > 0 => 'v',
                    _ => panic!("unexpected cell"),
                };
            }

            // for line in grid {
            //     println!("{}", line.iter().collect::<String>());
            // }

            return min_cost as usize;
        }
        let neighbours = neighbours_func(current, height, width);
        for neighbour in &neighbours {
            if !explored_cells.contains(neighbour) {
                let cost = costs[(neighbour.0 * width as isize + neighbour.1) as usize];
                let new_cost = cost + graded_cells.get(&current).unwrap();
                let new_cost_heuristic = new_cost + (target.0 - neighbour.0).abs() + (target.1 - neighbour.1).abs();
                if !graded_cells.contains_key(neighbour) || new_cost_heuristic < *graded_cells.get(neighbour).unwrap() {
                    froms.insert(*neighbour, current);
                    graded_cells.insert(*neighbour, new_cost);
                }
            }
        }
        explored_cells.insert(current);
        graded_cells.remove(&current);
    }
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j17.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    find_min_cost_path(s, neighbours_p2)
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j17.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j17_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(102, _p1(include_str!("j17_test.txt")));
        assert_eq!(843, _p1(include_str!("j17.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(94, _p2(include_str!("j17_test.txt")));
        assert_eq!(1017, _p2(include_str!("j17.txt")));
    }
}