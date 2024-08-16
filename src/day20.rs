use std::collections::HashMap;

use advent_of_code_2023::text_to_string_vec;
use itertools::Itertools;

#[derive(PartialEq, Eq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
    None
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Pulse {
    High,
    Low,
    None
}


#[derive(PartialEq, Eq)]
enum State {
    On,
    Off
}

struct Module<'a> {
    name: &'a str,
    state: HashMap<&'a str, State>,
    m_type: ModuleType,
    targets : Vec<&'a str>
}

impl<'a> Module<'a> {

    fn new(module_name : &'a str, module_type: &str, targets: Vec<&'a str>) -> Self {
        Module{
            name: module_name,
            state: HashMap::new(),
            m_type: match module_type {
                "%" => ModuleType::FlipFlop,
                "&" => ModuleType::Conjunction,
                "b" => ModuleType::Broadcaster,
                _ => ModuleType::None
            },
            targets: targets,
        }
    }

    fn register(&mut self, module_name : &'a str) {
        if self.m_type == ModuleType::Conjunction {
            self.state.insert(module_name, State::Off);
        }
    }

    fn init(&mut self) {
        if self.m_type == ModuleType::FlipFlop {
            self.state.insert(self.name, State::Off);
        }
    }

    fn process(&mut self, source: &'a str, pulse: Pulse) -> Vec<(&str, Pulse)> {
        let mut result = Vec::new();
        let pulse_to_send = 
        match self.m_type {
            ModuleType::FlipFlop => {
                match pulse {
                    Pulse::High | Pulse::None=> Pulse::None,
                    Pulse::Low => {
                        let state = self.state.get_mut(self.name).unwrap();
                        match state {
                            State::On => {
                                self.state.insert(self.name, State::Off);
                                Pulse::Low

                            },
                            State::Off => {
                                self.state.insert(self.name, State::On);
                                Pulse::High
                            },
                        }
                    },
                }
            },
            ModuleType::Conjunction => {
                match pulse {
                    Pulse::High => {
                        self.state.insert(source, State::On);
                    },
                    Pulse::Low => {
                        self.state.insert(source, State::Off);
                    },
                    Pulse::None => todo!(),
                };
                if self.state.values().all(|state| state == &State::On) {
                    Pulse::Low
                }
                else {
                    Pulse::High
                }
            },
            ModuleType::Broadcaster => {
                pulse
            },
            ModuleType::None => todo!(),
        };

        if pulse_to_send != Pulse::None {
            for t in self.targets.clone() {
                result.push((t, pulse_to_send));
            }
        }

        result
    }
}

pub fn solve_day20_part1(input: &str) -> u64 {
    let mut modules = HashMap::new();
    for l in input.lines() {
        if l.is_empty() {
            continue;
        }

        let (module, targets) = l.split(" -> ").next_tuple().unwrap();
        let module_type = &module[..1];
        let module_name = module[1..].trim();
        let targets = targets.split(", ").collect();

        modules.insert(module_name, Module::new(module_name, module_type, targets));
    };

    let mut register_modules = Vec::new();
    for m in modules.values_mut() {
        m.init();
        register_modules.push((m.name, m.targets.clone()));
    }
    for m in register_modules {
        for t in m.1 {
            modules.get_mut(t).unwrap().register(m.0);
        }
    }

    let mut count = 0;
    loop {
        let mut modules_to_process = Vec::new();
        modules_to_process.push(("button", Pulse::Low, "roadcaster"));
        count +=1;
        while let Some((source, pulse, m_name)) = modules_to_process.pop() {
            let m = modules.get(m_name).unwrap();
            let m = modules.get_mut(m_name).unwrap();
            let pulses = m.process(source, pulse);
            for (t, p) in pulses {
                modules_to_process.push((m_name, p, t));
            }
        }
        if count == 1000 {
            break
        }
    }

    0
}

pub fn solve_day20_part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests{
    use crate::day20::{solve_day20_part1, Pulse};

    use super::{Module, ModuleType};

    const EXAMPLE : &'static str =
"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    const EXAMPLE2 : &'static str =
"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

    #[test]
    fn test_day20_p1() {
        assert_eq!(solve_day20_part1(EXAMPLE), 32000000);
        assert_eq!(solve_day20_part1(EXAMPLE2), 11687500)
    }

    #[test]
    fn test_pulse_process_flipflop() {
        let mut flipflop_module = Module::new("test_module", "%", vec!["a", "b", "c"]);
        flipflop_module.init();

        assert_eq!(flipflop_module.process("test", Pulse::High), vec![]);
        assert_eq!(flipflop_module.process("test", Pulse::Low), vec![("a", Pulse::High), ("b", Pulse::High), ("c", Pulse::High)]);
        assert_eq!(flipflop_module.process("test", Pulse::High), vec![]);
        assert_eq!(flipflop_module.process("test", Pulse::Low), vec![("a", Pulse::Low), ("b", Pulse::Low), ("c", Pulse::Low)]);
    }

    #[test]
    fn test_pulse_process_conjunction(){
        let mut conjunction_module = Module::new("test_module", "&", vec!["a"]);
        conjunction_module.init();
        conjunction_module.register("a");
        conjunction_module.register("b");

        assert_eq!(conjunction_module.process("a", Pulse::High), vec![("a", Pulse::High)]);
        assert_eq!(conjunction_module.process("b", Pulse::High), vec![("a", Pulse::Low)]);

        assert_eq!(conjunction_module.process("a", Pulse::Low), vec![("a", Pulse::High)]);
        assert_eq!(conjunction_module.process("a", Pulse::High), vec![("a", Pulse::Low)]);
    }

    #[test]
    fn test_day20_p2() {
    }
}