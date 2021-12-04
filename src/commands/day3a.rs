use clap::Parser;

use super::{CommandImpl, DynError};


use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};


#[derive(Parser, Debug)]
pub struct Day3a {
    #[clap(long, short)]
    input: String,
}


fn most_frequent_bits(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let mut num_lines = 0;
    let mut count_of_ones: Vec<i32> = Vec::new();
    for l in buf.lines() {
        let line = l.expect("Could not parse line");
        let chars: Vec<char> = line.chars().collect();
        if count_of_ones.is_empty() {
            let len = line.len();
            count_of_ones = vec![0; len];
        }
        for (i, c) in chars.iter().enumerate() {
            if *c == '1' {
                count_of_ones[i] += 1
            }
        }
        num_lines += 1;
    }

    for i in 0..count_of_ones.len() {
        let ones = count_of_ones[i];
        let zeros = num_lines - ones;
        //println!("0: {:?} 1: {:?} total: {:?}", zeros, ones, num_lines);
        if ones >= zeros {
            count_of_ones[i] = 1
        } else {
            count_of_ones[i] = 0
        }
    }

    return count_of_ones;
}


fn bit_vec_to_decimal(vec: &Vec<i32>) -> i32 {
    let mut dec = 0;
    let mut factor = 1;
    for i in 0..vec.len() {
        let index = vec.len() - i - 1;
        dec += factor * vec[index];
        factor *= 2;
    }
    return dec
}

impl CommandImpl for Day3a {
    fn main(&self) -> Result<(), DynError> {
        let gamma: Vec<i32> = most_frequent_bits(Path::new(&self.input));
        let epsilon: Vec<i32> = gamma.iter().map(|bit| {
            if *bit == 1 {
                return 0;
            } else {
                return 1;
            }
        }).collect();
        println!("gamma bits: {:?} dec: {:?}", gamma, bit_vec_to_decimal(&gamma));
        println!("epsilon bits: {:?} dec: {:?}", epsilon, bit_vec_to_decimal(&epsilon));
        println!("Answer: {:?}", bit_vec_to_decimal(&gamma) * bit_vec_to_decimal(&epsilon));
        Ok(())
    }
}
