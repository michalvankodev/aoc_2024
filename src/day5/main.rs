use std::{
    borrow::BorrowMut,
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

fn main() -> Result<(), anyhow::Error> {
    let input_path_str = std::env::args().nth(1).expect("input path expected");

    let input_path = PathBuf::from(input_path_str);

    let mut result: i32 = 0;
    let mut incorrect_result: i32 = 0;
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    // let mut updates: Vec<Vec<i32>> = vec![];

    let mut reader = read_lines(input_path)?.map_while(Result::ok);

    // parse rules
    for line in reader.borrow_mut() {
        if line.is_empty() {
            break;
        }
        let mut split = line.split('|');
        let first_page = split.next().unwrap().parse::<i32>()?;
        let second_page = split.next().unwrap().parse::<i32>()?;
        let map_entry = rules.entry(first_page).or_default();
        map_entry.push(second_page);
    }

    // parse updates
    for line in reader {
        if line.is_empty() {
            break;
        }
        let split = line.split(',');
        let pages = split
            .map(|page| page.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let is_valid = validate_order(&rules, &pages);
        if is_valid {
            result += get_middle(&pages);
        } else {
            let correct_order = get_correct_order(&rules, &pages);
            incorrect_result += get_middle(&correct_order);
        }
    }

    println!("Valid: {result}, Invalid corrected: {incorrect_result}");

    Ok(())
}

fn get_correct_order(rules: &HashMap<i32, Vec<i32>>, pages: &Vec<i32>) -> Vec<i32> {
    let mut result = pages.clone();
    let empty = vec![];

    result.sort_by(|a, b| {
        let b_rules = rules.get(b).unwrap_or(&empty);

        if b_rules.contains(a) {
            return Ordering::Greater;
        }
        Ordering::Less
    });

    result
}

#[test]
fn test_get_correct_order() {
    let rules = HashMap::from([
        (75, vec![47, 61, 53, 29]),
        (47, vec![61, 53, 29]),
        (61, vec![53, 29]),
        (53, vec![29]),
    ]);
    let invalid_pages = vec![47, 75, 61, 53, 29];
    assert!(!validate_order(&rules, &invalid_pages), "pages are invalid");

    let corrected_order = get_correct_order(&rules, &invalid_pages);
    assert!(
        validate_order(&rules, &corrected_order),
        "corrected_order is valid"
    );
}

fn validate_order(rules: &HashMap<i32, Vec<i32>>, pages: &[i32]) -> bool {
    let empty = vec![];
    let mut page_iter = pages.iter().rev();

    while let Some(page) = page_iter.next() {
        let page_rules = rules.get(page).unwrap_or(&empty);
        let mut iter_clone = page_iter.clone();
        let rest_is_valid = !iter_clone.any(|other_page| page_rules.contains(other_page));
        if !rest_is_valid {
            return false;
        }
    }

    true
}

#[test]
fn test_validate_order() {
    let rules = HashMap::from([
        (75, vec![47, 61, 53, 29]),
        (47, vec![61, 53, 29]),
        (61, vec![53, 29]),
        (53, vec![29]),
    ]);
    let valid_pages = vec![75, 47, 61, 53, 29];
    let invalid_pages = vec![47, 75, 61, 53, 29];
    assert!(validate_order(&rules, &valid_pages), "Valid pages");
    assert!(!validate_order(&rules, &invalid_pages), "invalid pages")
}

fn get_middle(pages: &[i32]) -> i32 {
    let middle_index = pages.len() / 2;
    pages[middle_index]
}

#[test]
fn test_get_middle() {
    let pages = vec![1, 2, 3, 4, 5];
    assert_eq!(get_middle(&pages), 3);
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
