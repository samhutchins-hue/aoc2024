pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    let mut new_vector: Vec<Vec<u32>> = vec![];

    for line in input.lines() {
        let row: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        new_vector.push(row);
    }

    new_vector
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let reports: Vec<Vec<u32>> = input_generator(input);
    let mut safe_count = 0;

    for report in reports {
        if is_safe(&report) {
            safe_count += 1;
        }
    }

    safe_count
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let reports: Vec<Vec<u32>> = input_generator(input);
    let mut safe_count = 0;

    for report in reports {
        if is_safe_updated(&report) {
            safe_count += 1;
        }
    }

    safe_count
}

fn is_safe(report: &Vec<u32>) -> bool {

    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..report.len() - 1 {
        let diff: i32 = (report[i] as i32 - report[i + 1] as i32).abs();

        if diff < 1 || diff > 3 {
            return false;
        }

        if report[i] < report[i + 1] {
            decreasing = false;
        }

        if report[i] > report[i + 1] {
            increasing = false;
        }
    }

    increasing || decreasing
}

fn is_safe_updated(report: &Vec<u32>) -> bool {

    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);

        if is_safe(&modified_report) {
            return true;
        }
    }

    false
}
