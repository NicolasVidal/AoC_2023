use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn test_disjoint(id0: (&str, &str), id1: (&str, &str), id2: (&str, &str), links: &HashMap<&str, HashSet<&str>>) -> bool {
    let mut links = links.clone();

    false
}

#[allow(unused)]
pub fn _p1(s: &str, test: bool) -> usize {
    let mut all_ids = HashSet::new();
    let mut compo = HashMap::new();
    for line in s.lines() {
        let mut split = line.split(": ");
        let id = split.next().unwrap();
        let mut split_links = split.next().unwrap().split(' ');
        let mut links = HashSet::new();
        while let Some(link) = split_links.next() {
            links.insert(link);
        }

        for link in links.iter() {
            let mut entry = compo.entry(*link).or_insert(HashSet::new());
            entry.insert(id);
        }
        let mut entry = compo.entry(id).or_insert(HashSet::new());
        entry.extend(links);
        all_ids.insert(id);
    }



    let mut all_pairs = HashSet::new();

    for elem in compo.iter() {
        for link in elem.1 {
            if all_pairs.contains(&(*elem.0, *link)) || all_pairs.contains(&(*link, *elem.0)) {
                continue;
            }
            all_pairs.insert((*elem.0, *link));
        }
    }

    dbg!(&all_pairs.len());

    println!("{}", all_pairs.iter().map(|(id0, id1)| {
        format!("{0}\\[UndirectedEdge]{1}", id0, id1)
    }).join(","));

    // Plotting the graph in Mathematica gives :
    // "qdv" - "zhg"
    // "lsk" - "rfq"
    // "gpz" - "prk"

    let pairs_to_remove = if test {
        [
            ("hfx", "pzl"),
            ("bvb", "cmg"),
            ("nvd", "jqt"),
        ]
    } else {[
        ("qdv", "zhg"),
        ("lsk", "rfq"),
        ("gpz", "prk"),
    ]};

    let mut link_removed = 0;
    for (key, val) in compo.iter_mut() {
        for (p1, p2) in pairs_to_remove.iter() {
            if key == p1 {
                val.remove(p2);
            }
            if key == p2 {
                val.remove(p1);
            }
        }
    }

    let mut all_pairs = HashSet::new();

    for elem in compo.iter() {
        for link in elem.1 {
            if all_pairs.contains(&(*elem.0, *link)) || all_pairs.contains(&(*link, *elem.0)) {
                continue;
            }
            all_pairs.insert((*elem.0, *link));
        }
    }

    dbg!(&all_pairs.len());
    println!("{}", all_pairs.iter().map(|(id0, id1)| {
        format!("{0}\\[UndirectedEdge]{1}", id0, id1)
    }).join(","));

    let mut left_set_count = HashSet::new();
    let mut elem_to_add = Vec::new();
    elem_to_add.push(pairs_to_remove[0].0);

    while let Some(elem) = elem_to_add.pop() {
        if !left_set_count.insert(elem) {
            continue;
        }
        for link in compo.get(elem).unwrap() {
            elem_to_add.push(link);
        }
    }

    let left_number = left_set_count.len();

    let mut left_set_count = HashSet::new();
    let mut elem_to_add = Vec::new();
    elem_to_add.push(pairs_to_remove[0].1);

    while let Some(elem) = elem_to_add.pop() {
        if !left_set_count.insert(elem) {
            continue;
        }
        for link in compo.get(elem).unwrap() {
            elem_to_add.push(link);
        }
    }

    let right_number = left_set_count.len();

    // for (id0, &(p0, p1)) in all_pairs.iter().enumerate() {
    //     for (id1, &(p2, p3)) in all_pairs.iter().enumerate().skip(id0 + 1) {
    //         for (id2, &(p4, p5)) in all_pairs.iter().enumerate().skip(id1 + 1) {
    //             if test_disjoint((p0, p1), (p2, p3), (p4, p5), &compo) {
    //                 return id0 * id1 * id2;
    //             }
    //         }
    //     }
    // }

    left_number * right_number
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j25.txt"), false)
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    for line in s.lines() {}
    42
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j25.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j25_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(54, _p1(include_str!("j25_test.txt"), true));
        assert_eq!(555702, _p1(include_str!("j25.txt"), false));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(42, _p2(include_str!("j25_test.txt")));
        assert_eq!(42, _p2(include_str!("j25.txt")));
    }
}