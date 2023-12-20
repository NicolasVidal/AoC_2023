#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    const WIDTH: usize = 1200;
    const HEIGHT: usize = 1200;

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
    for line in s.lines() {
        let mut split = line.split(' ');
        let mut code = split.nth(2).unwrap().parse::<usize>().unwrap();
    }
    42
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
        assert_eq!(42, _p2(include_str!("j18.txt")));
    }
}