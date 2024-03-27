use std::{error::Error, fs::OpenOptions, process::exit};

use std::io::Write;

use clap::Parser;

use log::{LevelFilter, error, info, debug};

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

fn fetch_puzzle_input(year: i32, day: u8, session: &str) -> Result<String,  Box<dyn Error>> {

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

    info!("Start Solving Challenges");
    for i in 1..aoc_2023_rust_lib::NUM_DAYS+1 {
        println!("Day {:02}", i);
        let input: String;
        // get puzzle in put if get arg is set
        if args.get {
            debug!("Checking if input dir exists");
            let mut input_file = OpenOptions::new().write(true).truncate(true)
                .create(true).open(format!("input/day{:02}.txt", i + 1)).expect("Failed to open or create file!");

            info!("Downloading puzzle input from adventofcode.com");
            let session = match args.session {
                Some(ref s) => s.clone(),
                None => std::env::var("AOC_TOKEN").expect("Session key was not found in program args or env")
            };
            
            input = match fetch_puzzle_input(args.year, i, &session) {
                Ok(s) => s.clone(),
                Err(err) => { error!("Failed to fetch puzzle input: {err}"); continue;}
            };
            debug!("Writing puzzle input to file");
            input_file.write_all(input.as_bytes()).expect("Failed to write to input file!");
        }

        // solve challenge
        info!("Solving challenge");
        match args.part {
            1 => println!("Part 1: {:?}", aoc_2023_rust_lib::solve_part(i, 1)),
            2 => println!("Part 2: {:?}", aoc_2023_rust_lib::solve_part(i, 2)),
            0 => { println!("Part 1: {:?}", aoc_2023_rust_lib::solve_part(i, 1)); println!("Part 2: {:?}", aoc_2023_rust_lib::solve_part(i, 2)) }
            _ => { error!("Unknown part!"); exit(-1) }
        };
    }
}