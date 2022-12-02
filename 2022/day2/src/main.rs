use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let sum: u64 = lines
        .map(|l| {
            let l = l.unwrap();
            let two: Vec<_> = l.split(" ").collect();
            if two.len() != 2 {
                println!("{two:?}");
                return 0;
            }
            // let &[theirs, mine] = &two[..2] else {
            let &[theirs, victor] = &two[..2] else {
                panic!()
            };
            let theirs = Strategy::from_theirs(theirs);
            // let mine = Strategy::from_mine(mine);
            let victor = Victor::from_str(victor);
            let mine = Strategy::from_victor(theirs, victor);

            let mut value = mine.value();

            let result = fight(theirs, mine);
            assert!(result == victor);
            value += result.value();

            value as u64
        })
        .sum();

    println!("{sum}")
}

#[repr(u8)]
#[derive(Copy, Clone)]
enum Strategy {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
impl Strategy {
    fn from_theirs(c: &str) -> Self {
        match c {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!(),
        }
    }

    fn from_mine(c: &str) -> Self {
        match c {
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => panic!(),
        }
    }

    fn value(&self) -> u8 {
        *self as u8
    }

    fn from_victor(theirs: Strategy, victor: Victor) -> Self {
        use Strategy::*;
        use Victor::*;
        if matches!(victor, Tie) {
            return theirs;
        }
        match (theirs, victor) {
            (Rock, Me) => Paper,
            (Rock, Them) => Scissors,
            (Paper, Me) => Scissors,
            (Paper, Them) => Rock,
            (Scissors, Me) => Rock,
            (Scissors, Them) => Paper,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Victor {
    Me,
    Them,
    Tie,
}
impl Victor {
    fn value(&self) -> u8 {
        match self {
            Self::Them => 0,
            Self::Tie => 3,
            Self::Me => 6,
        }
    }

    fn from_str(s: &str) -> Self {
        use Victor::*;
        match s {
            "X" => Them,
            "Y" => Tie,
            "Z" => Me,
            _ => panic!(),
        }
    }
}

fn fight(theirs: Strategy, mine: Strategy) -> Victor {
    use Strategy::*;
    use Victor::*;
    match (theirs, mine) {
        (Rock, Rock) => Tie,
        (Rock, Paper) => Me,
        (Rock, Scissors) => Them,
        (Paper, Rock) => Them,
        (Paper, Paper) => Tie,
        (Paper, Scissors) => Me,
        (Scissors, Rock) => Me,
        (Scissors, Paper) => Them,
        (Scissors, Scissors) => Tie,
    }
}
