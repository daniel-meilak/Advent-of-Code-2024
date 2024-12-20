advent_of_code::solution!(17);

struct Registers {
    a: i32,
    b: i32,
    c: i32,
}

fn adv(a: &mut i32, operand: i32, pointer: &mut usize) {
    *a /= 2_i32.pow(operand as u32);
    *pointer += 2;
}

fn bxl(b: &mut i32, operand: i32, pointer: &mut usize) {
    *b ^= operand;
    *pointer += 2;
}

fn bst(b: &mut i32, operand: i32, pointer: &mut usize) {
    *b = operand % 8;
    *pointer += 2;
}

fn jnz(a: i32, operand: i32, pointer: &mut usize) {
    if a != 0 {
        *pointer = operand as usize
    } else {
        *pointer += 2;
    }
}

fn bxc(b: &mut i32, c: i32, pointer: &mut usize) {
    *b ^= c;
    *pointer += 2;
}

fn out(operand: i32, pointer: &mut usize, output: &mut Vec<String>) {
    output.push((operand % 8).to_string());
    *pointer += 2;
}

fn bdv(a: i32, b: &mut i32, operand: i32, pointer: &mut usize) {
    *b = a / 2_i32.pow(operand as u32);
    *pointer += 2;
}

fn cdv(a: i32, c: &mut i32, operand: i32, pointer: &mut usize) {
    *c = a / 2_i32.pow(operand as u32);
    *pointer += 2;
}

fn run_opcode(opcode: usize, operand: i32, registers: &mut Registers, pointer: &mut usize, output: &mut Vec<String>) { 
    let combo_operand = match operand {
        x @ 0..4 => x ,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        7 => 0,
        _ => panic!(),
    };

    match opcode {
        0 => adv(&mut registers.a, combo_operand, pointer),
        1 => bxl(&mut registers.b, operand, pointer),
        2 => bst(&mut registers.b, combo_operand, pointer),
        3 => jnz(registers.a, operand, pointer),
        4 => bxc(&mut registers.b, registers.c, pointer),
        5 => out(combo_operand, pointer, output),
        6 => bdv(registers.a, &mut registers.b, combo_operand, pointer),
        7 => cdv(registers.a, &mut registers.c, combo_operand, pointer),
        _ => panic!(),
    };
}

pub fn part_one(input: &str) -> Option<String> {
    let input_vector: Vec<String> = input
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect();

    let mut registers = Registers {
        a: input_vector[2].parse().unwrap(),
        b: input_vector[5].parse().unwrap(),
        c: input_vector[8].parse().unwrap()
    };

    let instructions: Vec<usize> = input_vector[10]
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut output = Vec::new();
    let mut pointer = 0;
    while pointer < instructions.len() {
        run_opcode(instructions[pointer], instructions[pointer + 1] as i32, &mut registers, &mut pointer, &mut output);
    }
    
    Some(output.join(","))
}

pub fn part_two(input: &str) -> Option<u32> {
    
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
