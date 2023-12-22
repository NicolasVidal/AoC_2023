use std::cmp::Reverse;
use std::collections::HashSet;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut bricks = Vec::new();

    let mut min = (0, 0, 0);
    let mut max = (0, 0, 0);

    for (i, line) in s.lines().enumerate() {
        let mut split = line.split('~');
        let mut start_split = split.next().unwrap().split(',');
        let mut end_split = split.next().unwrap().split(',');
        let mut start = (start_split.next().unwrap().parse::<usize>().unwrap(),
                         start_split.next().unwrap().parse::<usize>().unwrap(),
                         start_split.next().unwrap().parse::<usize>().unwrap());
        let mut end = (end_split.next().unwrap().parse::<usize>().unwrap(),
                       end_split.next().unwrap().parse::<usize>().unwrap(),
                       end_split.next().unwrap().parse::<usize>().unwrap()
        );
        let real_start = (
            start.0.min(end.0),
            start.1.min(end.1),
            start.2.min(end.2),
        );
        let real_end = (
            start.0.max(end.0),
            start.1.max(end.1),
            start.2.max(end.2),
        );

        min = (
            min.0.min(real_start.0),
            min.1.min(real_start.1),
            min.2.min(real_start.2),
        );

        max = (
            max.0.max(real_end.0),
            max.1.max(real_end.1),
            max.2.max(real_end.2),
        );

        bricks.push((i, real_start, real_end));
    }

    let mut z_lowest_offsets = Vec::new();

    for x in min.0..=max.0 {
        let mut y_lowest_offsets = Vec::new();
        for y in min.1..=max.1 {
            y_lowest_offsets.push((usize::MAX, 0))
        }
        z_lowest_offsets.push(y_lowest_offsets);
    }

    bricks.sort_unstable_by_key(|(_, (_, _, z), (_, _, _))| Reverse(*z));

    let mut total_bricks = bricks.clone();
    let mut supporting_brick = HashSet::new();

    loop {
        // for x in min.0..=max.0 {
        //     for y in min.1..=max.1 {
        //         print!("{}", z_lowest_offsets[x][y].0);
        //     }
        //     println!();
        // }
        // println!();
        //
        //
        // for x in min.0..=max.0 {
        //     for y in min.1..=max.1 {
        //         print!("{}", z_lowest_offsets[x][y].1);
        //     }
        //     println!();
        // }
        // println!();

        if bricks.len() == 0 {
            break;
        }

        let mut lowest_offset = i32::MAX;
        let mut support_position = (0, 0);
        let mut supports = Vec::new();
        let mut base_z = i32::MAX;

        let brick = bricks.pop().unwrap();
        let mut offset = i32::MAX;
        for x in brick.1.0..=brick.2.0 {
            for y in brick.1.1..=brick.2.1 {
                offset = brick.1.2 as i32 - z_lowest_offsets[x][y].1;

                assert!(offset >= 0);
                if offset < lowest_offset {
                    supports.clear();
                    lowest_offset = offset;
                    support_position = (x, y);
                    base_z = z_lowest_offsets[x][y].1;
                }

                if lowest_offset == offset {
                    supports.push(z_lowest_offsets[x][y].0);
                }
            }
        }

        for b in supports {
            supporting_brick.insert((b, brick.0));
        }

        let new_z = (brick.2.2 as i32 - brick.1.2 as i32) + 1 + z_lowest_offsets[support_position.0][support_position.1].1;

        for x in brick.1.0..=brick.2.0 {
            for y in brick.1.1..=brick.2.1 {
                z_lowest_offsets[x][y].0 = brick.0;
                z_lowest_offsets[x][y].1 = new_z;
            }
        }
    }

    let mut can_be_removed = 0usize;

    'looping: for brick in total_bricks {
        for (_, b) in supporting_brick.iter().filter(|(a, _)| *a == brick.0) {
            if supporting_brick.iter().filter(|(_, c)| *b == *c).count() == 1 {
                continue 'looping;
            }
        }

        can_be_removed += 1;
    }


    // 'b1: for b1 in bricks.iter() {
    //     'b2: for b2 in bricks.iter() {
    //         if b1.0 == b2.0 {
    //             continue;
    //         }
    //
    //         for support in supporting_brick.iter() {
    //             if support.0 != b1.0 && support.1 == b2.0 {
    //                 continue 'b1;
    //             }
    //         }
    //     }
    //     can_be_removed += 1;
    // }

    // dbg!(z_lowest_offsets);

    can_be_removed
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j22.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    for line in s.lines() {}
    42
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j22.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j22_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(5, _p1(include_str!("j22_test.txt")));
        assert_eq!(471, _p1(include_str!("j22.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(42, _p2(include_str!("j22_test.txt")));
        assert_eq!(42, _p2(include_str!("j22.txt")));
    }
}