use std::collections::BTreeMap;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    // split lines into columns & sort
    let mut c1 = vec![];
    let mut c2= BTreeMap::new();
    for line in _input.lines() {

        let n1 = line[0..5].parse::<i32>().unwrap();
        let n2 = line[8..13].parse::<i32>().unwrap();
        c1.push(n1);
        c2.entry(n2).and_modify(|frq| *frq += 1).or_insert(1);
    }

    Ok(c1.iter().map(|n| {
        *n * c2.get(n).or(Some(&0)).unwrap()
    }).sum::<i32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}