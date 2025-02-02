use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{anychar, digit1};
use nom::IResult;
use nom::multi::{fold_many1, many0, many1, many_till, separated_list0, separated_list1};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::error::Error;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let result = fold_many1(
        many_till(anychar::<&str, Error<_>>, separated_pair(
            preceded(tag("mul("), digit1), tag(","), terminated(digit1, tag(")")))
        ),
        || { 0 },
        |acc: u32, item| {
            let val1 : u32 = item.1.0.parse().expect("should parse");
            let val2 : u32 = item.1.1.parse().expect("should parse");
            acc + (val1 * val2)
        }
    )
        (_input).expect("should_parse").1;
    Ok(result.to_string())
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