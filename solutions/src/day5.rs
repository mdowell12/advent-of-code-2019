
use crate::intcode::IntCode;
use crate::intcode::Status;
use crate::intcode::parse_input;
use crate::intcode::run_intcode;
use crate::util::read_inputs;

fn run_1(input: &Vec<String>) -> i32 {
    let program = parse_input(input);
    let intcode = IntCode { program, ..Default::default() };
    let (_, outputs) = run_intcode(intcode, &vec![]);
    *outputs.last().unwrap()
}

fn run_2(input: &Vec<String>) -> i32 {
    let program = parse_input(input);
    let intcode = IntCode { program, ..Default::default() };
    let (_, outputs) = run_intcode(intcode, &vec![5]);
    // *outputs.last().unwrap()
    *outputs.last().unwrap()
}

fn run_tests() {
    // assert_eq!(
    //     (IntCode { program: vec![3500,9,10,70,2,3,11,0,99,30,40,50], position: 8, status: Status::FINISHED }, vec![]),
    //     run_intcode(IntCode { program: vec![1,9,10,3,2,3,11,0,99,30,40,50], ..Default::default() }, &vec![])
    // );

    // assert_eq!((vec![1002,4,3,4,99], vec![], 4, Status::FINISHED), run_intcode(&vec![1002,4,3,4,33], &vec![], 0));
    // assert_eq!((vec![1101,100,-1,4,99], vec![], 4, Status::FINISHED), run_intcode(&vec![1101,100,-1,4,0], &vec![], 0));
    // assert_eq!((vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, 112, 1, 1, 9], vec![1], 11, Status::FINISHED), run_intcode(&vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &vec![112], 0));
    // assert_eq!((vec![3, 3, 1105, 112, 9, 1101, 0, 0, 12, 4, 12, 99, 1], vec![1], 11, Status::FINISHED), run_intcode(&vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &vec![112], 0));
    // assert_eq!((vec![3, 9, 7, 9, 10, 9, 4, 9, 99, 0, 8], vec![0], 8, Status::FINISHED), run_intcode(&vec![3,9,7,9,10,9,4,9,99,-1,8], &vec![123], 0));
    // assert_eq!((vec![3,9,8,9,10,9,4,9,99,1,8], vec![1], 8, Status::FINISHED), run_intcode(&vec![3,9,8,9,10,9,4,9,99,-1,8], &vec![8], 0));
}

pub fn run() {
    run_tests();
    let lines = read_inputs(5);

    let result_1 = run_1(&lines);
    println!("Result 1: {}", result_1);

    let result_2 = run_2(&lines);
    println!("Result 2: {}", result_2);
}
