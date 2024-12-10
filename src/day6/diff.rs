use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

fn main() -> Result<(), anyhow::Error> {
    let input_path_str = std::env::args().nth(1).expect("input path expected");
    let diff_path_str = std::env::args().nth(2).expect("input path expected");

    let input_path = PathBuf::from(input_path_str);
    let diff_path = PathBuf::from(diff_path_str);

    let mut correct: HashSet<(u32, u32)> = HashSet::new();
    let mut incorrect: HashSet<(u32, u32)> = HashSet::new();

    for line in read_lines(input_path)?.map_while(Result::ok) {
        if line.is_empty() {
            break;
        }
        let mut split = line.split(", ");

        let y = split
            .next()
            .unwrap()
            .strip_prefix("(")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let x = split
            .next()
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        correct.insert((y, x));
    }

    for line in read_lines(diff_path)?.map_while(Result::ok) {
        if line.is_empty() {
            break;
        }
        let mut split = line.split(", ");

        let y = split
            .next()
            .unwrap()
            .strip_prefix("(")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let x = split
            .next()
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .parse::<u32>()
            .unwrap();

        incorrect.insert((y, x));
    }

    let diff = &incorrect.difference(&correct);

    diff.clone().for_each(|(y, x)| {
        println!("({y}, {x})");
    });
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
