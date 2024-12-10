use std::{
    fs::File,
    io::{self, BufRead},
    ops::{Add, Mul},
    path::{Path, PathBuf},
};

fn main() -> Result<(), anyhow::Error> {
    let input_path_str = std::env::args().nth(1).expect("input path expected");

    let input_path = PathBuf::from(input_path_str);

    let mut sum: u64 = 0;

    for line in read_lines(input_path)?.map_while(Result::ok) {
        if line.is_empty() {
            break;
        }
        // println!("new line: {line}");

        let mut split = line.split(':');
        let result = split.next().unwrap().parse::<u64>().unwrap();

        let numbers = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|str| str.parse::<u64>().unwrap());

        let numbers_len = numbers.clone().count();
        // println!("{numbers_len}");

        let operator_combinations = generate_combinations(numbers_len - 1);
        let operator_combinations_count = operator_combinations.clone().len();
        // println!("{operator_combinations_count}");

        let add: fn(u64, u64) -> u64 = Add::add;
        let start = [add];
        let has_sum = operator_combinations.iter().clone().find_map(|ops| {
            let operations = start.into_iter().chain(ops.clone());
            let sum: u64 = numbers
                .clone()
                .zip(operations)
                .fold(0, |acc, (number, op)| {
                    // println!("acc: {acc}, op {op:?} {number}");
                    op(acc, number)
                });
            if sum == result {
                return Some(sum);
            }
            None
        });
        // println!("hs {has_sum:?}");

        if let Some(hs) = has_sum {
            sum += hs;
        }
    }

    println!("sum: {sum}");

    Ok(())
}

type OperationCombinations = Vec<Vec<fn(u64, u64) -> u64>>;

fn generate_combinations(length: usize) -> OperationCombinations {
    let add: fn(u64, u64) -> u64 = Add::add;
    let mul: fn(u64, u64) -> u64 = Mul::mul;
    let conc: fn(u64, u64) -> u64 = |a, b| a * 10u64.pow(b.ilog10() + 1) + b;

    let operators = [add, mul, conc];

    if length == 0 {
        return vec![vec![]];
    }

    let mut result = vec![];

    for op in operators {
        for mut combination in generate_combinations(length - 1) {
            combination.insert(0, op);
            result.push(combination);
        }
    }

    result
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
