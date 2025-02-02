use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    // split lines into columns & sort
    let mut c1: Vec<i32> = vec![];
    let mut c2 : Vec<i32> = vec![];
    for line in _input.lines() {
        c1.push(line[0..5].parse::<i32>().unwrap());
        c2.push(line[8..13].parse::<i32>().unwrap());
    }
    c1.sort(); c2.sort();

    // loop over both colums and sum the difference
    Ok(c1.iter().zip(c2.iter()).map(|it| {
        (it.1 - it.0).abs()
    }).sum::<i32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        Ok(())
    }
}