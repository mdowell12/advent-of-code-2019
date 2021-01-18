use std::fs;

pub fn read_inputs(day: i8) -> Vec<String> {
    let path = format!("./inputs/{}.txt", day);
    let input = fs::read_to_string(path)
        .expect(&format!("Something went wrong reading the file for day {}", day).to_string());

    input.split("\n").map(String::from).filter(|s| s.len() > 0).collect()
}
