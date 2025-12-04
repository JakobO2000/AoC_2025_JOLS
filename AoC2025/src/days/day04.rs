pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut accessible = 0;

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }

            let mut count = 0;

            for (dr, dc) in directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nr < rows as isize &&
                   nc >= 0 && nc < cols as isize {

                    if grid[nr as usize][nc as usize] == '@' {
                        count += 1;
                    }
                }
            }

            if count < 4 {
                accessible += 1;
            }
        }
    }

    accessible
}

pub fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    if grid.is_empty() { return 0; }

    let rows = grid.len();
    let cols = grid[0].len();

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    let mut total_removed = 0usize;

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != '@' { continue; }

                let mut adj = 0;
                for (dr, dc) in directions {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                        if grid[nr as usize][nc as usize] == '@' {
                            adj += 1;
                        }
                    }
                }

                if adj < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (r, c) in to_remove {
            grid[r][c] = '.';
            total_removed += 1;
        }
    }

    total_removed
}


pub fn run(input: &str) {
    println!("Day 1:");
    println!("  Part 1: {}", part1(input));
    println!("  Part 2: {}", part2(input));
}
