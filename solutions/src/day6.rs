use std::collections::HashMap;
use crate::util::read_inputs;

fn run_1(input: &Vec<String>) -> i32 {
    let orbits = parse_orbits(input);
    let mut num_orbits = 0;

    for planet in orbits.keys() {
        let mut parent = orbits.get(planet).unwrap();
        while parent.to_string() != "COM" {
            num_orbits += 1;
            parent = orbits.get(parent).unwrap();
        }
        num_orbits += 1;
    }

    num_orbits
}

fn run_2(input: &Vec<String>) -> i32 {
    let orbits = parse_orbits(input);

    let mut santa_to_root = HashMap::new();
    let mut santa_parent = orbits.get("SAN").unwrap();
    let mut num_traversals = 0;
    while santa_parent.to_string() != "COM" {
        santa_to_root.insert(santa_parent, num_traversals);
        santa_parent = orbits.get(santa_parent).unwrap();
        num_traversals += 1;
    }
    santa_to_root.insert(&"COM", num_traversals);

    let mut my_parent = orbits.get("YOU").unwrap();
    let mut my_traversals = 0;
    while !santa_to_root.contains_key(my_parent) {
        my_parent = orbits.get(my_parent).unwrap();
        my_traversals += 1;
    }

    my_traversals + santa_to_root.get(my_parent).unwrap()
}

fn parse_orbits(input: &Vec<String>) -> HashMap<&str, &str> {
    let mut result = HashMap::new();
    for line in input {
        let parts: Vec<&str> = line.split(")").collect();
        result.insert(parts[1], parts[0]);
    }
    result
}

fn run_tests() {
    assert_eq!(
        42,
        run_1(&vec!["COM)B","B)C","C)D","D)E","E)F","B)G","G)H","D)I","E)J","J)K","K)L"].into_iter().map(|s| s.to_string()).collect())
    );

    assert_eq!(
        4,
        run_2(&vec!["COM)B","B)C","C)D","D)E","E)F","B)G","G)H","D)I","E)J","J)K","K)L", "K)YOU","I)SAN"].into_iter().map(|s| s.to_string()).collect())
    );
}

pub fn run() {
    run_tests();
    let lines = read_inputs(6);

    let result_1 = run_1(&lines);
    println!("Result 1: {}", result_1);

    let result_2 = run_2(&lines);
    println!("Result 2: {}", result_2);
}
