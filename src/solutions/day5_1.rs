
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
        .filter(|pages| check_if_valid(pages))
        .map(|pages| pages[pages.len() / 2])
        .sum::<i32>()
        .to_string()
}
