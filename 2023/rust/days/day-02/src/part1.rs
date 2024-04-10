use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::custom_error::AocError;

enum CubeColor {
    Red,
    Green,
    Blue,
}

struct Cube {
    amount: u32,
    color: CubeColor,
}

struct Round {
    cubes: Vec<Cube>,
}

struct Game {
    id: u8,
    rounds: Vec<Round>,
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    let colour = match color {
        "red" => CubeColor::Red,
        "blue" => CubeColor::Blue,
        "green" => CubeColor::Green,
        _ => panic!("Only red, blue or green cubes should exist"),
    };
    Ok((
        input,
        Cube {
            amount,
            color: colour,
        },
    ))
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, cubes) = separated_list1(tag(", "), parse_cube)(input)?;
    Ok((input, Round { cubes }))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), parse_round))(input)?;
    Ok((
        input,
        Game {
            id: id.parse().expect("game id should be a parseable number"),
            rounds,
        },
    ))
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let result = _input
        .lines()
        .filter_map(|line| -> Option<u32> {
            let (_, game) = parse_game(line).expect("each line should contain a valid game");
            let valid = game.rounds.iter().all(|round| {
                round.cubes.iter().all(|cube| match cube.color {
                    CubeColor::Red => cube.amount <= 12,
                    CubeColor::Green => cube.amount <= 13,
                    CubeColor::Blue => cube.amount <= 14,
                })
            });
            if valid {
                Some(u32::from(game.id))
            } else {
                None
            }
        })
        .sum::<u32>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
