use crate::intcode::IntCode;
use crate::intcode::Status;
use crate::intcode::parse_input;
use crate::intcode::run_intcode;
use crate::util::read_inputs;

fn run_1(input: &Vec<String>) -> i32 {
    let mut ints = parse_input(input);
    ints[1] = 12;
    ints[2] = 2;
    let intcode = IntCode { program: ints, ..Default::default() };
    let (output_intcode, _) = run_intcode(intcode, &vec![]);
    output_intcode.program[0]
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
            let intcode = IntCode { program, ..Default::default() };
            let (output_intcode, _) = run_intcode(intcode, &vec![]);
            let result = output_intcode.program[0];
            if result == 19690720 {
                return 100 * noun + verb
            }
        }
    }
    -1
}

fn run_tests() {
    assert_eq!(
        (IntCode { program: vec![2,0,0,0,99], position: 4, status: Status::FINISHED }, vec![]),
        run_intcode(IntCode { program: vec![1,0,0,0,99], ..Default::default() }, &vec![])
    );
    assert_eq!(
        (IntCode { program: vec![3500,9,10,70,2,3,11,0,99,30,40,50], position: 8, status: Status::FINISHED }, vec![]),
        run_intcode(IntCode { program: vec![1,9,10,3,2,3,11,0,99,30,40,50], ..Default::default() }, &vec![])
    );
}

pub fn run() {
    run_tests();
    let lines = read_inputs(2);

    let result_1 = run_1(&lines);
    println!("Result 1: {}", result_1);

    let result_2 = run_2(&lines);
    println!("Result 2: {}", result_2);
}
