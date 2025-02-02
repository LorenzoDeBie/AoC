use crate::custom_error::AocError;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, digit1};
use nom::combinator::map;
use nom::error::Error;
use nom::multi::{many1, many_till};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

fn parse_mult_ops(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
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

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mult_ops = parse_mult_ops(_input).expect("should parse").1;

    Ok(mult_ops
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
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
