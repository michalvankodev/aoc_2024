use std::{
    fs::{self},
    path::PathBuf,
};

fn main() -> Result<(), anyhow::Error> {
    let input_path_str = std::env::args().nth(1).expect("input path expected");

    let input_path = PathBuf::from(input_path_str);

    let contents = fs::read_to_string(input_path).expect("should be able to read file");
    let contents_iter = contents.chars();

    let mut drive: Vec<Option<usize>> = Vec::new();

    // Parse drive blocks
    let mut id = 0;
    let mut is_file = true;
    for ch in contents_iter {
        let repeats = ch.to_digit(10).unwrap_or(0) as usize;
        if is_file {
            let mut file_id_str = [Some(id)].repeat(repeats);
            drive.append(&mut file_id_str);
        } else {
            let mut empty_space_str = [None].repeat(repeats);
            drive.append(&mut empty_space_str);
            id += 1;
        }

        is_file = !is_file;
    }
    // let debug_drive_before_defragmentation = drive.iter().collect::<String>();
    // println!("{debug_drive_before_defragmentation}");

    // DeFragment drive blocks
    while id > 0 {
        // let debug_drive_after_defragmentation = drive
        //     .iter()
        //     .map(|block| match block {
        //         None => ".".to_owned(),
        //         Some(n) => n.to_string(),
        //     })
        //     .collect::<String>();
        // println!("id: {id} {debug_drive_after_defragmentation}");
        let drive_clone = drive.clone();
        let blocks_with_id = drive_clone.iter().enumerate().filter(|(_idx, block_id)| {
            if let Some(blck_id) = block_id {
                *blck_id == id
            } else {
                false
            }
        });
        // let blocks_count = blocks_with_id.clone().count();
        let Some(first) = blocks_with_id.clone().next() else {
            id -= 1;
            continue;
        };
        let last = blocks_with_id.clone().next_back().unwrap_or(first);

        let length = last.0 + 1 - first.0;

        if let Some(fittable_empty_block_start) = find_consecutive_nones_start(&drive, length) {
            if fittable_empty_block_start > first.0 {
                id -= 1;
                continue;
            }
            // println!("fittable_empty_block_start: {fittable_empty_block_start}");
            let blocks_to_replace: Vec<Option<usize>> =
                blocks_with_id.clone().map(|(_idx, block)| *block).collect();
            // println!("blocks_to_replace: {blocks_to_replace:?}");
            let empty_blocks: Vec<Option<usize>> = drive
                .splice(
                    fittable_empty_block_start..fittable_empty_block_start + length,
                    blocks_to_replace,
                )
                .collect();
            // println!("empty_blocks: {empty_blocks:?}");
            drive.splice(first.0..=last.0, empty_blocks);
        }

        id -= 1;
    }

    // let debug_drive_after_defragmentation = drive
    //     .iter()
    //     .map(|block| match block {
    //         None => ".".to_owned(),
    //         Some(n) => n.to_string(),
    //     })
    //     .collect::<String>();
    // println!("{debug_drive_after_defragmentation}");

    // Calculate checksum
    let checksum = drive.iter().enumerate().fold(0, |acc, (idx, block)| {
        acc + idx as u64 * block.unwrap_or(0) as u64
    });

    println!("Checksum: {checksum}");

    Ok(())
}

// fn advance_by<T: Iterator>(iter: &mut T, count: usize) {
//     for _ in 0..count {
//         iter.next();
//     }
// }

fn find_consecutive_nones_start(
    drive: &Vec<Option<usize>>,
    needed_consecutive_length: usize,
) -> Option<usize> {
    let iter = drive.iter().enumerate();

    let mut consecutive_length = 0;
    let mut start = None;

    for (idx, block) in iter {
        if block.is_none() {
            if consecutive_length == 0 {
                start = Some(idx);
            }
            consecutive_length += 1;
            if consecutive_length == needed_consecutive_length {
                return start;
            }
        } else {
            consecutive_length = 0;
            start = None;
        }
    }

    None
}
