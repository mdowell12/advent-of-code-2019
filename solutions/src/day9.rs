
use crate::intcode::ExitCode;
use crate::intcode::parse_input;
use crate::intcode::run_intcode;
use crate::util::read_inputs;

fn run_1(input: &Vec<String>) -> i64 {
    let program = parse_input(input);
    0
}

fn run_2(input: &Vec<String>) -> i64 {
    0
}

fn run_tests() {
    assert_eq!(
        (vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], vec![], 1, ExitCode::FINISHED),
        run_intcode(&vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], &vec![], 0)
    );
    println!("test 1 passed");

    assert_eq!(
        (vec![1102,34915192,34915192,7,4,7,99,0], vec![], 1, ExitCode::FINISHED),
        run_intcode(&vec![1102,34915192,34915192,7,4,7,99,0], &vec![], 0)
    );

    assert_eq!(
        (vec![104,1125899906842624,99], vec![1125899906842624], 1, ExitCode::FINISHED),
        run_intcode(&vec![104,1125899906842624,99], &vec![], 0)
    );

    assert_eq!(50346, run_2(&vec!["100756".to_string()]));
}

pub fn run() {
    run_tests();
    let lines = read_inputs(9);

    let result_1 = run_1(&lines);
    println!("Result 1: {}", result_1);

    let result_2 = run_2(&lines);
    println!("Result 2: {}", result_2);
}
