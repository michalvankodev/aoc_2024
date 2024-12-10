use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Bottom,
    Left,
    Top,
}

impl Direction {
    fn get_next(&self) -> Self {
        match self {
            Direction::Right => Direction::Bottom,
            Direction::Bottom => Direction::Left,
            Direction::Left => Direction::Top,
            Direction::Top => Direction::Right,
        }
    }

    fn get_next_pos(&self, (y, x): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Right => (y, x + 1),
            Direction::Bottom => (y + 1, x),
            Direction::Left => (y, x - 1),
            Direction::Top => (y - 1, x),
        }
    }
}

#[derive(Clone, Copy)]
enum Obj {
    Obstacle,
    Guard(Direction),
    Empty,
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
                _ => Obj::Empty,
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

    let starting_guard_position = guard_position;

    let mut visited_positions: HashSet<((usize, usize), Direction)> = HashSet::new();
    let map_width = map.first().unwrap().len();
    let map_height = map.len();

    // Main game loop
    loop {
        let ((y, x), direction) = guard_position;

        visited_positions.insert(((y, x), direction));
        let is_out_of_map = get_is_going_out_of_map((y, x), &direction, map_width, map_height);

        if is_out_of_map {
            break;
        }

        let (next_y, next_x) = direction.get_next_pos((y, x));

        let next_obj_on_map = map[next_y][next_x];

        if let Obj::Obstacle = next_obj_on_map {
            let next_direction = direction.get_next();
            guard_position.1 = next_direction;
            continue;
        }

        guard_position.0 = (next_y, next_x);
    }

    let unique = visited_positions
        .iter()
        .map(|(pos, _direction)| pos)
        .collect::<HashSet<_>>();

    let unique_len = unique.len();

    println!("unique paths: {unique_len}");

    let infinite_loops = unique
        .iter()
        .filter(|pos| ***pos != starting_guard_position.0)
        .filter(|pos| {
            let mut guard_position = starting_guard_position;
            let mut map = map.clone();
            let mut visited_positions: HashSet<((usize, usize), Direction)> = HashSet::new();
            map[pos.0][pos.1] = Obj::Obstacle;

            loop {
                let ((y, x), direction) = guard_position;

                if visited_positions.contains(&guard_position) {
                    println!("{guard_position:?}");
                    return true;
                }

                visited_positions.insert(((y, x), direction));
                let is_out_of_map =
                    get_is_going_out_of_map((y, x), &direction, map_width, map_height);

                if is_out_of_map {
                    break;
                }

                let (next_y, next_x) = direction.get_next_pos((y, x));

                let next_obj_on_map = map[next_y][next_x];

                if let Obj::Obstacle = next_obj_on_map {
                    let next_direction = direction.get_next();
                    guard_position.1 = next_direction;
                    continue;
                }

                guard_position.0 = (next_y, next_x);
            }

            false
        })
        .count();
    println!("infinite_loops_count: {infinite_loops}");

    Ok(())
}

fn get_is_going_out_of_map(
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

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
