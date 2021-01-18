use std::collections::HashMap;

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
    let mut memory: HashMap<i64, i64> = ints.iter()
                                            .enumerate()
                                            .map(|(i, v)| (i as i64, *v))
                                            .collect();
    let mut outputs = vec![];
    let mut instruction_pointer = start_position as i64;
    let mut relative_base = 0;
    let mut args_copy = args.clone();
    let mut arg_iter = args_copy.drain(..);

    while instruction_pointer < memory.len() as i64 {
        let (opcode, modes) = parse_instruction(*memory.get(&(instruction_pointer)).unwrap());
        // println!("i={} op={} m={:?} relbase={} out={:?} mem={:?}", instruction_pointer, opcode, modes, relative_base, outputs, memory);
        // println!("i={} op={} m={:?} relbase={} out={:?}", instruction_pointer, opcode, modes, relative_base, outputs);
        instruction_pointer = match opcode {
            1 => add(instruction_pointer, relative_base, &mut memory, &modes),
            2 => mult(instruction_pointer, relative_base, &mut memory, &modes),
            3 => {
                match arg_iter.next() {
                    Some(arg) => {
                        input(instruction_pointer, relative_base, &mut memory, arg, &modes)
                    },
                    None => {
                        // We want the program to exit as-is
                        return (final_program(&mut memory, ints.len()), outputs, instruction_pointer as usize, ExitCode::WAITING);
                    },
                }
            },
            4 => output(instruction_pointer, relative_base, &mut memory, &modes, &mut outputs),
            5 => jump_if_true(instruction_pointer, relative_base, &mut memory, &modes),
            6 => jump_if_false(instruction_pointer, relative_base, &mut memory, &modes),
            7 => less_than(instruction_pointer, relative_base, &mut memory, &modes),
            8 => equals(instruction_pointer, relative_base, &mut memory, &modes),
            9 => {
                // Move relative base
                let value = extract_value(instruction_pointer+1, &mut memory, modes[0], relative_base);
                relative_base += value;
                instruction_pointer + 2
            }
            99 => return (final_program(&mut memory, ints.len()), outputs, instruction_pointer as usize, ExitCode::FINISHED),
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

fn add(i: i64, relative_base: i64, memory: &mut HashMap<i64, i64>, modes: &Vec<i64>) -> i64 {
    let left = extract_value(i+1, memory, modes[0], relative_base);
    let right = extract_value(i+2, memory, modes[1], relative_base);

    let pos_to_overwrite = *memory.get(&(i+3)).unwrap();

    overwrite(pos_to_overwrite, left + right, relative_base, modes[2], memory);

    i + 4
}

fn mult(i: i64, relative_base: i64, memory: &mut HashMap<i64, i64>, modes: &Vec<i64>) -> i64 {
    let left = extract_value(i+1, memory, modes[0], relative_base);
    let right = extract_value(i+2, memory, modes[1], relative_base);

    let pos_to_overwrite = *memory.get(&(i+3)).unwrap();

    overwrite(pos_to_overwrite, left * right, relative_base, modes[2], memory);

    i + 4
}

fn input(i: i64, relative_base: i64, memory: &mut HashMap<i64, i64>, arg: i64, modes: &Vec<i64>) -> i64 {
    let position = *memory.get(&(i+1)).unwrap();

    overwrite(position, arg, relative_base, modes[0], memory);
    i + 2
}

fn output(
    i: i64,
    relative_base: i64,
    memory: &mut HashMap<i64, i64>,
    modes: &Vec<i64>,
    outputs: &mut Vec<i64>) -> i64 {

    let output_value = extract_value(i+1, memory, modes[0], relative_base);
    outputs.push(output_value);
    i + 2
}

fn jump_if_true(i: i64, relative_base: i64, memory: &mut HashMap<i64, i64>, modes: &Vec<i64>) -> i64 {
    let value = extract_value(i+1, memory, modes[0], relative_base);
    if value != 0 {
        let next_position = extract_value(i+2, memory, modes[1], relative_base);
        next_position
    } else {
        i + 3
    }
}

fn jump_if_false(i: i64, relative_base: i64, memory: &mut HashMap<i64, i64>, modes: &Vec<i64>) -> i64 {
    let value = extract_value(i+1, memory, modes[0], relative_base);
    if value == 0 {
        let next_position = extract_value(i+2, memory, modes[1], relative_base);
        next_position
    } else {
        i + 3
    }
}

fn less_than(i: i64, relative_base: i64, memory: &mut HashMap<i64, i64>, modes: &Vec<i64>) -> i64 {
    let left = extract_value(i+1, memory, modes[0], relative_base);
    let right = extract_value(i+2, memory, modes[1], relative_base);

    let pos_to_overwrite = *memory.get(&(i+3)).unwrap();
    let value = if left < right { 1 } else { 0 };

    overwrite(pos_to_overwrite, value, relative_base, modes[2], memory);

    i + 4
}

fn equals(i: i64, relative_base: i64, memory: &mut HashMap<i64, i64>, modes: &Vec<i64>) -> i64 {
    let left = extract_value(i+1, memory, modes[0], relative_base);
    let right = extract_value(i+2, memory, modes[1], relative_base);

    let pos_to_overwrite = *memory.get(&(i+3)).unwrap();
    let value = if left == right { 1 } else { 0 };

    overwrite(pos_to_overwrite, value, relative_base, modes[2], memory);

    i + 4
}

fn extract_value(instruction_position: i64, memory: &mut HashMap<i64, i64>, mode: i64, relative_base: i64) -> i64 {
    let value = *memory.get(&instruction_position).unwrap();
    match mode {
        // position mode
        0 => *memory.get(&value).unwrap_or(&0),
        // immediate mode
        1 => value as i64,
        // relative mode
        2 => *memory.get(&(value + relative_base)).unwrap_or(&0),
        _ => panic!(format!("Invalid parameter mode {:?}", mode))
    }
}

fn overwrite(i: i64, value: i64, relative_base: i64, mode: i64, memory: &mut HashMap<i64, i64>) {
    match mode {
        // position mode
        0 => {
            memory.insert(i, value);
        },
        // immediate mode
        1 => panic!("Cannot write in immediate mode"),
        // relative mode
        2 => {
            memory.insert(i + relative_base, value);
        },
        _ => panic!(format!("Invalid parameter mode {:?}", mode))
    }
}

fn final_program(memory: &mut HashMap<i64, i64>, original_len: usize) -> Vec<i64> {
    (0..original_len).map(|i| *memory.get(&(i as i64)).unwrap()).collect()
}
