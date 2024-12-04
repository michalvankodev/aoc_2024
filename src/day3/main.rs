use std::{
    fs::{self},
    path::PathBuf,
};

fn main() -> Result<(), anyhow::Error> {
    let input_path_str = std::env::args().nth(1).expect("input path expected");

    let input_path = PathBuf::from(input_path_str);

    let mut total: i32 = 0;
    let mut activation_state = true;

    let contents = fs::read_to_string(input_path).expect("should be able to read file");
    let mut contents_iter = contents.chars();

    while let Some(ch) = contents_iter.next() {
        match ch {
            'd' => {
                let dont_advance = contents_iter.clone().take(6).collect::<String>();

                if dont_advance.eq("on't()") {
                    activation_state = false;
                    advance_by(&mut contents_iter, 6);
                    continue;
                }
                if dont_advance.starts_with("o()") {
                    activation_state = true;
                    advance_by(&mut contents_iter, 3);
                    continue;
                }
            }
            'm' => {
                let mul_advance = contents_iter.clone().take(3).collect::<String>();
                if !mul_advance.eq("ul(") {
                    continue;
                }
                // advance iterator by 3
                advance_by(&mut contents_iter, 3);

                let first_n = contents_iter
                    .clone()
                    .take_while(|char| char.is_ascii_digit())
                    .collect::<String>();
                if first_n.is_empty() || first_n.len() > 3 {
                    continue;
                }
                advance_by(&mut contents_iter, first_n.len());
                let separator = contents_iter.clone().take(1).collect::<String>();
                if separator != "," {
                    continue;
                }
                advance_by(&mut contents_iter, 1);
                let second_n = contents_iter
                    .clone()
                    .take_while(|char| char.is_ascii_digit())
                    .collect::<String>();
                if second_n.is_empty() || second_n.len() > 3 {
                    continue;
                }
                advance_by(&mut contents_iter, second_n.len());

                let ending = contents_iter.clone().take(1).collect::<String>();
                if ending != ")" {
                    continue;
                }
                advance_by(&mut contents_iter, 1);
                let first = first_n.parse::<i32>().unwrap();
                let second = second_n.parse::<i32>().unwrap();

                if activation_state {
                    total += first * second;
                }
            }
            _ => {
                continue;
            }
        }
    }

    println!("{total}");

    Ok(())
}

fn advance_by<T: Iterator>(iter: &mut T, count: usize) {
    for _ in 0..count {
        iter.next();
    }
}
