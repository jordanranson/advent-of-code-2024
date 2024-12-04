use std::char;

fn find_xmas(puzzle: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let directions = [
        (-1, -1), // Up-left
        (-1, 1),  // Up-right
    ];

    fn check_cross(
        puzzle: &Vec<Vec<char>>,
        x: usize,
        y: usize,
        dx: isize,
        dy: isize,
    ) -> bool {
        let mut sum = 0;
        let target = 'S' as i32 + 'M' as i32;

        for i in [-1, 1] {
            let nx = x as isize + dx * i;
            let ny = y as isize + dy * i;

            if ny < 0
                || ny >= puzzle.len() as isize
                || nx < 0
                || nx >= puzzle[ny as usize].len() as isize
            {
                return false;
            }

            sum += puzzle[ny as usize][nx as usize] as i32
        }

        sum == target
    }

    directions
        .iter()
        .all(|&(dx, dy)| check_cross(puzzle, x, y, dx, dy))
}

pub fn solution (input: &str) -> String {
    let puzzle: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let xmas_count = puzzle.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, _)| {
            if puzzle[y][x] == 'A' {
                return find_xmas(&puzzle, x, y) as i32;
            }
            0
        }).sum::<i32>()
    }).sum::<i32>();

    xmas_count.to_string()
}
