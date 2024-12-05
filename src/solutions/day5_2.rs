use std::collections::{HashMap, HashSet, VecDeque};

fn topological_sort (rules: &Vec<Vec<i32>>, pages: &Vec<i32>) -> Vec<i32> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut edge_degree: HashMap<i32, usize> = HashMap::new();

    // build the graph
    for rule in rules {
        let (a, b) = (rule[0], rule[1]);
        if !pages.contains(&a) || !pages.contains(&b) {
            continue;
        }

        graph.entry(a).or_default().insert(b);
        *edge_degree.entry(b).or_default() += 1;
        edge_degree.entry(a).or_default();
    }

    // find nodes with no incoming edges
    let mut queue: VecDeque<i32> = edge_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&node, _)| node)
        .collect();

    // sort the nodes by their incoming edges until no edges remain
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

pub fn solution (input: &str) -> String {
    let parts = input
        .split("\n\n")
        .collect::<Vec<&str>>();

    let page_order_rules: Vec<Vec<i32>> = parts
        .get(0)
        .unwrap()
        .lines()
        .map(|x| 
            x
                .split("|")
                .map(|y| 
                    y
                        .trim()
                        .parse::<i32>()
                        .unwrap()
                )
                .collect()
        )
        .collect();

    let pages_per_update: Vec<Vec<i32>> = parts
        .get(1)
        .unwrap()
        .lines()
        .map(|x| 
            x
                .split(",")
                .map(|y| 
                    y
                        .trim()
                        .parse::<i32>()
                        .unwrap()
                )
                .collect()
        )
        .collect();

    let check_if_valid = |pages: &Vec<i32>| {
        for rule in &page_order_rules {
            let (page_a, page_b) = (rule[0], rule[1]);
    
            let mut index_a = None;
            let mut index_b = None;
    
            for (index, &page) in pages.iter().enumerate() {
                if page == page_a {
                    index_a = Some(index);
                }
                if page == page_b {
                    index_b = Some(index);
                }
    
                if index_a.is_some() && index_b.is_some() {
                    break;
                }
            }
    
            if let (Some(a), Some(b)) = (index_a, index_b) {
                if a > b {
                    return false;
                }
            }
        }
    
        true
    };

    pages_per_update
        .iter()
        .filter(|pages| !check_if_valid(pages))
        .map(|update| topological_sort(&page_order_rules, &update))
        .map(|pages| pages[pages.len() / 2])
        .sum::<i32>()
        .to_string()
}
