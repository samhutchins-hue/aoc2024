use regex::Regex;
use std::error::Error;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut total_sum = 0;

    for cap in re.captures_iter(input) {
        let x: u32 = cap[1].parse()?;
        let y: u32 = cap[2].parse()?;
        let product = x * y;
        total_sum += product;
    }

    Ok(total_sum)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> Result<u32, Box<dyn Error>> {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    let do_re = Regex::new(r"do\(\)")?;
    let dont_re = Regex::new(r"don't\(\)")?;

    let mut total_sum = 0;
    let mut enabled = true;

    let token_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)")?;

    for token in token_re.find_iter(input) {
        let token_str = token.as_str();

        if do_re.is_match(token_str) {
            enabled = true;
        } else if dont_re.is_match(token_str) {
            enabled = false;
        } else if enabled {
            if let Some(cap) = mul_re.captures(token_str) {
                let x: u32 = cap[1].parse()?;
                let y: u32 = cap[2].parse()?;
                total_sum += x * y;
            }
        }
    }

    Ok(total_sum)
}

