use std::collections::HashMap;
use glam::IVec2;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {

    // Vecs for each M and S we can find from an A position
    const DIRECTIONS: [[IVec2; 2]; 4] = [
        [IVec2::new(1, 1), IVec2::new(-1, -1)],
        [IVec2::new(-1, -1), IVec2::new(1, 1)],
        [IVec2::new(1, -1), IVec2::new(-1, 1)],
        [IVec2::new(-1, 1), IVec2::new(1, -1)],
    ];

    const SEQUENCE: [char; 2] = ['M', 'S'];

    // Construct grid in a Hashmap with IVec2 as keys and the char as value
    let grid = _input.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (IVec2::new(x as i32, y as i32), c)
            })
        }).collect::<HashMap<_, _>>();


    let result : usize = grid.iter()
        // only concerned with A as start position
        .filter(|(_pos, c)| { **c == 'A' })
        .filter(|(pos, _c)| {
            // for each A we iter over all possible directions
            DIRECTIONS.iter()
                .map(|mas_positions| {
                    mas_positions.iter()
                        .map(|mas_offset| {
                            // Get the character at the offset
                            grid.get(&(**pos + *mas_offset))
                        })
                        .enumerate()
                        .all(|(idx, c)| {
                            // check if char matches with M or S at offset
                            SEQUENCE.get(idx) == c
                        })
                })
                .filter(|b| *b)
                .count()
                // Now we need to have exactly 2 matches instead of just counting them
                == 2
        })
        .count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", process(input)?);
        Ok(())
    }
}