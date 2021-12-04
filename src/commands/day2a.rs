use clap::Parser;

use super::{CommandImpl, DynError};


use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};

use strum_macros::EnumString;

#[derive(Parser, Debug)]
pub struct Day2a {
    #[clap(long, short)]
    input: String,
}

#[derive(Debug, PartialEq, EnumString)]
enum Direction {
    #[strum(ascii_case_insensitive)]
    Forward,
    #[strum(ascii_case_insensitive)]
    Up,
    #[strum(ascii_case_insensitive)]
    Down,
}

struct Step {
    direction: Direction,
    length: i32,
}

impl FromStr for Step {
    type Err = String;


    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split(' ').collect();
        let direction: Direction = Direction::from_str(fields[0]).unwrap();
        let length: i32 = fields[1].parse::<i32>().unwrap();
        return Ok(Step { direction, length });
    }
}

fn slurp_steps(filename: impl AsRef<Path>) -> Vec<Step> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| Step::from_str(&l).unwrap())
        .collect()
}

impl CommandImpl for Day2a {
    fn main(&self) -> Result<(), DynError> {
        let steps = slurp_steps(Path::new(&self.input));
        let mut horizontal = 0;
        let mut depth = 0;
        for step in steps {
            match step { 
                Step { direction: Direction::Forward, length } => horizontal += length,
                Step { direction: Direction::Up, length } => depth -= length,
                Step { direction: Direction::Down, length } => depth += length,

            }
        }
        println!("horizontal: {:?} depth: {:?}", horizontal, depth);
        println!("{:?}", horizontal * depth);
        Ok(())
    }
}
