use crate::custom_error::AocError;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{anychar, digit1};
use nom::combinator::map;
use nom::error::Error;
use nom::multi::{fold_many1, many1, many_till};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::{IResult, InputTake};

enum Ops {
    Do,
    Dont,
    Mult(u32, u32),
}

fn parse(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many1(map(
        many_till(
            anychar::<&str, Error<_>>,
            separated_pair(
                preceded(tag("mul("), digit1),
                tag(","),
                terminated(digit1, tag(")")),
            ),
        ),
        |o1| o1.1,
    ))(input)
}

fn parse_mul(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        digit1,
        tag(","),
        terminated(digit1, tag(")")),
    )(input)
}

fn parse_until_op(input: &str) -> IResult<&str, &str> {
    let result = preceded(
        take_till(|c| c == 'm' || c == 'd'),
        alt((
            tag("do()"),
            tag("don't()"),
            tag("mul("),
        )),
    )(input);
    result
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut remainder = _input;
    let mut op = "";
    let mut mults : Vec<(&str, &str)> = Vec::new();
    let mut enabled = true;
    while remainder != "" {
        match parse_until_op(remainder) {
            Ok((r, o)) => {remainder = r; op = o;}
            Err(_) => {remainder = &remainder[1..]; continue}
        }
        match op {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            "mul(" => {
                if !enabled { continue; }
                match parse_mul(remainder) {
                    Ok((r, m)) => {
                        mults.push(m);
                        remainder = r;
                    }
                    Err(_) => {
                        remainder = &remainder[1..];
                        continue;
                    }
                }
            }
            _ => todo!("todo"),
        }
    }
    Ok(mults
        .iter()
        .map(|(x, y)| -> u32 {
            x.parse::<u32>().expect("should parse") * y.parse::<u32>().expect("should parse")
        })
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
