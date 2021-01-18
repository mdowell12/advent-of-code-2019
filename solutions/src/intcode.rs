
pub fn parse_input(input: &Vec<String>) -> Vec<i64> {
    let ints: Vec<i64> = input[0]
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    ints.clone()
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ExitCode {
    FINISHED,
    WAITING,
}

pub fn run_intcode(ints: &Vec<i64>, args: &Vec<i64>, start_position: usize) -> (Vec<i64>, Vec<i64>, usize, ExitCode) {
    let mut program = ints.clone();
    let mut outputs = vec![];
    let mut instruction_pointer = start_position;
    let mut relative_base = 0;
    let mut args_copy = args.clone();
    let mut arg_iter = args_copy.drain(..);

    while instruction_pointer < program.len() {
        let (opcode, modes) = parse_instruction(program[instruction_pointer]);
        println!("i={} op={} m={:?} relbase={} out={:?} prog={:?}", instruction_pointer, opcode, modes, relative_base, outputs, program);
        instruction_pointer = match opcode {
            1 => add(instruction_pointer, relative_base, &mut program, &modes),
            2 => mult(instruction_pointer, relative_base, &mut program, &modes),
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
            4 => output(instruction_pointer, relative_base, &mut program, &modes, &mut outputs),
            5 => jump_if_true(instruction_pointer, relative_base, &mut program, &modes),
            6 => jump_if_false(instruction_pointer, relative_base, &mut program, &modes),
            7 => less_than(instruction_pointer, relative_base, &mut program, &modes),
            8 => equals(instruction_pointer, relative_base, &mut program, &modes),
            9 => {
                // Move relative base
                let value = extract_value(program[instruction_pointer+1], &mut program, modes[0], relative_base);
                relative_base += value;
                instruction_pointer + 2
            }
            99 => return (program, outputs, instruction_pointer, ExitCode::FINISHED),
            _ => panic!(format!("Invalid opcode {:?}", opcode))
        };
    }

    panic!("Program pointer exceeded length of program without exiting")
}

fn parse_instruction(instruction: i64) -> (i64, Vec<i64>) {
    let mut digits: Vec<_> = instruction.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    while digits.len() < 5 {
        digits.insert(0, 0);
    }

    let opcode = digits[3] * 10 + digits[4];

    let mut modes: Vec<i64> = digits[0..3].iter().map(|d| *d as i64).collect();
    modes.reverse();

    (opcode as i64, modes.clone())
}

fn add(i: usize, relative_base: i64, program: &mut Vec<i64>, modes: &Vec<i64>) -> usize {
    let left = extract_value(program[i+1], program, modes[0], relative_base);
    let right = extract_value(program[i+2], program, modes[1], relative_base);

    let pos_to_overwrite = program[i+3];
    program[pos_to_overwrite as usize] = left + right;

    i + 4
}

fn mult(i: usize, relative_base: i64, program: &mut Vec<i64>, modes: &Vec<i64>) -> usize {
    let left = extract_value(program[i+1], program, modes[0], relative_base);
    let right = extract_value(program[i+2], program, modes[1], relative_base);

    let pos_to_overwrite = program[i+3];
    program[pos_to_overwrite as usize] = left * right;

    i + 4
}

fn input(i: usize, program: &mut Vec<i64>, arg: i64) -> usize {
    let position = program[i+1];
    program[position as usize] = arg;
    i + 2
}

fn output(i: usize, relative_base: i64, program: &mut Vec<i64>, modes: &Vec<i64>, outputs: &mut Vec<i64>) -> usize {
    let output_value = extract_value(program[i+1], program, modes[0], relative_base);
    outputs.push(output_value);
    i + 2
}

fn jump_if_true(i: usize, relative_base: i64, program: &mut Vec<i64>, modes: &Vec<i64>) -> usize {
    let value = extract_value(program[i+1], program, modes[0], relative_base);
    if value != 0 {
        let next_position = extract_value(program[i+2], program, modes[1], relative_base);
        next_position as usize
    } else {
        i + 3
    }
}

fn jump_if_false(i: usize, relative_base: i64, program: &mut Vec<i64>, modes: &Vec<i64>) -> usize {
    let value = extract_value(program[i+1], program, modes[0], relative_base);
    if value == 0 {
        let next_position = extract_value(program[i+2], program, modes[1], relative_base);
        next_position as usize
    } else {
        i + 3
    }
}

fn less_than(i: usize, relative_base: i64, program: &mut Vec<i64>, modes: &Vec<i64>) -> usize {
    let left = extract_value(program[i+1], program, modes[0], relative_base);
    let right = extract_value(program[i+2], program, modes[1], relative_base);

    let pos_to_overwrite = program[i+3];
    let value = if left < right { 1 } else { 0 };
    program[pos_to_overwrite as usize] = value;

    i + 4
}

fn equals(i: usize, relative_base: i64, program: &mut Vec<i64>, modes: &Vec<i64>) -> usize {
    let left = extract_value(program[i+1], program, modes[0], relative_base);
    let right = extract_value(program[i+2], program, modes[1], relative_base);

    let pos_to_overwrite = program[i+3];
    let value = if left == right { 1 } else { 0 };
    program[pos_to_overwrite as usize] = value;

    i + 4
}

fn extract_value(value: i64, program: &mut Vec<i64>, mode: i64, relative_base: i64) -> i64 {
    match mode {
        // position mode
        0 => program[value as usize],
        // reference mode
        1 => value as i64,
        // relative mode
        2 => program[(value + relative_base) as usize],
        _ => panic!(format!("Invalid parameter mode {:?}", mode))
    }
}
