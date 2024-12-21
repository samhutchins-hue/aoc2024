#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0), // up
    (1, 0), // down
    (0, -1), // left
    (0, 1), // right
    (-1, -1), // up left
    (-1, 1), // up right
    (1, -1), // down left
    (1, 1), // down right
];

#[aoc(day4, part1)]
pub fn solve_part1(grid: &[Vec<char>]) -> u32 {

    let target = ['X', 'M', 'A', 'S'];


    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            count += word_search(row, col, &grid, &target);
        }
    }
    count
}

fn word_search(row: usize, col: usize, grid: &[Vec<char>], target: &[char]) -> u32 {
    let mut count = 0;
    if grid[row][col] == target[0] {

        for (dx, dy) in DIRECTIONS {
            let mut found = true;

            for i in 1..target.len() {

                let new_row = row as isize + dx * i as isize;
                let new_col = col as isize + dy * i as isize;

                if new_row < 0 || new_row as usize >= grid.len() || new_col < 0 || new_col as usize >= grid[0].len() {
                    found = false;
                    break;
                }

                if target[i] != grid[new_row as usize][new_col as usize] {
                    found = false;
                    break;
                }
            }

            if found {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day4, part2)]
pub fn solve_part2(grid: &[Vec<char>]) -> u32 {
    let mut count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if has_mas(&grid, row, col) {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day4, part2, simple_brute_force)]
pub fn brute_force_part2(grid: &Vec<Vec<char>>) -> u32 {
    let n = grid.len();
    let m = grid[0].len();

    let mut count = 0;

    for i in 1..n - 1 {
        for j in 1..m - 1 {
            let nw_se = [grid[i - 1][j - 1], grid[i][j], grid[i + 1][j + 1]];
            let nw_se_matches = (nw_se[0] == 'M' && nw_se[1] == 'A' && nw_se[2] == 'S')
                || (nw_se[0] == 'S' && nw_se[1] == 'A' && nw_se[2] == 'M');

            let ne_sw = [grid[i + 1][j - 1], grid[i][j], grid[i - 1][j + 1]];
            let ne_sw_matches = (ne_sw[0] == 'M' && ne_sw[1] == 'A' && ne_sw[2] == 'S')
                || (ne_sw[0] == 'S' && ne_sw[1] == 'A' && ne_sw[2] == 'M');

            if nw_se_matches && ne_sw_matches {
                count += 1;
            }
        }
    }
    count
}

fn has_mas(grid: &[Vec<char>], row: usize, col: usize) -> bool {

    if !(1 <= row && row < grid.len() - 1 && 1 <= col && col < grid[0].len() - 1) {
        return false;
    }

    if grid[row][col] != 'A' {
        return false;
    }

    let diag_1 = format!("{}{}",
                         grid[row - 1][col - 1],
                         grid[row + 1][col + 1]);
    let diag_2 = format!("{}{}",
                         grid[row - 1][col + 1],
                         grid[row + 1][col - 1]);

    let valid_patterns = ["MS", "SM"];

    valid_patterns.contains(&diag_1.as_str()) && valid_patterns.contains(&diag_2.as_str())
}
