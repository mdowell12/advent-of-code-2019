
use crate::intcode::parse_input;
use crate::intcode::run_intcode;
use crate::util::read_inputs;

fn run_1(input: &Vec<String>) -> i32 {
    let program = parse_input(input);
    let (_, outputs) = run_intcode(&program, 1);
    *outputs.last().unwrap()
}

fn run_2(input: &Vec<String>) -> i32 {
    let program = parse_input(input);
    let (_, outputs) = run_intcode(&program, 5);
    *outputs.last().unwrap()
}

fn run_tests() {
    assert_eq!((vec![1002,4,3,4,99], vec![]), run_intcode(&vec![1002,4,3,4,33], -1));
    assert_eq!((vec![1101,100,-1,4,99], vec![]), run_intcode(&vec![1101,100,-1,4,0], -1));
    assert_eq!((vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 112, 1, 1, 9], vec![1]), run_intcode(&vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 112));
    assert_eq!((vec![3, 3, 1105, 112, 9, 1101, 0, 0, 12, 4, 12, 99, 1], vec![1]), run_intcode(&vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 112));
    assert_eq!((vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8], vec![0]), run_intcode(&vec![3,9,7,9,10,9,4,9,99,-1,8], 123));
    assert_eq!((vec![3,9,8,9,10,9,4,9,99,1,8], vec![1]), run_intcode(&vec![3,9,8,9,10,9,4,9,99,-1,8], 8));
}

pub fn run() {
    run_tests();
    let lines = read_inputs(5);

    let result_1 = run_1(&lines);
    println!("Result 1: {}", result_1);

    let result_2 = run_2(&lines);
    println!("Result 2: {}", result_2);
}
