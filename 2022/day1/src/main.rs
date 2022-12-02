use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let mut most_calories = vec![0, 0, 0];
    let mut current_calories = 0;
    for l in lines {
        let l = Line::from_str(&l.unwrap()).unwrap();
        match l {
            Line::Newline => {
                maybe_replace(&mut most_calories, current_calories);
                current_calories = 0;
            }
            Line::Calories(n) => {
                current_calories += n;
            }
        }
    }
    let total: u32 = most_calories.iter().sum();

    println!("{total}");
}

pub enum Line {
    Newline,
    Calories(u32),
}
impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Self::Newline);
        }

        u32::from_str(s).map(|n| Self::Calories(n))
    }
}

fn maybe_replace(top_3: &mut Vec<u32>, n_cal: u32) {
    let Some(idx) = top_3.iter().position(|x| n_cal > *x) else {
        return
    };
    top_3.insert(idx, n_cal);
    top_3.truncate(3);
}
