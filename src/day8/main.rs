use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point(usize, usize);
impl Point {
    fn get_antinodes(&self, second: &Point, map_width: usize, map_height: usize) -> Vec<Point> {
        let mut antinodes = vec![];

        let distance_y = self.0.abs_diff(second.0) as i32;
        let distance_x = self.1.abs_diff(second.1) as i32;

        let first_y = match self.0.cmp(&second.0) {
            std::cmp::Ordering::Less => self.0 as i32 - distance_y,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => self.0 as i32 + distance_y,
        };

        let first_x = match self.1.cmp(&second.1) {
            std::cmp::Ordering::Less => self.1 as i32 - distance_x,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => self.1 as i32 + distance_x,
        };

        if in_bounds(first_y, map_height) && in_bounds(first_x, map_width) {
            antinodes.push(Self(first_y as usize, first_x as usize));
        }

        let second_y = match self.0.cmp(&second.0) {
            std::cmp::Ordering::Less => second.0 as i32 + distance_y,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => second.0 as i32 - distance_y,
        };

        let second_x = match self.1.cmp(&second.1) {
            std::cmp::Ordering::Less => second.1 as i32 + distance_x,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => second.1 as i32 - distance_x,
        };

        if in_bounds(second_y, map_height) && in_bounds(second_x, map_width) {
            antinodes.push(Self(second_y as usize, second_x as usize));
        }
        antinodes
    }
}

fn in_bounds(n: i32, bound: usize) -> bool {
    n > -1 && n < bound as i32
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Antenna(char, Point);

fn main() -> Result<(), anyhow::Error> {
    let input_path_str = std::env::args().nth(1).expect("input path expected");
    let input_path = PathBuf::from(input_path_str);

    let (map, width, height) = parse_map(input_path)?;

    let antennas: HashSet<Antenna> = map
        .iter()
        .filter(|(_point, char)| **char != '.')
        .map(|(point, char)| Antenna(*char, *point))
        .collect();

    let mut antenna_map: HashMap<char, Vec<Point>> = HashMap::new();
    antennas.iter().for_each(|antenna| {
        let entry = antenna_map.entry(antenna.0).or_default();
        entry.push(antenna.1);
    });

    let antinodes: HashSet<Point> = antenna_map
        .iter()
        .filter(|(_char, points)| points.len() > 1)
        .flat_map(|(_char, points)| {
            points
                .iter()
                .combinations(2)
                .flat_map(|couple| couple[0].get_antinodes(couple[1], width, height))
        })
        .collect();

    let antinode_count = antinodes.len();
    println!("count {antinode_count}");
    // antinodes.iter().for_each(|node| println!("{node:?}"));

    Ok(())
}

fn parse_map(input_path: PathBuf) -> Result<(HashMap<Point, char>, usize, usize), anyhow::Error> {
    let mut map: HashMap<Point, char> = HashMap::new();
    let mut height = 0;
    let mut width = 0;

    for (y, line) in read_lines(input_path)?.map_while(Result::ok).enumerate() {
        if line.is_empty() {
            break;
        }

        let split = line.chars();
        split.clone().enumerate().for_each(|(x, ch)| {
            map.insert(Point(y, x), ch);
        });
        width = split.count();
        height = y + 1;
    }
    Ok((map, width, height))
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
