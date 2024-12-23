use std::collections::BTreeSet;

advent_of_code::solution!(17);

struct Computer {
    a: usize,
    b: usize,
    c: usize,
    output: Vec<usize>,
    pointer: usize,
}

impl Computer {
    fn adv(&mut self, operand: usize) {
        self.a /= 2_usize.pow(operand as u32);
        self.pointer += 2;
    }

    fn bxl(&mut self, operand: usize) {
        self.b ^= operand;
        self.pointer += 2;
    }

    fn bst(&mut self, operand: usize) {
        self.b = operand % 8;
        self.pointer += 2;
    }

    fn jnz(&mut self, operand: usize) {
        if self.a != 0 {
            self.pointer = operand
        } else {
            self.pointer += 2;
        }
    }

    fn bxc(&mut self) {
        self.b ^= self.c;
        self.pointer += 2;
    }

    fn out(&mut self, operand: usize) {
        self.output.push(operand % 8);
        self.pointer += 2;
    }

    fn bdv(&mut self, operand: usize) {
        self.b = self.a / 2_usize.pow(operand as u32);
        self.pointer += 2;
    }

    fn cdv(&mut self, operand: usize) {
        self.c = self.a / 2_usize.pow(operand as u32);
        self.pointer += 2;
    }

    fn combo(&mut self, operand: usize) -> usize {
        match operand {
            x @ 0..4 => x,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => 0,
            _ => panic!(),
        }
    }

    fn run(&mut self, instructions: &[usize]) {
        self.output.clear();

        while self.pointer < instructions.len() {
            let (opcode, operand) = (instructions[self.pointer], instructions[self.pointer + 1]);

            let combo = self.combo(operand);

            match opcode {
                0 => self.adv(combo),
                1 => self.bxl(operand),
                2 => self.bst(combo),
                3 => self.jnz(operand),
                4 => self.bxc(),
                5 => self.out(combo),
                6 => self.bdv(combo),
                7 => self.cdv(combo),
                _ => panic!(),
            };
        }

        self.pointer = 0;
    }

    fn formatted_output(&self) -> String {
        self.output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn copy_input(
        &mut self,
        a: usize,
        compare_index: usize,
        instructions: &[usize],
        copiers: &mut BTreeSet<usize>,
    ) -> usize {
        for n in 0..8 {
            let new_a = (a << 3) | n;
            self.a = new_a;
            self.b = 0;
            self.c = 0;
            self.run(instructions);

            if self.output == instructions[instructions.len() - compare_index..] {
                if self.output == instructions {
                    copiers.insert(new_a);
                } else {
                    self.copy_input(new_a, compare_index + 1, instructions, copiers);
                }
            }
        }

        *copiers.iter().next().unwrap_or(&0)
    }
}

fn setup(input: &str) -> (Computer, Vec<usize>) {
    let input_vector: Vec<String> = input
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect();

    let computer = Computer {
        a: input_vector[2].parse().unwrap(),
        b: input_vector[5].parse().unwrap(),
        c: input_vector[8].parse().unwrap(),
        output: Vec::new(),
        pointer: 0,
    };

    let instructions: Vec<usize> = input_vector[10]
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    (computer, instructions)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut computer, instructions) = setup(input);

    computer.run(&instructions);

    Some(computer.formatted_output())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut computer, instructions) = setup(input);
    computer.a = 0;

    let mut copiers = BTreeSet::new();

    Some(computer.copy_input(0, 1, &instructions, &mut copiers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5,7,3,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
