use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

fn main() -> Result<(), anyhow::Error> {
    let input_path_str = std::env::args().nth(1).expect("input path expected");

    let input_path = PathBuf::from(input_path_str);

    let mut matrix: Vec<Vec<char>> = vec![];
    let mut count: i32 = 0;

    // Get the matrix
    for line in read_lines(input_path)?.map_while(Result::ok) {
        if line.is_empty() {
            break;
        }
        let row = line.chars().collect::<Vec<char>>();
        matrix.push(row);
    }

    for (row_i, row) in matrix.clone().iter().enumerate() {
        if row_i == 0 {
            continue;
        }
        let potential_starts = row
            .iter()
            .enumerate()
            .filter(|(_char_index, char)| **char == 'A');

        // Left to right
        potential_starts.clone().for_each(|(start_index, _char)| {
            if start_index < 1 || start_index >= row.len() - 1 {
                return;
            }
            if row_i == matrix.len() - 1 {
                return;
            }
            // xmas just so they are all different
            let top_left = matrix
                .get(row_i - 1)
                .and_then(|row| row.get(start_index - 1))
                .unwrap_or(&'x');
            let top_right = matrix
                .get(row_i - 1)
                .and_then(|row| row.get(start_index + 1))
                .unwrap_or(&'m');
            let bottom_left = matrix
                .get(row_i + 1)
                .and_then(|row| row.get(start_index - 1))
                .unwrap_or(&'a');
            let bottom_right = matrix
                .get(row_i + 1)
                .and_then(|row| row.get(start_index + 1))
                .unwrap_or(&'s');

            if top_left == &'A'
                || top_right == &'A'
                || bottom_left == &'A'
                || bottom_right == &'A'
                || top_left == &'X'
                || top_right == &'X'
                || bottom_left == &'X'
                || bottom_right == &'X'
            {
                return;
            }

            if top_left == bottom_right || top_right == bottom_left {
                return;
            }
            count += 1;
        });
    }

    println!("{count}");

    Ok(())
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
