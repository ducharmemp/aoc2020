#![warn(
    clippy::all,
    clippy::perf,
    clippy::correctness,
    clippy::nursery,
    clippy::cargo,
    clippy::complexity,
    missing_debug_implementations,
    rust_2018_idioms
)]
use anyhow::{anyhow, Context, Error};
use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Password Checker", about = "Checks passwords for a shopkeeper")]
struct Args {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug)]
struct Line<'a> {
    pub constraints: [usize; 2],
    pub letter: &'a str,
    pub password: &'a str,
}

impl<'a> TryFrom<&'a str> for Line<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let parts = value.split_whitespace().collect::<Vec<_>>();
        if let [positions, letter, password] = &parts[0..3] {
            let letter = letter
                .strip_suffix(":")
                .context("Malformed letter in line")?;
            let parsed_positions = positions
                .split("-")
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            if let [first, second] = &parsed_positions[..] {
                Ok(Self {
                    constraints: [*first, *second],
                    letter: letter.clone(),
                    password: password.clone(),
                })
            } else {
                Err(anyhow!("Could not properly parse the constraints"))
            }
        } else {
            Err(anyhow!("Could not parse line, too many items"))
        }
    }
}

fn read_file(fname: PathBuf) -> std::io::Result<String> {
    let mut file = File::open(fname)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn index_str(string: &str, index: usize) -> String {
    string.chars().nth(index).unwrap().to_string()
}

fn solution_one(res: &str) -> anyhow::Result<i32> {
    let mut tot = 0;
    for line in res.lines() {
        let line = Line::try_from(line)?;

        let count = line.password.chars().fold(0, |count, character| {
            let character = character.to_string();
            if character.to_string() == line.letter {
                count + 1
            } else {
                count
            }
        });

        if line.constraints[0] <= count && count <= line.constraints[1] {
            tot += 1;
        }
    }

    Ok(tot)
}

fn solution_two(res: &str) -> anyhow::Result<i32> {
    let mut tot = 0;
    for line in res.lines() {
        let line = Line::try_from(line)?;
        let first_positional_character = index_str(line.password, line.constraints[0] - 1);
        let second_positional_character = index_str(line.password, line.constraints[1] - 1);

        let first_only_valid =
            first_positional_character == line.letter && second_positional_character != line.letter;
        let second_only_valid =
            first_positional_character != line.letter && second_positional_character == line.letter;

        match [first_only_valid, second_only_valid] {
            [true, false] | [false, true] => tot += 1,
            _ => (),
        }
    }

    Ok(tot)
}

fn main() {
    let opts = Args::from_args();
    let res = read_file(opts.input).expect("Could not read input file");

    println!(
        "Good passwords: {:?}",
        solution_one(&res).expect("Could not run the first solution")
    );
    println!(
        "Good passwords: {:?}",
        solution_two(&res).expect("Could not run the second solution")
    );
}
