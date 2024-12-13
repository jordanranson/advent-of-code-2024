use std::collections::{HashMap, HashSet};

fn gcd (a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn solution (input: &str) -> String {
    let mut antennae: HashMap<char, Vec<(u64, u64)>> = HashMap::new();

    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennae
                    .entry(c)
                    .or_insert(Vec::new())
                    .push((x as u64, y as u64));
            }
        }
    }

    let mut antinodes: HashSet<(u64, u64)> = HashSet::new();

    for (_, frequency) in antennae.iter() {
        for (i, &(x1, y1)) in frequency.iter().enumerate() {
            for &(x2, y2) in &frequency[i + 1..] {
                let dx = x2 as i64 - x1 as i64;
                let dy = y2 as i64 - y1 as i64;
                let gcd = gcd(dx.abs(), dy.abs());
                let step_x = dx / gcd;
                let step_y = dy / gcd;

                let mut x = x1 as i64;
                let mut y = y1 as i64;
                while x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height {
                    antinodes.insert((x as u64, y as u64));
                    x -= step_x;
                    y -= step_y;
                }

                let mut x = x2 as i64;
                let mut y = y2 as i64;
                while x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height {
                    antinodes.insert((x as u64, y as u64));
                    x += step_x;
                    y += step_y;
                }
            }
        }
    }

    for (_, positions) in antennae.iter() {
        for &antenna in positions {
            if positions.len() > 1 {
                antinodes.insert(antenna);
            }
        }
    }

    antinodes.len().to_string()
}
