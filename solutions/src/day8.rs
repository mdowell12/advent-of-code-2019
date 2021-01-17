use std::collections::HashMap;

use crate::util::read_inputs;

fn run_1(input: &Vec<String>, width: usize, height: usize) -> usize {
    let (layers, counts) = parse_layers(input, width, height);

    let layer_with_least_zeros = counts.iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap();

    let num_ones = layers.get(layer_with_least_zeros).unwrap().iter()
        .flat_map(|i| i)
        .filter(|i| **i == 1)
        .count();

    let num_twos = layers.get(layer_with_least_zeros).unwrap().iter()
        .flat_map(|i| i)
        .filter(|i| **i == 2)
        .count();

    num_ones * num_twos
}

fn run_2(input: &Vec<String>, width: usize, height: usize) -> Vec<Vec<usize>> {
    let (layers, _) = parse_layers(input, width, height);
    let mut image = vec![];

    for col in 0..height {
        let mut row_values = vec![];
        for row in 0..width {
            for l in 0..layers.len() {
                let val_this_layer = layers.get(&l).unwrap()[col][row];
                if val_this_layer != 2 {
                    row_values.push(val_this_layer);
                    break;
                }
            }
        }
        image.push(row_values);
    }
    image
}

fn parse_layers(input: &Vec<String>, width: usize, height: usize)
    -> (HashMap<usize, Vec<Vec<usize>>>, HashMap<usize, usize>) {
    let mut ints = input.get(0)
        .unwrap()
        .chars()
        .map(|s| s.to_digit(10).unwrap() as usize)
        .into_iter()
        .peekable();

    let mut layers: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
    let mut counts: HashMap<usize, usize> = HashMap::new();

    let mut layer = 0;

    while ints.peek().is_some() {
        let mut num_zeros = 0;
        let mut cols = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                let next = ints.next().unwrap();
                if next == 0 {
                    num_zeros += 1;
                }
                row.push(next);
            }
            cols.push(row);
        }
        layers.insert(layer, cols);
        counts.insert(layer, num_zeros);

        layer += 1;
    }

    (layers, counts)
}

fn run_tests() {
    assert_eq!(1, run_1(&vec!["123456789012".to_string()], 3, 2));

    assert_eq!(vec![vec![0, 1], vec![1, 0]], run_2(&vec!["0222112222120000".to_string()], 2, 2));
}

pub fn run() {
    run_tests();
    let lines = read_inputs(8);

    let result_1 = run_1(&lines, 25, 6);
    println!("Result 1: {}", result_1);

    let result_2 = run_2(&lines, 25, 6);
    println!("Result 2: {:?}", result_2);

    // Print the resulting image
    for line in result_2 {
        let s: String = line
            .into_iter()
            .map(|d| if d == 1 { "X" } else { " " })
            .collect();
        println!("{}", s);
    }
}
