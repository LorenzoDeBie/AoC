use log::info;
use std::{error::Error, fs};

pub static NUM_DAYS: u8 = 2;

pub fn solve_part(day: u8, part: u8) -> Result<String, Box<dyn Error>> {
    info!("Fetching input data from local file");
    let input_path : String;

    input_path = format!("days/day-{:02}/input.txt", day);
    

    let input = match fs::read_to_string(input_path) {
        Ok(s) => s.clone(),
        Err(err) => Err(format!("Failed to read puzzle input from fs: {err}"))?
    };
    match (day, part) {
        (1, 1) => day_01::part1::process(&input).map_err(|e| e.into()),
        (1, 2) => day_01::part2::process(&input).map_err(|e| e.into()),
        (2, 1) => day_02::part1::process(&input).map_err(|e| e.into()),
        //(2, 2) => day_02::part2::process(&input).map_err(|e| e.into()),
        _ => Err("That day or part does not exist!")?
    }
}

pub fn solve_day(day: u8) -> (Result<String, Box<dyn Error>>, Result<String, Box<dyn Error>>) {
    (solve_part(day, 1), solve_part(day, 2))
}

pub fn solve_all_days() -> Vec<(Result<String, Box<dyn Error>>, Result<String, Box<dyn Error>>)> {
    let mut res = vec![];
    for i in 1..NUM_DAYS+1 {
        res.push(solve_day(i));
    }
    return res;
}
