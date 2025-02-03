use std::collections::HashMap;
use glam::IVec2;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    
    // Vecs for each MAS we need to find from a X position
    const DIRECTIONS: [[IVec2; 3]; 8] = [
        [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)],
        [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)],
        [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)],
        [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)],
        [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)],
        [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)],
        [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)],
        [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)]
    ];
    
    const SEQUENCE: [char; 3] = ['M', 'A', 'S'];
    
    // Construct grid in a Hashmap with IVec2 as keys and the char as value
    let grid = _input.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (IVec2::new(x as i32, y as i32), c)
            })
        }).collect::<HashMap<_, _>>();

    
    let result : usize = grid.iter()
        // only concerned with X as startposition
        .filter(|(_pos, c)| { **c == 'X' })
        .map(|(pos, _c)| {
            // for each X we iter over all possible directions
            DIRECTIONS.iter()
                .map(|mas_positions| {
                    mas_positions.iter()
                        .map(|mas_offset| {
                            // Get the character at the offset
                            grid.get(&(*pos + *mas_offset))
                        })
                        .enumerate()
                        .all(|(idx, c)| {
                            // check if char matches with MAS at offset
                            SEQUENCE.get(idx) == c
                        })
                })
                .filter(|b| *b)
                .count()
        })
        .sum();

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
        assert_eq!("18", process(input)?);
        Ok(())
    }
}