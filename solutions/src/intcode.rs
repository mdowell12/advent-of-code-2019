
pub fn parse_input(input: &Vec<String>) -> Vec<i32> {
    let ints: Vec<i32> = input[0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    ints.clone()
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ExitCode {
    FINISHED,
    WAITING,
}

pub fn run_intcode(ints: &Vec<i32>, args: &Vec<i32>, start_position: usize) -> (Vec<i32>, Vec<i32>, usize, ExitCode) {
    let mut program = ints.clone();
    let mut outputs = vec![];
    let mut instruction_pointer = start_position;
    let mut args_copy = args.clone();
    let mut arg_iter = args_copy.drain(..);

    while instruction_pointer < program.len() {
        let (opcode, modes) = parse_instruction(program[instruction_pointer]);
        // println!("{} {} {:?} {:?}", instruction_pointer, opcode, modes, program);
        instruction_pointer = match opcode {
            1 => add(instruction_pointer, &mut program, &modes),
            2 => mult(instruction_pointer, &mut program, &modes),
            3 => {
                match arg_iter.next() {
                    Some(arg) => {
                        input(instruction_pointer, &mut program, arg)
                    },
                    None => {
                        // We want the program to exit as-is
                        return (program, outputs, instruction_pointer, ExitCode::WAITING);
                    },
                }
            },
            4 => output(instruction_pointer, &mut program, &mut outputs),
            5 => jump_if_true(instruction_pointer, &mut program, &modes),
            6 => jump_if_false(instruction_pointer, &mut program, &modes),
            7 => less_than(instruction_pointer, &mut program, &modes),
            8 => equals(instruction_pointer, &mut program, &modes),
            99 => return (program, outputs, instruction_pointer, ExitCode::FINISHED),
            _ => panic!(format!("Invalid opcode {:?}", opcode))
        };
    }

    panic!("Program pointer exceeded length of program without exiting")
}

fn parse_instruction(instruction: i32) -> (i32, Vec<i32>) {
    let mut digits: Vec<_> = instruction.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    while digits.len() < 5 {
        digits.insert(0, 0);
    }
    let opcode = digits[3] * 10 + digits[4];

    let mut modes: Vec<i32> = digits[0..3].iter().map(|d| *d as i32).collect();
    modes.reverse();

    (opcode as i32, modes.clone())
}

fn add(i: usize, program: &mut Vec<i32>, modes: &Vec<i32>) -> usize {
    let left = extract_value(program[i+1] as usize, program, modes[0]);
    let right = extract_value(program[i+2] as usize, program, modes[1]);

    let pos_to_overwrite = program[i+3];
    program[pos_to_overwrite as usize] = left + right;

    i + 4
}

fn mult(i: usize, program: &mut Vec<i32>, modes: &Vec<i32>) -> usize {
    let left = extract_value(program[i+1] as usize, program, modes[0]);
    let right = extract_value(program[i+2] as usize, program, modes[1]);

    let pos_to_overwrite = program[i+3];
    program[pos_to_overwrite as usize] = left * right;

    i + 4
}

fn input(i: usize, program: &mut Vec<i32>, arg: i32) -> usize {
        let position = program[i+1];
        program[position as usize] = arg;
        i + 2
}

fn output(i: usize, program: &mut Vec<i32>, outputs: &mut Vec<i32>) -> usize {
    let position = program[i+1];
    let output_value = program[position as usize];
    outputs.push(output_value);
    i + 2
}

fn jump_if_true(i: usize, program: &mut Vec<i32>, modes: &Vec<i32>) -> usize {
    let value = extract_value(program[i+1] as usize, program, modes[0]);
    if value != 0 {
        let next_position = extract_value(program[i+2] as usize, program, modes[1]);
        next_position as usize
    } else {
        i + 3
    }
}

fn jump_if_false(i: usize, program: &mut Vec<i32>, modes: &Vec<i32>) -> usize {
    let value = extract_value(program[i+1] as usize, program, modes[0]);
    if value == 0 {
        let next_position = extract_value(program[i+2] as usize, program, modes[1]);
        next_position as usize
    } else {
        i + 3
    }
}

fn less_than(i: usize, program: &mut Vec<i32>, modes: &Vec<i32>) -> usize {
    let left = extract_value(program[i+1] as usize, program, modes[0]);
    let right = extract_value(program[i+2] as usize, program, modes[1]);

    let pos_to_overwrite = program[i+3];
    let value = if left < right { 1 } else { 0 };
    program[pos_to_overwrite as usize] = value;

    i + 4
}

fn equals(i: usize, program: &mut Vec<i32>, modes: &Vec<i32>) -> usize {
    let left = extract_value(program[i+1] as usize, program, modes[0]);
    let right = extract_value(program[i+2] as usize, program, modes[1]);

    let pos_to_overwrite = program[i+3];
    let value = if left == right { 1 } else { 0 };
    program[pos_to_overwrite as usize] = value;

    i + 4
}

fn extract_value(value: usize, program: &mut Vec<i32>, mode: i32) -> i32 {
    match mode {
        0 => program[value],
        1 => value as i32,
        _ => panic!(format!("Invalid parameter mode {:?}", mode))
    }
}
