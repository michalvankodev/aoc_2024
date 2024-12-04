use std::{
    cmp::Ordering,
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

enum RowState {
    Increasing(i32),
    Decreasing(i32),
    Invalid,
    Initial(Option<i32>),
}

fn main() -> Result<(), anyhow::Error> {
    let input_path_str = std::env::args().nth(1).expect("input path expected");

    let input_path = PathBuf::from(input_path_str);
    let mut counter: i32 = 0;

    for line in read_lines(input_path)?.map_while(Result::ok) {
        if line.is_empty() {
            break;
        }

        let split = line
            .split_whitespace()
            .map(|value| str::parse::<i32>(value).unwrap())
            .collect::<Vec<i32>>();

        let possible_combinations = get_possible_combinations(&split);

        for combination in possible_combinations {
            let result = combination
                .into_iter()
                .fold(RowState::Initial(None), validate_row);
            match result {
                RowState::Invalid => {}
                _ => {
                    counter += 1;
                    break;
                }
            }
        }
    }

    println!("{counter}");

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

fn validate_row(state: RowState, value: i32) -> RowState {
    match state {
        RowState::Invalid => RowState::Invalid,
        RowState::Increasing(prev) => {
            if prev >= value || prev + 4 <= value {
                return RowState::Invalid;
            }
            RowState::Increasing(value)
        }
        RowState::Decreasing(prev) => {
            if prev <= value || prev - 4 >= value {
                return RowState::Invalid;
            }
            RowState::Decreasing(value)
        }
        RowState::Initial(Some(prev)) => match value.cmp(&prev) {
            Ordering::Less => {
                if prev - 4 >= value {
                    return RowState::Invalid;
                }
                RowState::Decreasing(value)
            }
            Ordering::Equal => RowState::Invalid,
            Ordering::Greater => {
                if prev + 4 <= value {
                    return RowState::Invalid;
                }
                RowState::Increasing(value)
            }
        },
        RowState::Initial(None) => RowState::Initial(Some(value)),
    }
}

fn get_possible_combinations(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut vectors = vec![nums.to_vec()];
    let orig = nums.to_vec();

    for index in 0..orig.len() {
        let mut copy = orig.clone();
        let _removed = copy.remove(index);
        vectors.push(copy);
    }

    vectors
}
