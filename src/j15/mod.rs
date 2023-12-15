fn hash(s: &str) -> u8 {
    let mut h = 0usize;
    for c in s.bytes() {
        h += c as usize;
        h *= 17;
        h %= 256;
    }
    h as u8
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut total = 0;
    for line in s.lines() {
        for hash in line.split(',').map(hash) {
            total += hash as usize;
        }
    }
    total
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j15.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut boxes = [(); 256].map(|_| heapless::Vec::<_, 8>::new());
    let mut max = 0;

    for line in s.lines() {
        for instruction in line.split(',') {
            let last_two_chars = instruction[instruction.len() - 2..].as_bytes();
            match last_two_chars {
                [b'=', n] if n.is_ascii_digit() => {
                    let label = &instruction[..instruction.len() - 2];
                    let num = n - b'0';
                    if let Some(pos) = boxes[hash(label) as usize].iter().position(|(l, _)| *l == label) {
                        boxes[hash(label) as usize][pos].1 = num;
                    } else {
                        boxes[hash(label) as usize].push((label, num));
                    }
                }
                [_, b'-'] => {
                    let label = &instruction[..instruction.len() - 1];
                    if let Some(pos) = boxes[hash(label) as usize].iter().position(|(l, _)| *l == label) {
                        boxes[hash(label) as usize].remove(pos);
                    }
                }
                _ => panic!(),
            }
        }
    }
    let mut total = 0;
    for (box_id, b) in boxes.iter().enumerate() {
        max = max.max(b.len());
        total += b.iter().enumerate().map(|(i, (_, n))|
            (box_id + 1) * (i + 1) * (*n as usize)).sum::<usize>();
    }
    total
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j15.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j15_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(1320, _p1(include_str!("j15_test.txt")));
        assert_eq!(514394, _p1(include_str!("j15.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(145, _p2(include_str!("j15_test.txt")));
        assert_eq!(236358, _p2(include_str!("j15.txt")));
    }
}