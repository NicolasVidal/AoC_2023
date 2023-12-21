#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    const WIDTH: usize = 768;
    const HEIGHT: usize = 768;

    let mut grid = [b'.'; WIDTH * HEIGHT];
    let start = (WIDTH / 2, HEIGHT / 2);
    let mut current = start;
    for line in s.lines() {
        let mut split = line.split(' ');
        let mut dir = split.next().unwrap().bytes().next().unwrap();
        let mut steps = split.next().unwrap().parse::<usize>().unwrap();
        let color = split.next().unwrap();

        let (d_row, d_col) =
            match dir {
                b'U' => (-1, 0),
                b'D' => (1, 0),
                b'L' => (0, -1),
                b'R' => (0, 1),
                _ => panic!("Invalid dir"),
            };
        while steps > 0 {
            let mut new_row = current.0 as isize + d_row;
            let mut new_col = current.1 as isize + d_col;
            if new_row < 0 || new_row >= HEIGHT as isize || new_col < 0 || new_col >= WIDTH as isize {
                panic!("Invalid position");
            }
            grid[new_row as usize * WIDTH + new_col as usize] = dir;
            steps -= 1;
            current = (new_row as usize, new_col as usize);
        }
    }

    let mut start = (0, 0);

    let mut count = 0;
    'row: for row in 0..HEIGHT {
        'col: for col in 0..WIDTH {
            if grid[row * WIDTH + col] != b'.' {
                start = (row, col);
                break 'row;
            }
        }
    }

    let mut current = start;
    let mut prev_instruction = b' ';

    let mut count = 0;
    loop {
        let mut new_row = current.0 as isize;
        let mut new_col = current.1 as isize;

        let new_instruction = grid[current.0 * WIDTH + current.1];
        if new_instruction == b'U' || (prev_instruction == b'U' && new_instruction == b'R') {
            // count_points_right
            let mut new_col = current.1 as isize + 1;
            while grid[current.0 * WIDTH + new_col as usize] == b'.' {
                count += 1;
                new_col += 1;
            }
        }

        match new_instruction {
            b'U' => {
                new_row += 1;
            }
            b'D' => {
                new_row -= 1;
            }
            b'L' => {
                new_col += 1;
            }
            b'R' => {
                new_col -= 1;
            }
            _ => panic!("Invalid char"),
        }

        prev_instruction = new_instruction;
        current = (new_row as usize, new_col as usize);
        count += 1;
        if start == current {
            break;
        }
    }


    // 'row: for row in 0..HEIGHT {
    //     'col: for col in 0..WIDTH {
    //         print!("{}", grid[row * WIDTH + col] as char);
    //     }
    //     println!();
    // }
    count
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j18.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut directions = vec![];
    let mut walls = vec![];
    let mut current_cell = (0isize, 0isize);
    for line in s.lines() {
        let mut split = line.split(' ');
        let mut code = split.nth(2).unwrap();
        let mut code = &code[2..(code.len() - 1)];
        let mut distance = 0isize;
        for c in code.bytes().take(5) {
            distance *= 16;
            match c {
                b'0'..=b'9' => {
                    distance += (c - b'0') as isize;
                }
                b'a'..=b'z' => {
                    distance += (c - b'a' + 10) as isize;
                }
                _ => panic!("Invalid char"),
            }
        }
        let direction = code.bytes().nth(5).unwrap();
        let direction = match direction {
            b'0' => b'R',
            b'1' => b'D',
            b'2' => b'L',
            b'3' => b'U',
            _ => panic!("Invalid char"),
        };

        directions.push((direction, distance));
        let mut new_cell = current_cell;
        match direction {
            b'R' => {
                new_cell.1 += distance;
            }
            b'D' => {
                new_cell.0 += distance;
            }
            b'L' => {
                new_cell.1 -= distance;
            }
            b'U' => {
                new_cell.0 -= distance;
            }
            _ => panic!("Invalid char"),
        }

        walls.push((current_cell, new_cell));
        current_cell = new_cell;
    }

    let mut total_distance = 0isize;
    let mut prev_direction = b' ';
    for (direction, wall) in directions.iter().zip(walls.iter()) {
        if direction.0 == b'U' || (prev_direction == b'U' && direction.0 == b'R') {
            for row in wall.0.0.min(wall.1.0)..(wall.0.0.max(wall.1.0) + 1) {
                let mut col = wall.0.1.max(wall.1.1);
                let mut smallest_diff = isize::MAX;
                for other_wall in walls.iter() {
                    if other_wall == wall {
                        continue;
                    }

                    if (other_wall.0.0.min(other_wall.1.0)..=other_wall.0.0.max(other_wall.1.0)).contains(&row) &&
                        other_wall.0.1.min(other_wall.1.1) >= col {
                        let diff = other_wall.0.1.min(other_wall.1.1) - col;
                        if diff < smallest_diff {
                            smallest_diff = diff;
                        }
                    }
                }
                smallest_diff = smallest_diff.max(1);
                total_distance += smallest_diff - 1;
            }
        }
        prev_direction = direction.0;
        total_distance += (wall.1.1.abs_diff(wall.0.1).max(1) * wall.1.0.abs_diff(wall.0.0).max(1)) as isize;
    }

    total_distance as usize
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j18.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j18_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(62, _p1(include_str!("j18_test.txt")));
        assert_eq!(31171, _p1(include_str!("j18.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(952408144115, _p2(include_str!("j18_test.txt")));
        assert_eq!(131431655002266, _p2(include_str!("j18.txt")));
    }
}