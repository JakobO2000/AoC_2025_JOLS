pub fn part1(input: &str) -> i32 {
    let mut char_int: i32;
    let mut jolt: i32 = 0;

    for line in input.lines() {
        let mut max_left: i32 = 0;
        let mut max_right: i32 = 0;

        let len = line.chars().count();

        for (i, char) in line.chars().enumerate() {
            char_int = char.to_digit(10).unwrap() as i32;

            let is_not_last = i + 1 < len;

            if is_not_last && char_int > max_left {
                max_left = char_int;
                max_right = 0;
                continue;
            } else if char_int > max_right {
                max_right = char_int;
            }
        }
        jolt += max_left * 10 + max_right;
    }

    jolt
}

pub fn part2(input: &str) -> u64 {
    let mut jolt: u64 = 0;
    let digits = 12;

    for line in input.lines() {
        let char_int: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let n = char_int.len();
        let mut stack: Vec<u32> = Vec::new();

        for (i, &d) in char_int.iter().enumerate() {
            while let Some(&last) = stack.last() {
                if d > last && stack.len() + (n - i) > digits {
                    stack.pop();
                } else {
                    break;
                }
            }

            if stack.len() < digits {
                stack.push(d);
            }
        }

        let mut number: u64 = 0;
        for &d in &stack {
            number = number * 10 + d as u64;
        }

        jolt += number;
    }

    jolt
}

pub fn run(input: &str) {
    println!("Day 1:");
    println!("  Part 1: {}", part1(input));
    println!("  Part 2: {}", part2(input));
}
