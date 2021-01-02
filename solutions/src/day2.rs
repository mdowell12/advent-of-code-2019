
use crate::util::read_inputs;

fn run_1(input: &Vec<String>) -> i32 {
    0
}

fn run_2(input: &Vec<String>) -> i32 {
    0
}

fn run_tests() {
    assert_eq!(33583, run_1(&vec!["100756".to_string()]));

    assert_eq!(50346, run_2(&vec!["100756".to_string()]));
}

pub fn run() {
    run_tests();
    let lines = read_inputs(1);

    let result_1 = run_1(&lines);
    println!("Result 1: {}", result_1);

    let result_2 = run_2(&lines);
    println!("Result 2: {}", result_2);
}
