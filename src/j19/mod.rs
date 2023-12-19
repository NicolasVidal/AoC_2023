use std::str::Lines;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Symbol {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum RuleType {
    Greater(Symbol, usize),
    Less(Symbol, usize),
    AlwaysTrue,
}

#[derive(Debug)]
struct Rule<'a> {
    target: &'a str,
    rule: RuleType,
}

#[derive(Debug)]
struct WorkFlow<'a> {
    _id: &'a str,
    rules: heapless::Vec<Rule<'a>, 10>,
}

#[derive(Debug, Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}


#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut accepted_count = 0usize;

    let mut lines = s.lines();

    let workflows = parse_workflows(&mut lines);

    // parse parts
    let mut parts = heapless::Vec::<Part, 1024>::new();

    for line in lines {
        let line = line[1..(line.len() - 1)].trim();
        let mut split = line.split(",");
        let mut part = Part {
            x: split.next().unwrap()[2..].parse::<usize>().unwrap(),
            m: split.next().unwrap()[2..].parse::<usize>().unwrap(),
            a: split.next().unwrap()[2..].parse::<usize>().unwrap(),
            s: split.next().unwrap()[2..].parse::<usize>().unwrap(),
        };
        parts.push(part).unwrap();
    }

    'parts: for part in parts {
        let mut current_id = "in";
        'start: loop {
            match current_id {
                "A" => {
                    accepted_count += part.x + part.m + part.a + part.s;
                    continue 'parts;
                }
                "R" => {
                    continue 'parts;
                }
                _ => {}
            }
            let workflow = workflows.get(current_id).unwrap();
            for rule in workflow.rules.iter() {
                match &rule.rule {
                    RuleType::Greater(symbol, value) => {
                        match symbol {
                            Symbol::X => {
                                if part.x > *value {
                                    current_id = rule.target;
                                    continue 'start;
                                }
                            }
                            Symbol::M => {
                                if part.m > *value {
                                    current_id = rule.target;
                                    continue 'start;
                                }
                            }
                            Symbol::A => {
                                if part.a > *value {
                                    current_id = rule.target;
                                    continue 'start;
                                }
                            }
                            Symbol::S => {
                                if part.s > *value {
                                    current_id = rule.target;
                                    continue 'start;
                                }
                            }
                        }
                    }
                    RuleType::Less(symbol, value) => {
                        match symbol {
                            Symbol::X => {
                                if part.x < *value {
                                    current_id = rule.target;
                                    continue 'start;
                                }
                            }
                            Symbol::M => {
                                if part.m < *value {
                                    current_id = rule.target;
                                    continue 'start;
                                }
                            }
                            Symbol::A => {
                                if part.a < *value {
                                    current_id = rule.target;
                                    continue 'start;
                                }
                            }
                            Symbol::S => {
                                if part.s < *value {
                                    current_id = rule.target;
                                    continue 'start;
                                }
                            }
                        }
                    }
                    RuleType::AlwaysTrue => {
                        current_id = rule.target;
                        continue 'start;
                    }
                }
            }
        }
    }

    accepted_count
}

fn parse_workflows<'a, 'b>(lines: &'b mut Lines<'a>) -> heapless::FnvIndexMap<&'a str, WorkFlow<'a>, 1024> {
    let mut workflows = heapless::FnvIndexMap::<&str, WorkFlow, 1024>::new();
    // parse workflows
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        let mut split = line.split("{");
        let id = split.next().unwrap();
        let rules = split.next().unwrap();
        let rules = rules.trim_end_matches("}").split(",");
        let mut rules_vec = heapless::Vec::<Rule, 10>::new();
        for rule in rules {
            let mut rule = rule.split(":");

            match (rule.next(), rule.next()) {
                (Some(target), None) => {
                    rules_vec.push(Rule {
                        target,
                        rule: RuleType::AlwaysTrue,
                    }).unwrap();
                }
                (Some(condition), Some(target)) => {
                    let symbol = condition.bytes().next().unwrap();
                    let symbol = match symbol {
                        b'x' => Symbol::X,
                        b'm' => Symbol::M,
                        b'a' => Symbol::A,
                        b's' => Symbol::S,
                        _ => panic!("Invalid symbol"),
                    };

                    let amount = condition[2..].parse::<usize>().unwrap();

                    let rule_type = match condition.bytes().nth(1).unwrap() {
                        b'<' => RuleType::Less(symbol, amount),
                        b'>' => RuleType::Greater(symbol, amount),
                        _ => panic!("Invalid rule"),
                    };

                    rules_vec.push(Rule {
                        target,
                        rule: rule_type,
                    }).unwrap();
                }
                _ => panic!("Invalid rule"),
            }
        }
        workflows.insert(id, WorkFlow {
            _id: id,
            rules: rules_vec,
        }).unwrap();
    }
    workflows
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j19.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut lines = s.lines();

    let workflows = parse_workflows(&mut lines);

    let mut start = "A";

    let mut acceptance_workflows = heapless::Vec::<(&str, Part, Part), 1024>::new();

    acceptance_workflows.push((start, Part { x: 0, m: 0, a: 0, s: 0 }, Part { x: 4000, m: 4000, a: 4000, s: 4000 })).unwrap();

    let mut final_acceptance = heapless::Vec::<(&str, Part, Part), 1024>::new();

    while let Some((target, old_min, old_max)) = acceptance_workflows.pop() {
        if target == "in" {
            final_acceptance.push((target, old_min, old_max)).unwrap();
            continue;
        }
        for (id, workflow) in &workflows {
            let mut old_min = old_min.clone();
            let mut old_max = old_max.clone();
            for rule in &workflow.rules {
                let mut min = old_min.clone();
                let mut max = old_max.clone();

                println!("try_to_match : x: {}-{} m: {}-{} a: {}-{} s: {}-{}",
                         min.x, max.x, min.m, max.m, min.a, max.a, min.s, max.s);
                if rule.target == target {
                    match rule.rule {
                        RuleType::Greater(symbol, v) => {
                            match symbol {
                                Symbol::X => { min.x = min.x.max(v + 1) }
                                Symbol::M => { min.m = min.m.max(v + 1) }
                                Symbol::A => { min.a = min.a.max(v + 1) }
                                Symbol::S => { min.s = min.s.max(v + 1) }
                            }
                        }
                        RuleType::Less(symbol, v) => {
                            match symbol {
                                Symbol::X => { max.x = max.x.min(v - 1) }
                                Symbol::M => { max.m = max.m.min(v - 1) }
                                Symbol::A => { max.a = max.a.min(v - 1) }
                                Symbol::S => { max.s = max.s.min(v - 1) }
                            }
                        }
                        RuleType::AlwaysTrue => {}
                    }
                    if min.x > max.x || min.m > max.m || min.a > max.a || min.s > max.s {
                        continue;
                    }
                    acceptance_workflows.push((id, min, max)).unwrap();
                } else {
                    match rule.rule {
                        RuleType::Greater(symbol, v) => {
                            match symbol {
                                Symbol::X => { old_max.x = old_max.x.min(v) }
                                Symbol::M => { old_max.m = old_max.m.min(v) }
                                Symbol::A => { old_max.a = old_max.a.min(v) }
                                Symbol::S => { old_max.s = old_max.s.min(v) }
                            }
                        }
                        RuleType::Less(symbol, v) => {
                            match symbol {
                                Symbol::X => { old_min.x = old_min.x.max(v) }
                                Symbol::M => { old_min.m = old_min.m.max(v) }
                                Symbol::A => { old_min.a = old_min.a.max(v) }
                                Symbol::S => { old_min.s = old_min.s.max(v) }
                            }
                        }
                        RuleType::AlwaysTrue => {}
                    }
                }
            }
        }
    }

    println!();
    let mut result = 0;
    for (_, min, max) in final_acceptance {
        println!("x: {}-{} m: {}-{} a: {}-{} s: {}-{}",
                 min.x, max.x, min.m, max.m, min.a, max.a, min.s, max.s);

        result += (max.x - min.x + 1) * (max.m - min.m + 1) * (max.a - min.a + 1) * (max.s - min.s + 1);
    }

    result
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j19.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j19_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(19114, _p1(include_str!("j19_test.txt")));
        assert_eq!(263678, _p1(include_str!("j19.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        assert_eq!(167409079868000, _p2(include_str!("j19_test.txt")));
        assert_eq!(42, _p2(include_str!("j19.txt")));
    }
}