use std::fs;

// iterate over sliding window of size 2, count number of times the right-most number is larger than the left-most
pub fn run() {
    let content: String = fs::read_to_string("input/day01").unwrap();
    let numbers: Vec<i32> = content.lines().map(|l| l.parse().unwrap()).collect();

    let mut increases = 0;
    for n in numbers.as_slice().windows(2) {
        if n[0] < n[1] {
            increases += 1;
        }
    }
    println!("day01a: {}", increases); // 1184
}