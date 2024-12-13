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

fn compact_disk (mut disk: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut free_indices = Vec::new();
    let mut file_indices = Vec::new();

    for (i, block) in disk.iter().enumerate() {
        if block.is_none() {
            free_indices.push(i);
        }
    }

    for (i, block) in disk.iter().enumerate() {
        if block.is_some() {
            file_indices.push(i);
        }
    }

    // move blocks by pairing the leftmost free space with the rightmost file block 
    // if there is a gap (free_index < file_index) then update both free and file positions
    let mut free_ptr = 0;
    let mut file_ptr = file_indices.len().saturating_sub(1);

    while free_ptr < free_indices.len() && !file_indices.is_empty() {
        if file_ptr >= file_indices.len() {
            break;
        }

        let free_pos = free_indices[free_ptr];
        let file_pos = file_indices[file_ptr];

        if free_pos < file_pos {
            // move the file block to the free space
            let file_id = disk[file_pos].unwrap();
            disk[free_pos] = Some(file_id);
            disk[file_pos] = None;

            // use up the position we just filled
            free_ptr += 1;
            if file_ptr == 0 {
                // no more file positions to the left
                break;
            } else {
                file_ptr -= 1;
            }
        } else {
            // if free_pos is not before file_pos then we're done
            break;
        }
    }

    disk
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
    let (disk, _) = parse_disk_map(input);
    let compacted = compact_disk(disk);
    let checksum = compute_checksum(&compacted);
    checksum.to_string()
}
