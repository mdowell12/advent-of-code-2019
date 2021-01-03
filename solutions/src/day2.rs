
use crate::util::read_inputs;

fn run_1(input: &Vec<String>) -> i32 {
    let mut ints: Vec<i32> = input[0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
        ;
    ints[1] = 12;
    ints[2] = 2;
    let result = run_int_code(&ints);
    result[0]
}

fn run_2(input: &Vec<String>) -> i32 {
    let ints: Vec<i32> = input[0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
        ;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut program = ints.clone();
            program[1] = noun;
            program[2] = verb;
            let result = run_int_code(&program)[0];
            if result == 19690720 {
                return 100 * noun + verb
            }
        }
    }
    -1
}

fn run_int_code(ints: &Vec<i32>) -> Vec<i32> {
    let mut program = ints.clone();
    let mut instruction_pointer = 0;

    while instruction_pointer < program.len() {
        let inc = match program[instruction_pointer] {
            1 => add(instruction_pointer, &mut program),
            2 => mult(instruction_pointer, &mut program),
            99 => break,
            _ => panic!(format!("Invalid opcode {}", instruction_pointer))
        };
        instruction_pointer += inc;
    }

    program
}

fn add(i: usize, program: &mut Vec<i32>) -> usize {
    let pos_left = program[i+1];
    let left = program[pos_left as usize];

    let pos_right = program[i+2];
    let right = program[pos_right as usize];

    let pos_to_overwrite = program[i+3];
    program[pos_to_overwrite as usize] = left + right;

    4
}

fn mult(i: usize, program: &mut Vec<i32>) -> usize {
    let pos_left = program[i+1];
    let left = program[pos_left as usize];

    let pos_right = program[i+2];
    let right = program[pos_right as usize];

    let pos_to_overwrite = program[i+3];
    program[pos_to_overwrite as usize] = left * right;

    4
}

fn run_tests() {
    assert_eq!(vec![2,0,0,0,99], run_int_code(&vec![1,0,0,0,99]));
    assert_eq!(vec![3500,9,10,70,2,3,11,0,99,30,40,50], run_int_code(&vec![1,9,10,3,2,3,11,0,99,30,40,50]));
}

pub fn run() {
    run_tests();
    let lines = read_inputs(2);

    let result_1 = run_1(&lines);
    println!("Result 1: {}", result_1);

    let result_2 = run_2(&lines);
    println!("Result 2: {}", result_2);
}
