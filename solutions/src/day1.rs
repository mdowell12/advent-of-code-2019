
use crate::util::read_inputs;

fn run_1(input: &Vec<String>) -> i32 {
    input.iter()
        .map(|line| line.parse::<i32>().unwrap())
        .map(fuel_requirement)
        .sum()
}

fn run_2(input: &Vec<String>) -> i32 {
    input.iter()
        .map(|line| line.parse::<i32>().unwrap())
        .map(fuel_requirement_inclusive)
        .sum()
}

fn fuel_requirement_inclusive(mass: i32) -> i32 {
    let mut acc = fuel_requirement(mass);
    let mut next = fuel_requirement(acc);
    while next > 0 {
        acc += next;
        next = fuel_requirement(next);
    }

    acc
}

fn fuel_requirement(mass: i32) -> i32 {
    (mass / 3) - 2
}


fn run_tests() {
    assert_eq!(33583, run_1(&vec!["100756".to_string()]));
    assert_eq!(4, run_1(&vec!["12".to_string(), "14".to_string()]));

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
