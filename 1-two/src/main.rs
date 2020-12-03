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
use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Error};
#[cfg(test)]
use demonstrate::demonstrate;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Password Checker", about = "Checks passwords for a shopkeeper")]
struct Args {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug, PartialEq)]
pub struct PasswordConstraint {
    pub first: usize,
    pub second: usize,
}

impl<'a> TryFrom<&'a str> for PasswordConstraint {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (first, second) = value
            .split("-")
            .map(|val| val.parse::<usize>().context("Could not parse value"))
            .collect::<Result<Vec<_>, Self::Error>>()
            .map(|parsed| (parsed[0], parsed[1]))?;

        Ok(Self { first, second })
    }
}

#[derive(Debug, PartialEq)]
pub struct Line<'a> {
    pub constraints: PasswordConstraint,
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

            Ok(Self {
                constraints: PasswordConstraint::try_from(*positions)?,
                letter,
                password: password,
            })
        } else {
            Err(anyhow!("Could not parse line, too many items"))
        }
    }
}

pub fn read_file(fname: PathBuf) -> std::io::Result<String> {
    let mut file = File::open(fname)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn index_str(string: &str, index: usize) -> String {
    string
        .chars()
        .nth(index)
        .expect("String could not be indexed")
        .to_string()
}

pub fn solution_one(res: &str) -> anyhow::Result<i32> {
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

        if line.constraints.first <= count && count <= line.constraints.second {
            tot += 1;
        }
    }

    Ok(tot)
}

pub fn solution_two(res: &str) -> anyhow::Result<i32> {
    let mut tot = 0;
    for line in res.lines() {
        let line = Line::try_from(line)?;
        let first_positional_character = index_str(line.password, line.constraints.first - 1);
        let second_positional_character = index_str(line.password, line.constraints.second - 1);

        let first_only_valid =
            first_positional_character == line.letter && second_positional_character != line.letter;
        let second_only_valid =
            first_positional_character != line.letter && second_positional_character == line.letter;

        if first_only_valid ^ second_only_valid {
            tot += 1;
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

#[cfg(test)]
demonstrate! {
    describe "PasswordConstraint" {
        use super::*;
        context "with good input" {

            before {
                let input = "1-3";
            }

            it "parses the constraint correctly" {
                let res = PasswordConstraint::try_from(input);
                assert_eq!(res.is_ok(), true);
                assert_eq!(res.unwrap(), PasswordConstraint { first: 1, second: 3 });
            }
        }

        context "with bad input" {
            before {
                let input = "1*3";
            }

            it "returns an error" {
                let res = PasswordConstraint::try_from(input);
                assert_eq!(res.is_err(), true);
            }
        }
    }

    describe "Line" {
        use super::*;
        context "with good input" {

            before {
                let input = "1-3 a: abcdef";
            }

            it "parses the line correctly" {
                let res = Line::try_from(input);
                assert_eq!(res.is_ok(), true);
                assert_eq!(res.unwrap(), Line { constraints: PasswordConstraint { first: 1, second: 3 }, letter: "a", password: "abcdef" });
            }
        }

        context "with bad input for a line constraint" {
            before {
                let input = "1*3 a: abcdef";
            }

            it "returns an error" {
                let res = Line::try_from(input);
                assert_eq!(res.is_err(), true);
            }
        }

        context "with bad input for a letter/password" {
            before {
                let input = "1-3 a abcdef";
            }

            it "returns an error" {
                let res = Line::try_from(input);
                assert_eq!(res.is_err(), true);
            }
        }
    }

    describe "read_file" {
        use super::*;
        use std::io::{Write, Seek, SeekFrom};
        use tempfile::NamedTempFile;

        context "when the file exists" {
            before {
                let expected = "Test 123";
                let mut tmp: NamedTempFile = NamedTempFile::new().unwrap();
                write!(tmp, "Test 123").unwrap();
                tmp.seek(SeekFrom::Start(0)).unwrap();
            }

            it "can read the file" {
                let res = read_file(tmp.path().into());
                assert_eq!(res.is_ok(), true);
                assert_eq!(res.unwrap(), expected);
            }
        }
    }

    describe "index_str" {
        use super::*;

        context "with a good index" {
            it "returns the string containing the character at that index" {
                assert_eq!(index_str("123", 1), "2");
            }
        }

         context "with a out of bounds index" {
            #[should_panic]
            it "panics" {
                index_str("123", 4);
            }
         }
    }
}
