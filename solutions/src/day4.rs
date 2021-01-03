
use std::collections::HashMap;

use crate::util::read_inputs;

fn run_1(input: &Vec<String>) -> i32 {
    let range: Vec<usize> = input[0].split("-").map(|s| s.parse().unwrap()).collect();
    let mut num_valid = 0;
    for i in range[0]..range[1]+1 {
        if meets_criteria(i.to_string()) {
            num_valid += 1;
        }
    }
    num_valid
}

fn run_2(input: &Vec<String>) -> i32 {
    let range: Vec<usize> = input[0].split("-").map(|s| s.parse().unwrap()).collect();
    let mut num_valid = 0;
    for i in range[0]..range[1]+1 {
        if meets_criteria_2(i.to_string()) {
            num_valid += 1;
        }
    }
    num_valid
}

fn meets_criteria(password: String) -> bool {
    let digits: Vec<usize> = password.chars().map(|c| c as usize).collect();
    let mut has_double = false;
    for i in 0..(digits.len()-1) {
        if digits[i] > digits[i+1] {
            return false;
        }
        if digits[i] == digits[i+1] {
            has_double = true;
        }
    }
    has_double
}

fn meets_criteria_2(password: String) -> bool {
    let digits: Vec<usize> = password.chars().map(|c| c as usize).collect();
    let mut counts = HashMap::new();
    for i in 0..(digits.len()-1) {
        if digits[i] > digits[i+1] {
            return false;
        }
        *counts.entry(digits[i]).or_insert(0) += 1;
    }
    *counts.entry(*digits.last().unwrap()).or_insert(0) += 1;

    counts.values().any(|count| *count == 2)
}

fn run_tests() {
    assert_eq!(true, meets_criteria("111111".to_string()));
    assert_eq!(false, meets_criteria("223450".to_string()));
    assert_eq!(false, meets_criteria("123789".to_string()));

    assert_eq!(true, meets_criteria_2("112233".to_string()));
    assert_eq!(false, meets_criteria_2("123444".to_string()));
    assert_eq!(true, meets_criteria_2("111122".to_string()));
}

pub fn run() {
    run_tests();
    let lines = read_inputs(4);

    let result_1 = run_1(&lines);
    println!("Result 1: {}", result_1);

    let result_2 = run_2(&lines);
    println!("Result 2: {}", result_2);
}
