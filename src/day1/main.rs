use std::{
    fs::File,
    io::{self, BufRead},
    ops::{Add, Mul},
    path::{Path, PathBuf},
};

fn main() -> Result<(), anyhow::Error> {
    let input_path_str = std::env::args().nth(1).expect("input path expected");

    let input_path = PathBuf::from(input_path_str);

    let add: fn(u32, u32) -> u32 = Add::add;
    let mul: fn(u32, u32) -> u32 = Mul::mul;
    let operators = [add, mul];
    let mut sum: u32 = 0;

    for line in read_lines(input_path)?.map_while(Result::ok) {
        if line.is_empty() {
            break;
        }
        let mut split = line.split(':');
        let result = split.next().unwrap().parse::<u32>().unwrap();

        let numbers = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|str| str.parse::<u32>().unwrap());
        let numbers_len = numbers.count();

        let operator_combinations = operators.iter().cartesian_product();

        let sum: u32 = numbers.reduce(|acc, number| add(acc, number)).unwrap();
    }

    println!("sum: {sum}");

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
