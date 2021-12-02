use std::fs;

pub fn run() {
    let content: String = fs::read_to_string("input/day02").unwrap();

    let mut pos = 0;
    let mut depth = 0;

    for line in content.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        let (dir, value) = (tokens[0], tokens[1].parse::<i32>().unwrap());
        match dir {
            "up" => depth -= value,
            "down" => depth += value,
            "forward" => pos += value,
            _ => panic!("eek")
        }
    }

    println!("day02a: {}", pos * depth); // 1989014
}