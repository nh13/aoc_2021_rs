use clap::Parser;

use super::{CommandImpl, DynError};

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[derive(Parser, Debug)]
pub struct Day1b {
    #[clap(long, short)]
    input: String,
}

fn slurp_depths(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| l.parse::<i32>().unwrap())
        .collect()
}

impl CommandImpl for Day1b {
    fn main(&self) -> Result<(), DynError> {
        let depths = slurp_depths(Path::new(&self.input));
        let mut prev = None;
        let mut increased = 0;
        for window in depths.windows(3) {
            let sum: i32 = window.iter().sum();
            if let Some(p) = prev {
                if p < sum{
                    increased += 1;
                    //println!("{:?} < {:?}", p, sum);
                }
            }
            prev = Some(sum);
        }
        println!("{:?}", increased);
        Ok(())
    }
}
