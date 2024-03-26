use std::{process::exit, fs::{OpenOptions, self}, error::Error};

use std::io::Write;

use clap::Parser;
use env_logger;

use log::{LevelFilter, error, info, debug, warn};

use chrono::prelude::*;
use aoc_2023_rust::AdventOfCode;

use aoc_2023_rust::days::day01::*;


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
    #[arg(long, default_value_t = false)]
    sample: bool,

    /// Download input from adventofcode.com
    #[arg(short, long, default_value_t = false)]
    get: bool,

    /// Session token for adventofcode.com, can also be given through AOC_TOKEN env var
    #[arg(short, long)]
    session: Option<String>,

    /// Print Verbose Output
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn fetch_puzzle_input(year: i32, day: usize, session: &str) -> Result<String,  Box<dyn Error>> {

    debug!("Fetching puzzle input from adventofcode.com");
    let http_client = reqwest::blocking::Client::new();
    let res = http_client
        .get(format!("https://adventofcode.com/{}/day/{}/input", year, day+1))
        .header(reqwest::header::COOKIE, format!("session={}", session))
        .send()?;

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
            1 => aoc.days.push(Box::new(Day01::default())),
            _ => { error!("Unknown day!"); exit(-1) }
        }
    } else {
        aoc = AdventOfCode { days: vec![
            Box::new(Day01::default())
        ] };
    }

    info!("Start Solving Challenges");

    for (i, day) in aoc.days.iter_mut().enumerate() {
        println!("Day {:02}", i+1);
        let mut input: String;
        // get puzzle in put if get arg is set
        if args.get {
            debug!("Checking if input dir exists");
            let mut input_file = OpenOptions::new().write(true).truncate(true)
                .create(true).open(format!("input/day{:02}.txt", i + 1)).expect("Failed to open or create file!");

            info!("Downloading puzzle input from adventofcode.com");
            let session = args.session.clone().or_else(|| {std::env::var("AOC_TOKEN").ok() });
            if session.is_some() {
                input = match fetch_puzzle_input(args.year, i, &session.unwrap()) {
                    Ok(s) => s.clone(),
                    Err(err) => { error!("Failed to fetch puzzle input: {err}"); continue;}
                };
                debug!("Writing puzzle input to file");
                input_file.write_all(input.as_bytes()).expect("Failed to write to input file!");
            }
            else {
                warn!("Failed to parse AOC_TOKEN from args or env");
            }
        }

        info!("Fetching input data from local file");
        let input_path : String;
        if args.sample {
            input_path = format!("input/day{:02}_sample.txt", i + 1);
        }
        else {
            input_path = format!("input/day{:02}.txt", i + 1);
        }

        input = match fs::read_to_string(input_path) {
            Ok(s) => s.clone(),
            Err(err) => { error!("Failed to read puzzle input from fs: {err}"); continue;}
        };


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