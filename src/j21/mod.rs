
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

    for row in 0..rows {
        for col in 0..cols {
            if current_pos[row * cols + col] == b'O' {
                grid[row * cols + col] = b'O';
            }
            print!("{}", grid[row * cols + col] as char);
        }
        println!();
    }
    println!();

    current_pos.iter().filter(|c| **c == b'O').count()
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j21.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    for line in s.lines() {

    }
    42
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
        assert_eq!(42, _p1(include_str!("j21_test.txt")));
        assert_eq!(3617, _p1(include_str!("j21.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(42, _p2(include_str!("j21_test.txt")));
        assert_eq!(42, _p2(include_str!("j21.txt")));
    }
}