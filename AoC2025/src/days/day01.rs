pub fn part1(input: &str) -> i32 {
    let mut dial = 50;
    let mut count = 0;
    let mut direction: i32;
    let mut number: i32;

    for line in input.lines() {
        direction = if line.starts_with('L') { -1 } else { 1 };
        
        number = line[1..].parse().unwrap();
        dial = ((dial + direction*number) as i32).rem_euclid(100);
        if dial == 0 {
            count += 1;
        }

    }

    count
}

pub fn part2(input: &str) -> i32 {
    let mut dial: i32 = 50;
    let mut count: i32 = 0;
    let mut direction: i32;
    for line in input.lines(){
        direction = if line.starts_with('L') { -1 } else { 1 };
        let rotations: i32 = line[1..].parse().unwrap();
        count += rotations / 100;
        let number = rotations % 100;
        let not_zero = dial !=0;
        if direction == -1 {
            dial -= number;
        }
        else {
            dial += number;
        }

        if (dial <= 0 && not_zero) || dial > 99{
            count +=1
        }
        dial = dial.rem_euclid(100)

    }
    count
}

pub fn run(input: &str) {
    println!("Day 1:");
    println!("  Part 1: {}", part1(input)); 
    println!("  Part 2: {}", part2(input)); 
}
