use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
pub enum InstructionType {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    inst: InstructionType,
    num: i64,
}

#[aoc_generator(day8)]
pub fn load_input(input: &str) -> Vec<Instruction> {
    let mut output = vec![];
    for line in input.lines() {
        let vals: Vec<_> = line.split_whitespace().collect();
        let mut inst = InstructionType::Acc;
        match vals[0] {
            "acc" => inst = InstructionType::Acc,
            "jmp" => inst = InstructionType::Jmp,
            "nop" => inst = InstructionType::Nop,
            _ => (),
        }
        let num = vals[1].replace("+", "").parse().unwrap();
        let inst = Instruction { inst, num };
        output.push(inst);
    }
    output
}

pub struct VM {
    ptr: i64,
    acc: i64,
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    pub fn new() -> VM {
        VM { ptr: 0, acc: 0 }
    }

    pub fn run(&mut self, inst: &Instruction) {
        match inst.inst {
            InstructionType::Acc => {
                self.acc += inst.num;
                self.ptr += 1;
            }
            InstructionType::Jmp => self.ptr += inst.num,
            InstructionType::Nop => self.ptr += 1,
        }
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &[Instruction]) -> i64 {
    let mut vm = VM::new();
    let mut visited = HashSet::new();
    visited.insert(0);

    let output;
    loop {
        vm.run(&input[vm.ptr as usize]);
        if visited.contains(&vm.ptr) {
            output = vm.acc;
            break;
        } else {
            visited.insert(vm.ptr);
        }
    }

    output
}

pub fn run_to_completion(input: &[Instruction]) -> Option<i64> {
    let mut vm = VM::new();
    let mut visited = HashSet::new();
    visited.insert(0);

    loop {
        if vm.ptr as usize >= input.len() {
            break;
        }
        vm.run(&input[vm.ptr as usize]);
        if visited.contains(&vm.ptr) {
            return None;
        } else {
            visited.insert(vm.ptr);
        }
    }

    Some(vm.acc)
}

#[aoc(day8, part2)]
pub fn part2(input: &[Instruction]) -> i64 {
    // Try jmp -> nop
    for i in 0..input.len() {
        let mut input_clone: Vec<Instruction> = input.to_vec();
        if input[i].inst == InstructionType::Jmp {
            input_clone[i].inst = InstructionType::Nop;
        }
        if let Some(acc) = run_to_completion(&input_clone) {
            return acc;
        }
    }

    // Try nops -> jmp
    for i in 0..input.len() {
        let mut input_clone: Vec<Instruction> = input.to_vec();
        if input[i].inst == InstructionType::Nop {
            input_clone[i].inst = InstructionType::Jmp;
        }
        if let Some(acc) = run_to_completion(&input_clone) {
            return acc;
        }
    }

    // Should never get here...
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/8a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/8a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 8);
    }
}
