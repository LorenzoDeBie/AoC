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
    _id: u8,
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
            _id: id.parse().expect("game id should be a parseable number"),
            rounds,
        },
    ))
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let result = _input
        .lines()
        .map(|line| {
            let (_, game) = parse_game(line).expect("each line should contain a valid game");
            let mut min_red = u32::MIN;
            let mut min_blue = u32::MIN;
            let mut min_green = u32::MIN;
            game.rounds.iter().for_each(|round| {
                round.cubes.iter().for_each(|cube| match cube.color {
                    CubeColor::Red => {
                        if cube.amount > min_red {
                            min_red = cube.amount
                        }
                    }
                    CubeColor::Green => {
                        if cube.amount > min_green {
                            min_green = cube.amount
                        }
                    }
                    CubeColor::Blue => {
                        if cube.amount > min_blue {
                            min_blue = cube.amount
                        }
                    }
                });
            });
            min_red * min_blue * min_green
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
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
