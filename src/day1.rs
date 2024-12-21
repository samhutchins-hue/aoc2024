pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_column = vec![];
    let mut right_column = vec![];

    for line in input.lines() {
        let mut parts = line.split_whitespace().map(|d| d.parse().unwrap());
        left_column.push(parts.next().unwrap());
        right_column.push(parts.next().unwrap());
    }

    (left_column, right_column)
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let (mut left_column, mut right_column) = input_generator(input);

    left_column.sort_unstable();
    right_column.sort_unstable();

    let sum = left_column
        .iter()
        .zip(right_column)
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    sum
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let (mut left_column, mut right_column) = input_generator(input);

    left_column.sort_unstable();
    right_column.sort_unstable();

    //let mut similarity = 0;
    let mut similarity = 0;
    let mut i = 0;
    let mut j = 0;

    while i < left_column.len() {

        let current_num = left_column[i];
        let mut count = 0;

        while j < right_column.len() && right_column[j] < current_num {
            j += 1;
        }

        while j < right_column.len() && right_column[j] == current_num {
            count += 1;
            j += 1;
        }

        similarity += count * current_num;

        i += 1;
    }

    similarity

}
