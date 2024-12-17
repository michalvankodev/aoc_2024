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
    loop {
        if drive.last().is_none() {
            drive.pop();
            continue;
        }

        let Some(first_empty) = drive.iter().enumerate().find(|(_, block)| block.is_none()) else {
            break;
        };
        // Rust gold
        drive.swap_remove(first_empty.0);
    }

    // let debug_drive_after_defragmentation = drive.iter().collect::<String>();
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
