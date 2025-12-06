use aoc24::input::Input;
use itertools::Itertools;

const IN: Input = Input::new(include_str!("day17.txt"));

fn run(input: Input) -> (String, usize) {
    let (left, right) = input.split_once("\n\n");
    let program = Program {
        registers: left.numbers().map(|x| x as usize).collect(),
        instr: right.numbers().map(|x| x as usize).collect(),
        pointer: 0,
        output: vec![],
    };

    let mut p1_program = program.clone();
    p1_program.run();
    let p1 = p1_program.output.iter().join(",");

    let expected_output = program.instr.clone();
    let expected_len = program.instr.len();

    let mut right_digits = expected_len - 1;
    let mut i = 10usize.pow(expected_len as u32 - 2);
    loop {
        let mut program_clone = program.clone();
        program_clone.registers[0] = i;
        program_clone.run();

        if program_clone.output == expected_output {
            return (p1, i);
        }

        if (right_digits..expected_len).all(|d| program_clone.output[d] == expected_output[d]) {
            right_digits -= 1;
        }

        for d in right_digits..expected_len {
            if program_clone.output[d] == expected_output[d] {}
        }

        i += 1.max(10usize.pow((right_digits - 3) as u32));
    }
}

#[derive(Debug, Clone)]
struct Program {
    registers: Vec<usize>,
    instr: Vec<usize>,
    pointer: usize,
    output: Vec<usize>,
}

impl Program {
    pub fn run(&mut self) {
        while self.pointer < self.instr.len() - 1 {
            let opcode = self.instr[self.pointer];
            let operand = self.instr[self.pointer + 1];
            let combo = || match operand {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => self.registers[0],
                5 => self.registers[1],
                6 => self.registers[2],
                _ => unreachable!(),
            };

            match opcode {
                0 => self.registers[0] /= 2usize.pow(combo() as u32),
                1 => self.registers[1] ^= operand,
                2 => self.registers[1] = combo() % 8,
                3 => {
                    if self.registers[0] != 0 {
                        self.pointer = operand;
                        continue;
                    }
                }
                4 => self.registers[1] ^= self.registers[2],
                5 => self.output.push(combo() % 8),
                6 => self.registers[1] = self.registers[0] / 2usize.pow(combo() as u32),
                7 => self.registers[2] = self.registers[0] / 2usize.pow(combo() as u32),
                _ => unreachable!(),
            }

            self.pointer += 2;
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    let (p1, p2) = run(IN);
    let elapsed = now.elapsed();

    println!("Day 17\n======");
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
    println!("{}µs\n", elapsed.as_micros());
}
