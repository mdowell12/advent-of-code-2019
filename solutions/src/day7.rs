use std::collections::HashMap;

use itertools::Itertools;

use crate::intcode::ExitCode;
use crate::intcode::parse_input;
use crate::intcode::run_intcode;
use crate::util::read_inputs;

fn run_1(input: &Vec<String>) -> i32 {
    let program = parse_input(input);
    let phase_permutations = (0..5).permutations(5);

    let mut max_output = 0;

    for phases in phase_permutations {

        let mut next_input = 0;
        for phase in phases {
            let (_, output, _) = run_intcode(&program, &vec![phase, next_input]);
            next_input = output[0];
        }
        if next_input > max_output {
            max_output = next_input;
        }

    }
    max_output
}

fn run_2(input: &Vec<String>) -> i32 {
    let program = parse_input(input);
    let phase_permutations = (5..10).permutations(5);

    let mut max_output = 0;

    for phases in phase_permutations {
        // let phases = vec![9,8,7,6,5];

        let mut amplifiers: HashMap<usize, Vec<i32>> = HashMap::new();
        amplifiers.insert(0, program.clone());
        amplifiers.insert(1, program.clone());
        amplifiers.insert(2, program.clone());
        amplifiers.insert(3, program.clone());
        amplifiers.insert(4, program.clone());

        let mut next_input = 0;
        let mut i = 0;
        loop {
            let amplifier = i % phases.len();
            let phase = phases[amplifier];
            let amplifier_program = amplifiers.get(&amplifier).unwrap();

            let (next_program, output, exit_code) = run_intcode(&amplifier_program, &vec![phase, next_input]);
            println!("{:?} {:?} {:?} {:?}", i, phase, output, next_input);

            next_input = output[0];

            if (exit_code == ExitCode::FINISHED) & (amplifier == 4) {
                // Is final amplifier and has exited
                break;
            }

            amplifiers.insert(amplifier, next_program);
            i += 1;
        }

        if next_input > max_output {
            max_output = next_input;
        }
    }

    max_output
}

fn run_tests() {
    assert_eq!(43210, run_1(&vec!["3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string()]));
    assert_eq!(54321, run_1(&vec!["3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0".to_string()]));
    assert_eq!(65210, run_1(&vec!["3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0".to_string()]));

    assert_eq!(139629729, run_2(&vec!["3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5".to_string()]));
    assert_eq!(18216, run_2(&vec!["3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10".to_string()]));
}

pub fn run() {
    run_tests();
    let lines = read_inputs(7);

    let result_1 = run_1(&lines);
    println!("Result 1: {}", result_1);

    let result_2 = run_2(&lines);
    println!("Result 2: {}", result_2);
}
