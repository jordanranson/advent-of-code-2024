fn find_xmas (puzzle: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let xmas = ['X', 'M', 'A', 'S'];
    let directions = [
        (-1, 0),  // Up
        (1, 0),   // Down
        (0, -1),  // Left
        (0, 1),   // Right
        (-1, -1), // Up-left
        (-1, 1),  // Up-right
        (1, -1),  // Down-left
        (1, 1),   // Down-right
    ];

    fn check_direction(
        puzzle: &Vec<Vec<char>>,
        x: usize,
        y: usize,
        xmas: &[char],
        dx: isize,
        dy: isize,
    ) -> bool {
        for i in 0..xmas.len() {
            let nx = x as isize + i as isize * dx;
            let ny = y as isize + i as isize * dy;

            if ny < 0
                || ny >= puzzle.len() as isize
                || nx < 0
                || nx >= puzzle[ny as usize].len() as isize
            {
                return false;
            }

            if puzzle[ny as usize][nx as usize] != xmas[i] {
                return false;
            }
        }
        true
    }

    directions
        .iter()
        .filter(|&&(dx, dy)| check_direction(puzzle, x, y, &xmas, dx, dy))
        .count() as i32
}

pub fn solution (input: &str) -> String {
    let puzzle: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let xmas_count = puzzle.iter().enumerate().map(|(y, row)| {
        row.iter().enumerate().map(|(x, _)| {
            if puzzle[y][x] == 'X' {
                return find_xmas(&puzzle, x, y);
            }
            0
        }).sum::<i32>()
    }).sum::<i32>();

    xmas_count.to_string()
}
