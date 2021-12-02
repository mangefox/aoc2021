use std::fs;

pub fn run() {
    let content: String = fs::read_to_string("input/day01").unwrap();
    let numbers: Vec<i32> = content.lines().map(|l| l.parse().unwrap()).collect();

    let mut increases = 0;
    for n in numbers.as_slice().windows(4) {
        if n[0] < n[3] {
            increases += 1;
        }
    }
    println!("day01b: {}", increases); // 1158
}