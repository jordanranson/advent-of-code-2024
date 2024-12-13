fn count_distinct_trails (
    grid: &Vec<Vec<u32>>, 
    cur_row: usize, 
    cur_col: usize, 
    visited: &mut Vec<Vec<bool>>, 
) -> u64 {
    if cur_row >= grid.len() || cur_col >= grid[0].len() || visited[cur_row][cur_col] {
        return 0; 
    }

    visited[cur_row][cur_col] = true;

    // found end of trail
    if grid[cur_row][cur_col] == 9 {
        visited[cur_row][cur_col] = false; // backtrack for other paths
        return 1;
    }

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]; // right, left, down, up
    let mut trail_count = 0;
    
    for &(dir_row, dir_col) in &directions {
        let next_row = cur_row as isize + dir_row;
        let next_col = cur_col as isize + dir_col;
        
        if next_row >= 0 && next_row < grid.len() as isize && next_col >= 0 && next_col < grid[0].len() as isize {
            let next_row = next_row as usize;
            let next_col = next_col as usize;

            if grid[next_row][next_col] == grid[cur_row][cur_col] + 1 {
                trail_count += count_distinct_trails(grid, next_row, next_col, visited);
            }
        }
    }
    
    visited[cur_row][cur_col] = false; // backtrack for other paths

    trail_count
}

pub fn solution (input: &str) -> String {
    let map = input
        .lines()
        .map(|line| line.chars().map(|char| char.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let trailheads = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &cell)| cell == 0)
                .map(move |(x, _)| (x, y))
        })
        .collect::<Vec<_>>();

    let trailhead_ratings: Vec<u64> = trailheads
        .iter()
        .map(|&(x, y)| {
            let mut visited = vec![vec![false; map[0].len()]; map.len()];
            count_distinct_trails(&map, y, x, &mut visited)
        })
        .collect();

    trailhead_ratings
        .iter()
        .sum::<u64>()
        .to_string()
}
