use std::cmp::Ordering;
use itertools::Itertools;
use crate::custom_error::AocError;
use crate::{parse, Report};

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let reports = parse(_input).expect("Failed to parse").1;

    let result = reports.iter().map(|report| -> bool {
        let mut increasing = true;
        let mut decreasing = true;
        for (a, b) in report.iter().tuple_windows::<(_,_)>() {
            // check if keeps increasing or keeps decreasing
            match a.cmp(b) {
                Ordering::Greater => increasing = false,
                Ordering::Less => decreasing = false,
                Ordering::Equal => return false,

            }

            if !(increasing || decreasing) { return false }

            // Check if difference is not to much
            let diff = a.abs_diff(*b);
            if !(1..=3).contains(&diff) { return false }
        }
        true
    }).filter(|x| *x).count().to_string();


    Ok(result)
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}