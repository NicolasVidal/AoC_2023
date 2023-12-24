use std::collections::HashMap;
use std::fmt::Display;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut grid = s.lines().map(|l| l.bytes().collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();

    grid[0][1] = b'#';

    let mut zones_to_explore = Vec::<((usize, usize), usize)>::new();
    zones_to_explore.push(((1, 1), 1));

    let mut max_end = 0;

    let edges = Vec::<((usize, usize), (usize, usize), usize)>::new();

    while let Some(((mut row, mut col), mut cost)) = zones_to_explore.pop() {
        let (mut prev_row, mut prev_col) = (row, col);
        loop {
            let mut empty_found = false;
            let (old_row, old_col) = (row, col);

            if row == grid.len() - 1 && col == grid[0].len() - 2 {
                max_end = max_end.max(cost);
                break;
            }

            if grid[row - 1][col] == b'.' && (row - 1, col) != (prev_row, prev_col) {
                row -= 1;
                empty_found = true;
            } else if grid[row + 1][col] == b'.' && (row + 1, col) != (prev_row, prev_col) {
                row += 1;
                empty_found = true;
            } else if grid[row][col - 1] == b'.' && (row, col - 1) != (prev_row, prev_col) {
                col -= 1;
                empty_found = true;
            } else if grid[row][col + 1] == b'.' && (row, col + 1) != (prev_row, prev_col) {
                col += 1;
                empty_found = true;
            }

            let mut found = false;
            if !empty_found && grid[row - 1][col] == b'^' && (row - 1, col) != (prev_row, prev_col) {
                zones_to_explore.push(((row - 2, col), cost + 2));
                found = true;
            }
            if !empty_found && grid[row + 1][col] == b'v' && (row + 1, col) != (prev_row, prev_col) {
                zones_to_explore.push(((row + 2, col), cost + 2));
                found = true;
            }
            if !empty_found && grid[row][col - 1] == b'<' && (row, col - 1) != (prev_row, prev_col) {
                zones_to_explore.push(((row, col - 2), cost + 2));
                found = true;
            }
            if !empty_found && grid[row][col + 1] == b'>' && (row, col + 1) != (prev_row, prev_col) {
                zones_to_explore.push(((row, col + 2), cost + 2));
                found = true;
            }

            if found {
                break;
            }

            (prev_row, prev_col) = (old_row, old_col);
            assert_eq!(empty_found, true);

            cost += 1;
        }
    }

    max_end
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j23.txt"))
}

#[derive(Clone, Debug)]
struct Node {
    row: usize,
    col: usize,
    id: usize,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: ({}, {})", self.id, self.row, self.col)
    }
}

#[derive(Clone, Debug)]
struct Edge {
    from: usize,
    to: usize,
    cost: usize,
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut grid = s.lines().map(|l| l.bytes().collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();

    grid[0][1] = b'v';

    let rows = grid.len();
    let cols = grid[0].len();

    grid[rows - 1][cols - 2] = b'v';

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    nodes.push(Node { row: 0, col: 1, id: 0 });
    nodes.push(Node { row: rows - 1, col: cols - 2, id: 1 });

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == b'#' {
                continue;
            }
            let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

            let mut intersections = 0;
            for dir in dirs {
                let new_row = row as i32 + dir.0;
                let new_col = col as i32 + dir.1;

                if new_row < 0 || new_row >= rows as i32 || new_col < 0 || new_col >= cols as i32 {
                    continue;
                }

                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if grid[new_row][new_col] == b'#' {
                    continue;
                }

                intersections += 1;
            }

            if intersections > 2 {
                nodes.push(Node { row, col, id: nodes.len() });
            }
        }
    }

    for node in nodes.iter() {
        let mut grid = grid.clone();
        let mut cells_to_expand_from = Vec::new();
        cells_to_expand_from.push((node.row, node.col, 0));

        while let Some((row, col, mut cost)) = cells_to_expand_from.pop() {
            cost += 1;
            grid[row][col] = b'#';

            let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

            for dir in dirs {
                let new_row = row as i32 + dir.0;
                let new_col = col as i32 + dir.1;

                if new_row < 0 || new_row >= rows as i32 || new_col < 0 || new_col >= cols as i32 {
                    continue;
                }

                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if grid[new_row][new_col] == b'#' {
                    continue;
                }

                if let Some(new_node) = nodes.iter().find(|n| n.row == new_row && n.col == new_col) {
                    edges.push(Edge { from: node.id, to: new_node.id, cost });
                    continue;
                }

                if grid[new_row][new_col] != b'#' {
                    cells_to_expand_from.push((
                        new_row,
                        new_col,
                        cost)
                    );
                }
            }
        }
    }

    let mut neighbours = HashMap::new();

    for edge in edges.iter() {
        neighbours.entry(edge.from).or_insert(Vec::new()).push((edge.to, edge.cost));
    }


    let start = nodes.iter().find(|n| n.row == 0 && n.col == 1).unwrap().id;
    let goal = nodes.iter().find(|n| n.row == rows - 1 && n.col == cols - 2).unwrap().id;

    let mut to_explore = vec![(start, 0, Vec::new())];

    let mut max = 0;
    let mut best = None;
    while let Some((node, cost, mut explored)) = to_explore.pop() {
        if node == goal {
            if cost > max {
                max = cost;
                best = Some(explored);
            }
            continue;
        }

        explored.push(node);

        if let Some(neighbours) = neighbours.get(&node) {
            for (neighbour, neighbour_cost) in neighbours.iter() {
                if explored.contains(neighbour) {
                    continue;
                }

                let explored = explored.clone();
                to_explore.push((*neighbour, cost + neighbour_cost, explored));
            }
        } else {
            panic!("no neighbours for node {}", node);
        }
    }

    max
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j23.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j23_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(94, _p1(include_str!("j23_test.txt")));
        assert_eq!(1930, _p1(include_str!("j23.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(154, _p2(include_str!("j23_test.txt")));
        assert_eq!(6230, _p2(include_str!("j23.txt")));
    }
}