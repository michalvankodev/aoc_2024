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
        let mut antinodes: Vec<Point> = Vec::new();
        antinodes.push(*self);
        antinodes.push(*second);

        let distance_y = self.0.abs_diff(second.0) as i32;
        let distance_x = self.1.abs_diff(second.1) as i32;

        let vec_y = match self.0.cmp(&second.0) {
            std::cmp::Ordering::Less => -distance_y,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => distance_y,
        };

        let vec_x = match self.1.cmp(&second.1) {
            std::cmp::Ordering::Less => -distance_x,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => distance_x,
        };

        let vec = (vec_y, vec_x);

        let mut reverse_y = self.0 as i32 - vec.0;
        let mut reverse_x = self.1 as i32 - vec.1;
        while in_bounds(reverse_y, map_height) && in_bounds(reverse_x, map_width) {
            let antinode = Self(reverse_y as usize, reverse_x as usize);
            antinodes.push(antinode);
            reverse_y -= vec.0;
            reverse_x -= vec.1;
        }

        let mut drive_y = self.0 as i32 + vec.0;
        let mut drive_x = self.1 as i32 + vec.1;
        while in_bounds(drive_y, map_height) && in_bounds(drive_x, map_width) {
            let antinode = Self(drive_y as usize, drive_x as usize);
            antinodes.push(antinode);
            drive_y += vec.0;
            drive_x += vec.1;
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
