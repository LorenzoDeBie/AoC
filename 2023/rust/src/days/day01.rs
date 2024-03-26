// use clap::error::ContextValue::String;
use log::{debug, info};
use crate::AdventOfCodeDay;
use std::string::String;

#[derive(Default)]
pub struct Day01 {
    // data structs go here
    calibration_doc: String
}

impl AdventOfCodeDay for Day01 {
    // logic goes here
    fn parse_input(&mut self, input: String) {
        println!("{}", input);
        self.calibration_doc = input;

    }

    fn part1(&self) -> String {
        info!("Solving Day1, part 1");
        let lines = self.calibration_doc.lines().collect::<Vec<_>>();
        let mut result = 0;
        for line in lines {
            debug!("Now analyzing line: {}", line);
            // find first digit
            let mut chars = line.chars();
            while let Some(c) = chars.next() {
                if c.is_ascii_digit() {
                    result += c.to_digit(10).unwrap() * 10;
                    debug!("Found First digit: {}", c.to_digit(10).unwrap());
                    break;
                }
            }
            let mut revline = line.chars().rev();
            while let Some(c) = revline.next() {
                if c.is_ascii_digit() {
                    result += c.to_digit(10).unwrap();
                    debug!("Found Last digit: {}", c.to_digit(10).unwrap());
                    break;
                }
            }
        }
        return result.to_string();
    }

    fn part2(&self) -> String {
        let numbers = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        /*let numbers_reverse = numbers.iter().map(
            |number| {
                let new_string = (*number).to_string();
                let number_reverse = new_string.chars().rev().collect::<String>();
                return number_reverse;
            }
        ).collect::<Vec<String>>();*/

        let mut result = 0;
        for line in self.calibration_doc.lines() {
            
            'first: for i in (0..line.len()).rev() {
                let sstring = &line[0..i + 1];
                for (value, number) in numbers.iter().enumerate() {
                    if sstring.starts_with(number) {
                        result += value * 10;
                        break 'first;
                    }
                }
            }

            'last: for i in 0..line.len() {
                let sstring = &line[(line.len() - i - 1)..line.len()];
                for (value, number) in numbers.iter().enumerate() {
                    if sstring.starts_with(number) {
                        result += value;
                        break 'last;
                    }
                }
            }
        }
        return String::from("Day 2, part 2")
    }
}