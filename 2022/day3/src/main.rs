#![feature(iter_next_chunk)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, Read};

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let result = compute2(&input);

    println!("{result}");
}

fn compute(input: &str) -> u64 {
    let lines = input.lines();

    let mut sum: u64 = 0;
    for l in lines {
        let l = l.trim();
        let num_item = l.len();
        let (l1, l2) = l.split_at(num_item / 2);

        let mut itemset = HashSet::new();

        for c in l1.chars() {
            itemset.insert(c);
        }

        for c in l2.chars() {
            if itemset.contains(&c) {
                sum += get_priority(c) as u64;
                break;
            }
        }
    }

    sum
}

fn compute2(input: &str) -> u64 {
    let lines = input.lines();

    let mut sum: u64 = 0;
    let mut rest = lines;
    while let Ok(next) = rest.next_chunk::<3>() {
        let common = find_common(next.into_iter());
        sum += get_priority(common) as u64;
    }
    sum
}

fn find_common<'a>(lines: impl Iterator<Item = &'a str>) -> char {
    let mut counter = HashMap::new();
    for l in lines {
        println!("{l}");
        let l = l.trim();
        let encountered: HashSet<char> = l.chars().collect();
        for c in encountered {
            let n = counter.get(&c).copied().unwrap_or(0 as u32);
            counter.insert(c, n + 1 as u32);
        }
    }

    println!("{counter:?}");
    let (c, _n) = counter
        .iter()
        .find(|(k, v)| **v >= 3)
        .expect("could not find shared item");
    *c
}

fn get_priority(c: char) -> u8 {
    let result = if c.is_ascii_uppercase() {
        (c as u8 - 'A' as u8) + 27
    } else {
        assert!(c.is_ascii_lowercase());
        (c as u8 - 'a' as u8) + 1
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
        "#;

    #[test]
    fn test_prio() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }

    #[test]
    fn test() {
        let result = compute(INPUT.trim());

        assert_eq!(result, 157);
    }

    #[test]
    fn test_common() {
        let mut input = INPUT.trim().lines();
        assert_eq!(
            find_common(input.next_chunk::<3>().unwrap().into_iter()),
            'r'
        );
        println!("here");
        assert_eq!(
            find_common(input.next_chunk::<3>().unwrap().into_iter()),
            'Z'
        );
    }
}
