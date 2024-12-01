use std::{
    borrow::Borrow,
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

fn main() -> Result<(), anyhow::Error> {
    let input_path_str = std::env::args().nth(1).expect("input path expected");

    let input_path = PathBuf::from(input_path_str);

    let mut first_row: Vec<u32> = vec![];
    let mut second_row: HashMap<u32, u32> = HashMap::new();

    for line in read_lines(input_path)?.map_while(Result::ok) {
        if line.is_empty() {
            break;
        }
        let mut split = line.split_whitespace();
        first_row.push(split.next().unwrap().parse::<u32>().unwrap());

        let second_value = split.next().unwrap().parse::<u32>().unwrap();

        let counter = second_row.entry(second_value).or_insert(0);
        *counter += 1;
    }

    let results: u32 = first_row
        .iter()
        .map(|first| {
            let count = second_row.get(first).unwrap_or(0.borrow());
            first * count
        })
        .sum();

    println!("{results}");

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
