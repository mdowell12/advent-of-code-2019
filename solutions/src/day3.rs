use std::collections::HashMap;
use std::collections::HashSet;

use crate::util::read_inputs;

fn run_1(input: &Vec<String>) -> i32 {
    let wire_1: Vec<&str> = input[0].split(",").collect();
    let wire_2: Vec<&str> = input[1].split(",").collect();

    let wire_1_coordinates_to_steps = get_coordinates_traveled(&wire_1);
    let wire_2_coordinates_to_steps = get_coordinates_traveled(&wire_2);

    let wire_1_coordinates: HashSet<(i32, i32)> = wire_1_coordinates_to_steps.keys().cloned().collect();
    let wire_2_coordinates: HashSet<(i32, i32)> = wire_2_coordinates_to_steps.keys().cloned().collect();

    wire_1_coordinates.intersection(&wire_2_coordinates).into_iter()
        .map(manhattan_dist)
        .min()
        .unwrap()
}

fn manhattan_dist(coordinate: &(i32, i32)) -> i32 {
    let (x, y) = coordinate;
    x.abs() + y.abs()
}

fn get_coordinates_traveled(wire: &Vec<&str>) -> HashMap<(i32, i32), i32> {
    let mut coordinates = HashMap::new();

    let mut current = (0, 0);
    let mut steps = 0;
    for instruction in wire.iter() {
        let (direction, magnitude_str) = instruction.split_at(1);
        let magnitude: i32 = magnitude_str.parse().unwrap();
        for _ in 1..(magnitude + 1) {
            steps += 1;
            let (x, y) = current;
            current = match direction {
                "R" => (x + 1, y),
                "L" => (x - 1, y),
                "U" => (x, y + 1),
                "D" => (x, y - 1),
                _ => panic!(format!("Invalid direction {}", direction))
            };
            // TODO handle case when position encountered twice
            coordinates.insert(current, steps);
        }
    }
    coordinates
}

fn run_2(input: &Vec<String>) -> i32 {
    let wire_1: Vec<&str> = input[0].split(",").collect();
    let wire_2: Vec<&str> = input[1].split(",").collect();

    let wire_1_coordinates_to_steps = get_coordinates_traveled(&wire_1);
    let wire_2_coordinates_to_steps = get_coordinates_traveled(&wire_2);

    let wire_1_coordinates: HashSet<(i32, i32)> = wire_1_coordinates_to_steps.keys().cloned().collect();
    let wire_2_coordinates: HashSet<(i32, i32)> = wire_2_coordinates_to_steps.keys().cloned().collect();

    wire_1_coordinates.intersection(&wire_2_coordinates).into_iter()
        .map(|coord| wire_1_coordinates_to_steps.get(coord).unwrap() + wire_2_coordinates_to_steps.get(coord).unwrap())
        .min()
        .unwrap()
}

fn run_tests() {
    assert_eq!(159, run_1(&vec![
        "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(),
        "U62,R66,U55,R34,D71,R55,D58,R83".to_string()
    ]));

    assert_eq!(135, run_1(&vec![
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(),
        "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string()
    ]));

    assert_eq!(610, run_2(&vec![
        "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(),
        "U62,R66,U55,R34,D71,R55,D58,R83".to_string()
    ]));

    assert_eq!(410, run_2(&vec![
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(),
        "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string()
    ]));
}

pub fn run() {
    run_tests();
    let lines = read_inputs(3);

    let result_1 = run_1(&lines);
    println!("Result 1: {}", result_1);

    let result_2 = run_2(&lines);
    println!("Result 2: {}", result_2);
}
