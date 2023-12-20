use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SignalType {
    Low,
    High,
}

impl Display for SignalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalType::Low => write!(f, "low"),
            SignalType::High => write!(f, "high"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType<'a> {
    FlipFlop { state: bool },
    Conjunction { memory: heapless::FnvIndexMap<&'a str, SignalType, 16> },
    Broadcaster,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module<'a> {
    module_type: ModuleType<'a>,
    output: heapless::Vec<&'a str, 16>,
}

#[allow(unused)]
pub fn _p1(s: &str) -> usize {
    let mut nodes = heapless::FnvIndexMap::<&str, Module, 64>::new();

    for line in s.lines() {
        let mut split = line.split(" -> ");
        let id = split.next().unwrap();
        let output = split.next().unwrap();
        let outputs = output.split(", ").collect::<heapless::Vec<&str, 16>>();
        if id == "broadcaster" {
            nodes.insert(id, Module { module_type: ModuleType::Broadcaster, output: outputs }).unwrap();
        } else {
            match id.bytes().next().unwrap() {
                b'%' => {
                    let id = &id[1..];
                    nodes.insert(id, Module { module_type: ModuleType::FlipFlop { state: false }, output: outputs }).unwrap();
                }
                b'&' => {
                    let id = &id[1..];
                    let mut memory = heapless::FnvIndexMap::<&str, SignalType, 16>::new();
                    nodes.insert(id, Module { module_type: ModuleType::Conjunction { memory }, output: outputs }).unwrap();
                }
                _ => panic!("Invalid module type"),
            }
        }
    }

    let mut memories_to_init = heapless::Vec::<(&str, &str), 64>::new();

    for (id, module) in nodes.iter() {
        for module_output_id in module.output.iter() {
            if !nodes.contains_key(module_output_id) {
                continue
            }
            let module_output = nodes.get(module_output_id).unwrap();
            if let ModuleType::Conjunction { .. } = module_output.module_type {
                memories_to_init.push((*module_output_id, *id)).unwrap();
            }
        }
    }

    for (module_output_id, id) in memories_to_init {
        let module = nodes.get_mut(module_output_id).unwrap();
        if let ModuleType::Conjunction { ref mut memory } = module.module_type {
            memory.insert(id, SignalType::Low).unwrap();
        }
    }

    let mut signals_queue = heapless::Deque::<(&str, &str, SignalType), 64>::new();

    let mut low_count = 0;
    let mut high_count = 0;
    for _ in 0..1000 {
        signals_queue.push_back(("button", "broadcaster", SignalType::Low)).unwrap();
        while let Some((from, to, signal_type)) = signals_queue.pop_front() {
            // println!("{from} -{signal_type}-> {to}");
            if signal_type == SignalType::Low {
                low_count += 1;
            } else {
                high_count += 1;
            }
            if !nodes.contains_key(to) {
                continue;
            }
            let module = nodes.get_mut(to).unwrap();
            match module.module_type {
                ModuleType::FlipFlop { ref mut state } => {
                    if signal_type == SignalType::Low {
                        *state = !*state;
                        let signal_type = if *state { SignalType::High } else { SignalType::Low };
                        for output in module.output.iter() {
                            signals_queue.push_back((to, output, signal_type)).unwrap();
                        }
                    }
                }
                ModuleType::Conjunction { ref mut memory } => {
                    memory.insert(from, signal_type).unwrap();
                    // dbg!(&memory);
                    if memory.values().all(|&x| x == SignalType::High) {
                        for output in module.output.iter() {
                            signals_queue.push_back((to, output, SignalType::Low)).unwrap();
                        }
                    } else {
                        for output in module.output.iter() {
                            signals_queue.push_back((to, output, SignalType::High)).unwrap();
                        }

                    }
                }
                ModuleType::Broadcaster => {
                    for output in module.output.iter() {
                        signals_queue.push_back((to, output, signal_type)).unwrap();
                    }
                }
            }
        }
    }

    // dbg!(low_count, high_count);
    low_count * high_count
}

#[allow(unused)]
pub fn p1() -> usize {
    _p1(include_str!("j20.txt"))
}

#[allow(unused)]
pub fn _p2(s: &str) -> usize {
    let mut nodes = heapless::FnvIndexMap::<&str, Module, 64>::new();

    for line in s.lines() {
        let mut split = line.split(" -> ");
        let id = split.next().unwrap();
        let output = split.next().unwrap();
        let outputs = output.split(", ").collect::<heapless::Vec<&str, 16>>();
        if id == "broadcaster" {
            nodes.insert(id, Module { module_type: ModuleType::Broadcaster, output: outputs }).unwrap();
        } else {
            match id.bytes().next().unwrap() {
                b'%' => {
                    let id = &id[1..];
                    nodes.insert(id, Module { module_type: ModuleType::FlipFlop { state: false }, output: outputs }).unwrap();
                }
                b'&' => {
                    let id = &id[1..];
                    let mut memory = heapless::FnvIndexMap::<&str, SignalType, 16>::new();
                    nodes.insert(id, Module { module_type: ModuleType::Conjunction { memory }, output: outputs }).unwrap();
                }
                _ => panic!("Invalid module type"),
            }
        }
    }

    let mut memories_to_init = heapless::Vec::<(&str, &str), 64>::new();


    for (id, module) in nodes.iter() {
        for module_output_id in module.output.iter() {
            if !nodes.contains_key(module_output_id) {
                continue
            }
            let module_output = nodes.get(module_output_id).unwrap();
            if let ModuleType::Conjunction { .. } = module_output.module_type {
                memories_to_init.push((*module_output_id, *id)).unwrap();
            }
        }
    }

    for (module_output_id, id) in memories_to_init {
        let module = nodes.get_mut(module_output_id).unwrap();
        if let ModuleType::Conjunction { ref mut memory } = module.module_type {
            memory.insert(id, SignalType::Low).unwrap();
        }
    }

    let mut signals_queue = heapless::Deque::<(&str, &str, SignalType), 64>::new();

    let mut button_press = 0;
    'super_loop: loop {
        button_press += 1;
        signals_queue.push_back(("button", "broadcaster", SignalType::Low)).unwrap();
        while let Some((from, to, signal_type)) = signals_queue.pop_front() {
            if signal_type == SignalType::Low && to == "rx" {
                break 'super_loop;
            }
            if !nodes.contains_key(to) {
                continue;
            }
            let module = nodes.get_mut(to).unwrap();
            match module.module_type {
                ModuleType::FlipFlop { ref mut state } => {
                    if signal_type == SignalType::Low {
                        *state = !*state;
                        let signal_type = if *state { SignalType::High } else { SignalType::Low };
                        for output in module.output.iter() {
                            signals_queue.push_back((to, output, signal_type)).unwrap();
                        }
                    }
                }
                ModuleType::Conjunction { ref mut memory } => {
                    memory.insert(from, signal_type).unwrap();
                    // dbg!(&memory);
                    if memory.values().all(|&x| x == SignalType::High) {
                        for output in module.output.iter() {
                            signals_queue.push_back((to, output, SignalType::Low)).unwrap();
                        }
                    } else {
                        for output in module.output.iter() {
                            signals_queue.push_back((to, output, SignalType::High)).unwrap();
                        }

                    }
                }
                ModuleType::Broadcaster => {
                    for output in module.output.iter() {
                        signals_queue.push_back((to, output, signal_type)).unwrap();
                    }
                }
            }
        }
    }

    button_press
}

#[allow(unused)]
pub fn p2() -> usize {
    _p2(include_str!("j20.txt"))
}

#[cfg(test)]
#[allow(unused)]
mod j20_tests {
    #[allow(unused)]
    use super::*;

    #[test]
    #[allow(unused)]
    fn test_p1() {
        assert_eq!(32000000, _p1(include_str!("j20_test_p1.txt")));
        assert_eq!(11687500, _p1(include_str!("j20_test_p1_2.txt")));
        assert_eq!(670984704, _p1(include_str!("j20.txt")));
    }

    #[test]
    #[allow(unused)]
    fn test_p2() {
        // assert_eq!(42, _p2(include_str!("j20_test_p1.txt")));
        assert_eq!(42, _p2(include_str!("j20.txt")));
    }
}