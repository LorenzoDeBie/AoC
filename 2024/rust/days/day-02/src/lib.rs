use nom::character::complete;
use nom::character::complete::space1;
use nom::character::streaming::newline;
use nom::IResult;
use nom::multi::separated_list1;

pub mod custom_error;

pub mod part1;
pub mod part2;

type Report = Vec<u32>;

fn parse(_input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(
        newline,
        separated_list1(space1, complete::u32)
    )(_input)
}