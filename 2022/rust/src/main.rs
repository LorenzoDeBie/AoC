mod advent_of_code;
mod days;

use std::{process::exit, fs::{OpenOptions, self}, error::Error};

use clap::Parser;
use env_logger;

use log::{LevelFilter, error, info, debug, warn};

use crate::{advent_of_code::AdventOfCode, days::day01::Day01};

use chrono::prelude::*;

#[derive(Parser)]
struct Args {
    /// Day to solve, 0 == all days
    #[arg(short, long, default_value_t = 0)]
    day: u8,


    /// Year of AOC to solve
    #[arg(short, long, default_value_t = chrono::Utc::now().year())]
    year: i32,

    /// Part to solve, 0 == both parts
    #[arg(short, long, default_value_t = 0)]
    part: u8,

    /// Use sample input
    #[arg(short, long, default_value_t = false)]
    test: bool,

    /// Download input from adventofcode.com
    #[arg(short, long, default_value_t = false)]
    get: bool,

    /// Session token for adventofcode.com, can also be given through AOC_SESSION env var
    #[arg(short, long)]
    session: Option<String>,

    /// Print Verbose Output
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,    
}

fn fetch_puzzle_input(day: usize, session: &str) -> Result<String,  Box<dyn Error>> {
    info!("Getting puzzle input");
    debug!("Checking if input dir exists");
    let mut input_file = OpenOptions::new().write(true).create_new(true).open(format!("input/day{:02}.txt", day+1))?;

    debug!("Fetching puzzle input from adventofcode.com");
    let http_client = reqwest::blocking::Client::new();
    let mut res = http_client
    .get(format!("https://adventofcode.com/2022/day/{}/input", day+1))
    .header(reqwest::header::COOKIE, format!("session={}", session))
    .send()?;

    debug!("Writing puzzle input to file");
    res.copy_to(&mut input_file)?;

    return Ok(res.text()?);
}

fn main() {
    //parse cli args
    let args = Args::parse();
    
    //setup logging config
    match args.verbose {
        0 => env_logger::Builder::new().filter_level(LevelFilter::Error).init(),
        1 => env_logger::Builder::new().filter_level(LevelFilter::Info).init(),
        2 => env_logger::Builder::new().filter_level(LevelFilter::Debug).init(),
        _ => env_logger::Builder::new().filter_level(LevelFilter::Trace).init(),
    }

    let mut aoc ;

    if args.day != 0 {
        aoc = AdventOfCode { days: vec![] };
        match args.day {
            1 => aoc.days.push(Box::new(Day01{})),
            _ => { error!("Unknown day!"); exit(-1) }
        }
    } else {
        aoc = AdventOfCode { days: vec![
            Box::new(Day01{})
        ] };
    }

    info!("Start Solving Challenges");

    for (i, day) in aoc.days.iter().enumerate() {
        println!("Day {:02}", i+1);
        let input: String;
        if args.get {
            let session = args.session.clone().or_else(|| {std::env::var("AOC_TOKEN").ok() });
            if session.is_some() {
                input = match fetch_puzzle_input(i, &session.unwrap()) {
                    Ok(s) => s.clone(),
                    Err(err) => { error!("Failed to fetch puzzle input: {err}"); continue;}
                }
            }
            else {
                warn!("Failed to parse AOC_TOKEN from args or env");
                info!("Fetching input data from local file");
                input = match fs::read_to_string(format!("input/day{:02}.txt", i + 1)) {
                    Ok(s) => s.clone(),
                    Err(err) => { error!("Failed to read puzzle input from fs: {err}"); continue;}
                };
            }
        } else {
            info!("Fetching input data from local file");
            input = match fs::read_to_string(format!("input/day{:02}.txt", i + 1)) {
                Ok(s) => s.clone(),
                Err(err) => { error!("Failed to read puzzle input from fs: {err}"); continue;}
            };
        }

        info!("Parsing input");
        day.parse_input(input);

        // solve challenge
        info!("Solving challenge");
        match args.part {
            1 => println!("Part 1: {}", day.part1()),
            2 => println!("Part 2: {}", day.part2()),
            0 => { println!("Part 1: {}", day.part1()); println!("Part 2: {}", day.part2()) }
            _ => { error!("Unknown part!"); exit(-1) }
        };
    }

}
