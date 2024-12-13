use std::collections::{HashMap, HashSet};

pub fn solution (input: &str) -> String {
    let mut antennae: HashMap<char, Vec<(f64, f64)>> = HashMap::new();

    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennae
                    .entry(c)
                    .or_insert(Vec::new())
                    .push((x as f64, y as f64));
            }
        }
    }

    let mut antinodes: HashSet<(u64, u64)> = HashSet::new();

    for (_, frequency) in antennae.iter() {
        for point_a in frequency.iter() {
            for point_b in frequency.iter() {
                if point_a == point_b {
                    continue;
                }

                let (x1, y1) = *point_a;
                let (x2, y2) = *point_b;

                let dist = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
                let angle = (y2 - y1).atan2(x2 - x1);

                let antinode_a_x = x1 - dist * angle.cos();
                let antinode_a_y = y1 - dist * angle.sin();
                if antinode_a_x >= 0.0
                    && antinode_a_y >= 0.0
                    && antinode_a_x < width as f64
                    && antinode_a_y < height as f64
                {
                    antinodes.insert((antinode_a_x.round() as u64, antinode_a_y.round() as u64));
                }

                let antinode_b_x = x2 + dist * angle.cos();
                let antinode_b_y = y2 + dist * angle.sin();
                if antinode_b_x >= 0.0
                    && antinode_b_y >= 0.0
                    && antinode_b_x < width as f64
                    && antinode_b_y < height as f64
                {
                    antinodes.insert((antinode_b_x.round() as u64, antinode_b_y.round() as u64));
                }
            }
        }
    }

    antinodes.len().to_string()
}
