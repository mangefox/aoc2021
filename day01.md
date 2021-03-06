# day 1a

### rust
```rust
let content: String = fs::read_to_string("input/day01").unwrap();
let numbers: Vec<i32> = content.lines().map(|l| l.parse().unwrap()).collect();

let mut increases = 0;
for n in numbers.as_slice().windows(2) {
    if n[0] < n[1] {
        increases += 1;
    }
}
println!("day01a: {}", increases); // 1184
```

### kotlin
```kotlin
val input = File("input/01.txt").readLines().toInts()

val increases = input.windowed(2).count { (i, j) -> j > i }

println(increases) // 1184
```

### python
```python
with open("input/day01") as file:
    numbers = list(map(lambda x: int(x), file.readlines()))

    increases = 0
    for (i, n) in enumerate(numbers):
        if i > 0 and n > numbers[i-1]:
            increases += 1

    print(increases)  # 1184
```

# day 1b

### rust
```rust
let content: String = fs::read_to_string("input/day01").unwrap();
let numbers: Vec<i32> = content.lines().map(|l| l.parse().unwrap()).collect();

let mut increases = 0;
for n in numbers.as_slice().windows(4) {
    if n[0] < n[3] {
        increases += 1;
    }
}
println!("day01b: {}", increases); // 1158
```

### kotlin
```kotlin
val input = File("input/01.txt").readLines().toInts()

val increases = input.windowed(4).count { (i, _, _, j) -> j > i }

println(increases) // 1158
```

### python
```python
with open("input/day01") as file:
    numbers = list(map(lambda x: int(x), file.readlines()))

    increases = 0
    for (i, n) in enumerate(numbers):
        if i > 2 and n > numbers[i-3]:
            increases += 1

    print(increases)  # 1158
```