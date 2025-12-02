pub fn part1(input: &str) -> u64 {
    let ranges = parse_ranges(input);

    let mut sum_invalid_ids: u64 = 0;

    for (start, end) in ranges {

        let len_start = num_digits(start);
        let len_end = num_digits(end);

        if len_start % 2 == 1 && len_end % 2 == 1 {
            if len_start == len_end {
                continue;
            }
        }

        for id in start..=end {
            if is_invalid_id(id) {
                sum_invalid_ids += id;
            }
        }
    }

    sum_invalid_ids
}

fn num_digits(mut n: u64) -> usize {
    if n == 0 { return 1; }
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

pub fn is_invalid_id(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    let (left, right) = s.split_at(half);

    left == right
}


pub fn part2(input: &str) -> u64 {
    let ranges = parse_ranges(input);

    let mut sum_invalid_ids: u64 = 0;

    for (start, end) in ranges {
        for id in start..=end {
            if is_invalid_id_part2(id) {
                sum_invalid_ids += id;
            }
        }
    }

    sum_invalid_ids
}

pub fn is_invalid_id_part2(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();

    for block_size in 1..=len / 2 {
        if len % block_size != 0 {
            continue;
        }

        let block = &s[..block_size];

        let mut valid = true;
        for chunk_start in (0..len).step_by(block_size) {
            if &s[chunk_start..chunk_start + block_size] != block {
                valid = false;
                break;
            }
        }

        if valid {
            return true;
        }
    }

    false
}

pub fn parse_ranges(input: &str) -> Vec<(u64,u64)> {
    input.trim().split(',').map(|range| {
        let (start, end) = range.split_once('-').unwrap();
        (
            start.parse::<u64>().unwrap(),
            end.parse::<u64>().unwrap(),
        )
    }).collect()
}

pub fn run(input: &str) {
    println!("Day 2:");
    println!("  Part 1: {}", part1(input)); 
    println!("  Part 2: {}", part2(input)); 

}
