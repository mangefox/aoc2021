# day 2a

### rust
```rust
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
```

### kotlin
```kotlin
val input = File("input/02.txt").readLines()

var pos = 0
var depth = 0
for (line in input) {
    val (dir, value) = line.split(" ")
    when (dir) {
        "up" -> depth -= value.toInt()
        "down" -> depth += value.toInt()
        "forward" -> pos += value.toInt()
    }
}

println(pos * depth) // 1989014
```

### python
```python
with open("input/day02") as file:
    pos = 0
    depth = 0
    for line in file.readlines():
        dir, value = line.split(" ")
        match dir:
            case "up":
                depth -= int(value)
            case "down":
                depth += int(value)
            case "forward":
                pos += int(value)

    print(pos * depth)  # 1989014
```

# day 2b

### rust
```rust
let content: String = fs::read_to_string("input/day02").unwrap();

let mut pos = 0;
let mut depth = 0;
let mut aim = 0;

for line in content.lines() {
    let tokens: Vec<&str> = line.split(" ").collect();
    let (dir, value) = (tokens[0], tokens[1].parse::<i32>().unwrap());
    match dir {
        "up" => aim -= value,
        "down" => aim += value,
        "forward" => {
            pos += value;
            depth += aim * value;
        },
        _ => panic!("eek")
    }
}

println!("day02b: {}", pos * depth); // 2006917119
```

### kotlin
```kotlin
val input = File("input/02.txt").readLines()

var pos = 0
var depth = 0
var aim = 0
for (line in input) {
    val (dir, value) = line.split(" ")
    when (dir) {
        "up" -> aim -= value.toInt()
        "down" -> aim += value.toInt()
        "forward" -> {
            pos += value.toInt()
            depth += aim * value.toInt()
        }
    }
}

println(pos * depth) // 2006917119
```

### python
```python
with open("input/day02") as file:
    pos = 0
    depth = 0
    aim = 0
    for line in file.readlines():
        dir, value = line.split(" ")
        match dir:
            case "up":
                aim -= int(value)
            case "down":
                aim += int(value)
            case "forward":
                pos += int(value)
                depth += aim * int(value)

    print(pos * depth)  # 2006917119
```