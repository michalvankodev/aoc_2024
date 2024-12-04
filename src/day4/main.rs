use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

enum Direction {
    LeftToRight,
}

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
        let potential_starts = row
            .iter()
            .enumerate()
            .filter(|(_char_index, char)| **char == 'X');

        // Left to right
        potential_starts.clone().for_each(|(start_index, _char)| {
            let next_char = matrix.get(row_i).and_then(|row| row.get(start_index + 1));
            if let Some('M') = next_char {
                let next_char = matrix.get(row_i).and_then(|row| row.get(start_index + 2));
                if let Some('A') = next_char {
                    let next_char = matrix.get(row_i).and_then(|row| row.get(start_index + 3));
                    if let Some('S') = next_char {
                        count += 1;
                    }
                }
            }
        });
        // Right to left
        potential_starts.clone().for_each(|(start_index, _char)| {
            if start_index < 3 {
                return;
            }
            let next_char = matrix.get(row_i).and_then(|row| row.get(start_index - 1));
            if let Some('M') = next_char {
                let next_char = matrix.get(row_i).and_then(|row| row.get(start_index - 2));
                if let Some('A') = next_char {
                    let next_char = matrix.get(row_i).and_then(|row| row.get(start_index - 3));
                    if let Some('S') = next_char {
                        count += 1;
                    }
                }
            }
        });

        // Diagonal top to bottom left to right
        potential_starts.clone().for_each(|(start_index, _char)| {
            let next_char = matrix
                .get(row_i + 1)
                .and_then(|row| row.get(start_index + 1));
            if let Some('M') = next_char {
                let next_char = matrix
                    .get(row_i + 2)
                    .and_then(|row| row.get(start_index + 2));
                if let Some('A') = next_char {
                    let next_char = matrix
                        .get(row_i + 3)
                        .and_then(|row| row.get(start_index + 3));
                    if let Some('S') = next_char {
                        count += 1;
                    }
                }
            }
        });

        // Diagonal top to bottom right to left
        potential_starts.clone().for_each(|(start_index, _char)| {
            if start_index < 3 {
                return;
            }
            let next_char = matrix
                .get(row_i + 1)
                .and_then(|row| row.get(start_index - 1));
            if let Some('M') = next_char {
                let next_char = matrix
                    .get(row_i + 2)
                    .and_then(|row| row.get(start_index - 2));
                if let Some('A') = next_char {
                    let next_char = matrix
                        .get(row_i + 3)
                        .and_then(|row| row.get(start_index - 3));
                    if let Some('S') = next_char {
                        count += 1;
                    }
                }
            }
        });

        // Diagonal bottom to top right to left
        potential_starts.clone().for_each(|(start_index, _char)| {
            if start_index < 3 {
                return;
            }
            if row_i < 3 {
                return;
            }
            let next_char = matrix
                .get(row_i - 1)
                .and_then(|row| row.get(start_index - 1));
            if let Some('M') = next_char {
                let next_char = matrix
                    .get(row_i - 2)
                    .and_then(|row| row.get(start_index - 2));
                if let Some('A') = next_char {
                    let next_char = matrix
                        .get(row_i - 3)
                        .and_then(|row| row.get(start_index - 3));
                    if let Some('S') = next_char {
                        count += 1;
                    }
                }
            }
        });
        // Diagonal bottom to top left to right
        potential_starts.clone().for_each(|(start_index, _char)| {
            if row_i < 3 {
                return;
            }
            let next_char = matrix
                .get(row_i - 1)
                .and_then(|row| row.get(start_index + 1));
            if let Some('M') = next_char {
                let next_char = matrix
                    .get(row_i - 2)
                    .and_then(|row| row.get(start_index + 2));
                if let Some('A') = next_char {
                    let next_char = matrix
                        .get(row_i - 3)
                        .and_then(|row| row.get(start_index + 3));
                    if let Some('S') = next_char {
                        count += 1;
                    }
                }
            }
        });

        // bottom to top
        potential_starts.clone().for_each(|(start_index, _char)| {
            if row_i < 3 {
                return;
            }
            let next_char = matrix.get(row_i - 1).and_then(|row| row.get(start_index));
            if let Some('M') = next_char {
                let next_char = matrix.get(row_i - 2).and_then(|row| row.get(start_index));
                if let Some('A') = next_char {
                    let next_char = matrix.get(row_i - 3).and_then(|row| row.get(start_index));
                    if let Some('S') = next_char {
                        count += 1;
                    }
                }
            }
        });

        // top to bottom
        potential_starts.clone().for_each(|(start_index, _char)| {
            let next_char = matrix.get(row_i + 1).and_then(|row| row.get(start_index));
            if let Some('M') = next_char {
                let next_char = matrix.get(row_i + 2).and_then(|row| row.get(start_index));
                if let Some('A') = next_char {
                    let next_char = matrix.get(row_i + 3).and_then(|row| row.get(start_index));
                    if let Some('S') = next_char {
                        count += 1;
                    }
                }
            }
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
