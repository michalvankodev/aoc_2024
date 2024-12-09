use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Bottom,
    Left,
    Top,
}

#[derive(Clone, Copy)]
enum Obj {
    Obstacle,
    Guard(Direction),
    Empty(bool),
}

fn main() -> Result<(), anyhow::Error> {
    let input_path_str = std::env::args().nth(1).expect("input path expected");

    let input_path = PathBuf::from(input_path_str);

    let mut map: Vec<Vec<Obj>> = vec![];

    for line in read_lines(input_path)?.map_while(Result::ok) {
        if line.is_empty() {
            break;
        }
        let chars = line
            .chars()
            .map(|ch| match ch {
                '#' => Obj::Obstacle,
                '>' => Obj::Guard(Direction::Right),
                'v' => Obj::Guard(Direction::Bottom),
                '<' => Obj::Guard(Direction::Left),
                '^' => Obj::Guard(Direction::Top),
                _ => Obj::Empty(false),
            })
            .collect::<Vec<Obj>>();
        map.push(chars);
    }

    // Find position of guard
    let map_clone = map.clone();
    let mut guard_position = map_clone
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            let guard_in_row = row.iter().enumerate().find_map(|(x, obj)| {
                if let Obj::Guard(direction) = obj {
                    return Some((x, direction));
                }
                None
            });

            if let Some((x, direction)) = guard_in_row {
                return Some(((y, x), *direction));
            }
            None
        })
        .expect("Guard has to be on the map");

    let mut steps: u32 = 0;
    let map_width = map.first().unwrap().len();
    let map_height = map.len();

    loop {
        let ((y, x), direction) = guard_position;

        map[y][x] = Obj::Empty(true);

        let is_out_of_map = get_is_out_of_map((y, x), &direction, map_width, map_height);
        println!("{direction:?} - ({y}, {x})");

        if is_out_of_map {
            break;
        }

        let (mut next_y, mut next_x) = get_next_pos((y, x), &direction);

        let next_obj_on_map = map[next_y][next_x];

        if let Obj::Obstacle = next_obj_on_map {
            let next_direction = get_next_direction(&direction);
            guard_position.1 = next_direction;
            (next_y, next_x) = get_next_pos((y, x), &next_direction);
        }
        guard_position.0 = (next_y, next_x);

        steps += 1;
    }

    let unique: i32 = map
        .iter()
        .flatten()
        .map(|obj| match obj {
            Obj::Empty(true) => 1,
            _ => 0,
        })
        .sum();

    println!("Steps: {steps}, unique paths: {unique}");

    Ok(())
}

fn get_next_pos((y, x): (usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Right => (y, x + 1),
        Direction::Bottom => (y + 1, x),
        Direction::Left => (y, x - 1),
        Direction::Top => (y - 1, x),
    }
}

fn get_is_out_of_map(
    (y, x): (usize, usize),
    direction: &Direction,
    map_width: usize,
    map_height: usize,
) -> bool {
    if y == 0 && matches!(direction, Direction::Top) {
        return true;
    }

    if x == 0 && matches!(direction, Direction::Left) {
        return true;
    }

    if y == map_height - 1 && matches!(direction, Direction::Bottom) {
        return true;
    }

    if x == map_width - 1 && matches!(direction, Direction::Right) {
        return true;
    }

    false
}

fn get_next_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Right => Direction::Bottom,
        Direction::Bottom => Direction::Left,
        Direction::Left => Direction::Top,
        Direction::Top => Direction::Right,
    }
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
