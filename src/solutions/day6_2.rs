fn find_obstruction(grid: Vec<Vec<char>>, position: (i32, i32), obstruction: (i32, i32)) -> bool {
    let (mut x, mut y) = position;
    let start_position = position; // Keep track of the starting position

    let mut cur_direction = 0;
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // Left, Down, Right, Up

    let mut steps = 0;

    loop {
        if steps > grid.len() * grid[0].len() {
            return true;
        }

        let (dx, dy) = directions[cur_direction];
        let next_x = x + dx;
        let next_y = y + dy;

        // Check if next position is out of bounds
        if next_x < 0 || next_y < 0 || next_x >= grid[0].len() as i32 || next_y >= grid.len() as i32 {
            return false;
        }

        // Check if next position hits an obstruction or obstacle
        if grid[next_y as usize][next_x as usize] == '#' || (next_x, next_y) == obstruction {
            cur_direction = (cur_direction + 1) % 4; // Change direction clockwise
        } else {
            x = next_x;
            y = next_y;
        }

        steps += 1;
    }

    true
}
pub fn solution(input: &str) -> String {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (x, y) = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|&c| c == '^')
                .map(|j| (j as i32, i as i32))
        })
        .unwrap_or((0, 0));

    let mut num_found_obstructions = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let obstruction = (j as i32, i as i32);

            if find_obstruction(grid.clone(), (x, y), obstruction) {
                num_found_obstructions += 1;
            }
        }
    }

    num_found_obstructions.to_string()
}
