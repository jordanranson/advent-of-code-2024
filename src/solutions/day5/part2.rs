use std::collections::{HashMap, HashSet, VecDeque};

fn topological_sort (rules: &[Vec<i32>], pages: &[i32]) -> Vec<i32> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut edge_degree: HashMap<i32, usize> = HashMap::new();

    // build the graph
    for rule in rules {
        let (a, b) = (rule[0], rule[1]);
        if pages.contains(&a) && pages.contains(&b) {
            graph.entry(a).or_default().insert(b);
            *edge_degree.entry(b).or_default() += 1;
            edge_degree.entry(a).or_default();
        }
    }

    // find nodes with no incoming edges
    let mut queue: VecDeque<i32> = edge_degree
        .iter()
        .filter_map(|(&node, &deg)| if deg == 0 { Some(node) } else { None })
        .collect();

    // perform topological sorting
    let mut sorted = Vec::new();
    while let Some(node) = queue.pop_front() {
        sorted.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(deg) = edge_degree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    sorted
        .into_iter()
        .filter(|page| pages.contains(page))
        .collect()
}

fn parse_input (char: char, input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(char).map(|x| x.trim().parse::<i32>().unwrap()).collect())
        .collect()
}

fn is_valid_update (rules: &[Vec<i32>], pages: &[i32]) -> bool {
    for rule in rules {
        let (page_a, page_b) = (rule[0], rule[1]);
        let index_a = pages.iter().position(|&p| p == page_a);
        let index_b = pages.iter().position(|&p| p == page_b);

        if let (Some(a), Some(b)) = (index_a, index_b) {
            if a > b {
                return false;
            }
        }
    }
    true
}

pub fn solution (input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules = parse_input('|', parts[0]);
    let updates = parse_input(',', parts[1]);

    updates
        .iter()
        .filter(|pages| !is_valid_update(&rules, pages))
        .map(|update| topological_sort(&rules, update))
        .map(|pages| pages[pages.len() / 2])
        .sum::<i32>()
        .to_string()
}
