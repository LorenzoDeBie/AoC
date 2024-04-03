use crate::custom_error::AocError;
#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let result = _input.lines().map(|l| {
        let it = [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("zero", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("0", 0)
        ].iter().filter_map(|(p, v)| {
            // match l.find(p) {
            //     Some(i) => Some((i, v)),
            //     None => None,
            // }
            l.find(p).map(|i| (i, v))

        });
        
        let (_, min) = it.min_by(|(i, _), (i2, _)| i.cmp(i2) ).expect("should be a number");
        

        let it = [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("zero", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("0", 0)
        ].iter().filter_map(|(p, v)| {
            // match l.find(p) {
            //     Some(i) => Some((i, v)),
            //     None => None,
            // }
            l.rfind(p).map(|i| (i, v))

        });
        
        let (_, max) = it.max_by(|(i, _), (i2, _)| i.cmp(i2) ).expect("should be a number");
        
         min * 10 + max
    }).sum::<usize>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }

    #[test]
    fn line_test() -> miette::Result<()> {
        let input = "6nfhcklxlkg9jbqmqrrxmhn9two6";
        assert_eq!("66", process(input)?);
        Ok(())
    }
}