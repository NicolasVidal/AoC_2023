use std::collections::HashMap;

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut lines = s.lines();
    let pattern = lines.next().unwrap();

    lines.next().unwrap();
    let mut map = HashMap::new();
    for line in lines {
        let mut splits = line.split(" = ");
        let key = splits.next().unwrap();
        let choices = splits.next().unwrap();
        let mut choices = choices.split(", ");
        let left = choices.next().unwrap()[1..].to_string();
        let right = choices.next().unwrap()[..3].to_string();

        map.insert(key, (left, right));
    }

    let mut node = map.get("AAA").unwrap();
    let mut counter = 0;
    loop {
        for (_, c) in pattern.chars().enumerate() {
            let next_node = match c {
                'L' => node.0.as_str(),
                'R' => node.1.as_str(),
                _ => panic!("Unknown char {}", c),
            };
            counter += 1;
            if next_node == "ZZZ" {
                return counter;
            }
            node = map.get(next_node).unwrap();
        }
    }
    panic!()
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j8.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut lines = s.lines();
    let pattern = lines.next().unwrap();

    let mut starts = vec![];
    lines.next().unwrap();
    let mut map = HashMap::new();
    for line in lines {
        let mut splits = line.split(" = ");
        let key = splits.next().unwrap();
        if key.ends_with('A') {
            starts.push(key);
        }
        let choices = splits.next().unwrap();
        let mut choices = choices.split(", ");
        let left = choices.next().unwrap()[1..].to_string();
        let right = choices.next().unwrap()[..3].to_string();

        map.insert(key, (left, right));
    }

    let mut nodes = starts.into_iter().map(|n| map.get(n).unwrap()).collect::<Vec<_>>();
    let mut next_nodes = vec![];
    let mut counter = 0;
    loop {
        for (_, c) in pattern.chars().enumerate() {
            next_nodes.clear();
            for node in nodes.iter() {
                let next_node = match c {
                    'L' => node.0.as_str(),
                    'R' => node.1.as_str(),
                    _ => panic!("Unknown char {}", c),
                };
                next_nodes.push(next_node);
            }
            counter += 1;

            nodes.clear();

            if next_nodes.iter().all(|n| n.ends_with('Z')) {
                return counter;
            }

            for next_node in next_nodes.iter() {
                nodes.push(map.get(next_node).unwrap());
            }
        }
    }
    panic!()
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j8.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j8_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(2, _p1(include_str!("j8_test.txt")));
        assert_eq!(15517, _p1(include_str!("j8.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(6, _p2(include_str!("j8_test_p2.txt")));
        assert_eq!(42, _p2(include_str!("j8.txt")));
    }
}