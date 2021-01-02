use std::fs;
use std::str;

fn run_1(input: &[&str]) -> i32 {
    input.iter()
        .map(|line| line.parse::<i32>().unwrap())
        .map(fuel_requirement)
        .sum()
}

fn run_2(input: &[&str]) -> i32 {
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
    assert_eq!(33583, run_1(&["100756"]));
    assert_eq!(4, run_1(&["12", "14"]));

    assert_eq!(50346, run_2(&["100756"]));
}

fn main() {
    run_tests();

    let input = fs::read_to_string("../inputs/1.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = input.split("\n").collect();

    let result_1 = run_1(&lines);
    println!("Result 1: {}", result_1);

    let result_2 = run_2(&lines);
    println!("Result 2: {}", result_2);

}
