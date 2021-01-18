
use crate::intcode::ExitCode;
use crate::intcode::parse_input;
use crate::intcode::run_intcode;
use crate::util::read_inputs;

fn run_1(input: &Vec<String>) -> i64 {
    run_boost(input, 1)
}

fn run_2(input: &Vec<String>) -> i64 {
    run_boost(input, 2)
}

fn run_boost(input: &Vec<String>, arg: i64) -> i64 {
    let program = parse_input(input);
    let (_, outputs, _, _) = run_intcode(&program, &vec![arg], 0);
    outputs[0]
}

fn run_tests() {
    assert_eq!(
        (vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], 15, ExitCode::FINISHED),
        run_intcode(&vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], &vec![], 0)
    );

    assert_eq!(
        (vec![1102, 34915192, 34915192, 7, 4, 7, 99, 1219070632396864], vec![1219070632396864], 6, ExitCode::FINISHED),
        run_intcode(&vec![1102,34915192,34915192,7,4,7,99,0], &vec![], 0)
    );

    assert_eq!(
        (vec![104,1125899906842624,99], vec![1125899906842624], 2, ExitCode::FINISHED),
        run_intcode(&vec![104,1125899906842624,99], &vec![], 0)
    );
}

pub fn run() {
    run_tests();
    let lines = read_inputs(9);

    let result_1 = run_1(&lines);
    println!("Result 1: {}", result_1);

    let result_2 = run_2(&lines);
    println!("Result 2: {}", result_2);
}
