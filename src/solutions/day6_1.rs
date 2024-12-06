pub fn solution(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let (mut x, mut y) = grid.iter().enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == '^').map(|j| (j as i32, i as i32)))
        .unwrap_or((0, 0));

    let mut positions = vec![(x, y)];
    let mut cur_direction = 0;
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // up, right, down, left

    loop {
        let (dx, dy) = directions[cur_direction];
        let next_x = x + dx;
        let next_y = y + dy;

        if next_x < 0 || next_y < 0 || next_x >= grid[0].len() as i32 || next_y >= grid.len() as i32 {
            break;
        }

        if grid[next_y as usize][next_x as usize] == '#' {
            cur_direction = (cur_direction + 1) % 4;
        } else {
            x = next_x;
            y = next_y;
            if !positions.contains(&(x, y)) {
                positions.push((x, y));
            }
            
        }
    }

    positions.len().to_string()
}
