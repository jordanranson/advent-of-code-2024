use std::collections::HashSet;

fn find_obstruction (grid: &[Vec<char>], position: (i32, i32), obstruction: (i32, i32)) -> bool {
    let (mut x, mut y) = position;

    let mut cur_direction = 0;
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // up, right, down, left

    let mut visited = HashSet::new();

    let grid_width = grid[0].len() as i32;
    let grid_height = grid.len() as i32;

    loop {
        let (dx, dy) = directions[cur_direction];
        let next_x = x + dx;
        let next_y = y + dy;

        if next_x < 0 || next_y < 0 || next_x >= grid_width || next_y >= grid_height {
            return false;
        }

        if grid[next_y as usize][next_x as usize] == '#' || (next_x, next_y) == obstruction {
            if !visited.insert((cur_direction, x, y)) {
                return true;
            }
            cur_direction = (cur_direction + 1) % 4;
        } else {
            x = next_x;
            y = next_y;
        }
    }
}

pub fn solution (input: &str) -> String {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut x = 0;
    let mut y = 0;
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '^') {
            x = j as i32;
            y = i as i32;
            break;
        }
    }

    let num_infinite_loops = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, _)| (i, j)))
        .filter(|&(i, j)| {
            grid[i][j] != '#' && find_obstruction(&grid, (x, y), (j as i32, i as i32))
        })
        .count();

    num_infinite_loops.to_string()
}
