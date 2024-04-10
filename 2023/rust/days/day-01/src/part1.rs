use crate::custom_error::AocError;
use tracing::{debug, log::info};
#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    info!("Solving Day1, part 1");
    let result = _input
        // split into lines
        .lines()
        // map each line to digit
        .map(|line| {
            debug!("Analyzing line {line}");
            // get first digit
            let first = line
                .chars()
                .find_map(|c| c.to_digit(10))
                .expect("Should be a number!");
            // get last digit
            let last = line
                .chars()
                .rev()
                .find_map(|c| c.to_digit(10))
                .expect("Should be a number!");
            // return concatenated digit
            first * 10 + last
            // sum all the digits
        })
        .sum::<u32>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
