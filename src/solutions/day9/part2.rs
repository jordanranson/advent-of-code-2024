fn parse_disk_map (input: &str) -> (Vec<Option<usize>>, usize) {
    let chars: Vec<_> = input.chars().collect();
    let mut index = 0;
    let mut file_id = 0;
    let mut disk = Vec::new();

    while index < chars.len() {
        let file_len = chars[index].to_digit(10).unwrap() as usize;
        index += 1;

        // fill in file blocks
        for _ in 0..file_len {
            disk.push(Some(file_id));
        }
        file_id += 1;

        // fill in free space
        if index < chars.len() {
            let free_len = chars[index].to_digit(10).unwrap() as usize;
            index += 1;

            for _ in 0..free_len {
                disk.push(None);
            }
        }
    }

    (disk, file_id)
}

fn move_file (disk: &mut [Option<usize>], file_id: usize) {
    // `disk` is a slice of `Option<usize>` where `Some(file_id)` means
    // that position is occupied by a block of the given file_id, and `None` means it is free
    let positions: Vec<usize> = disk.iter()
        .enumerate()
        .filter(|(_, &block)| block == Some(file_id))
        .map(|(i, _)| i)
        .collect();

    if positions.is_empty() {
        return;
    }

    let file_size = positions.len();
    let file_start = *positions.first().unwrap();
    let file_end = *positions.last().unwrap();

    // find a contiguous free span that can hold the file
    let mut best_span: Option<usize> = None; 
    let mut current_free_start = None;

    for i in 0..file_start {
        match disk[i] {
            None => {
                // empty space found so start a new run if one isn't already in progress
                if current_free_start.is_none() {
                    current_free_start = Some(i);
                }
            }
            Some(_) => {
                // end the current free run when an occupied block is found
                if let Some(start) = current_free_start {
                    let free_len = i - start; 

                    if free_len >= file_size {
                        best_span = Some(start);
                        break;
                    }
                    current_free_start = None;
                }
            }
        }
    }

    // if we ended before the file_start and were still tracking a free run then check that run too
    if current_free_start.is_some() {
        let start = current_free_start.unwrap();
        let free_len = file_start - start;

        if free_len >= file_size && best_span.is_none() {
            best_span = Some(start);
        }
    }

    // if we found a suitable span, move the file
    if let Some(free_start) = best_span {
        let target_start = free_start;
        let target_end = free_start + file_size - 1;

        // move the file
        for i in target_start..=target_end {
            disk[i] = Some(file_id);
        }

        // free up the old space
        for i in file_start..=file_end {
            disk[i] = None;
        }
    }
}

fn compute_checksum (disk: &[Option<usize>]) -> usize {
    let mut checksum = 0;
    for (i, block) in disk.iter().enumerate() {
        if let Some(file_id) = block {
            checksum += i * file_id;
        }
    }
    checksum
}

pub fn solution (input: &str) -> String {
    let (mut disk, file_count) = parse_disk_map(input);

    for file_id in (0..file_count).rev() {
        move_file(&mut disk, file_id);
    }

    let checksum = compute_checksum(&disk);
    checksum.to_string()
}
