use std::fs;

use anyhow::Context;

pub fn main() -> anyhow::Result<()> {
    let input =
        fs::read_to_string("../fixtures/day4").context("Should have been able to read the file")?;

    println!("{}", part_one(&input));
    // println!("{}", part_two(&input));

    Ok(())
}

/// Directions from the `X` position. Add the tuple of values to the x,y of the `X` position to see
/// if it spells XMAS
const DIRECTIONS: &[&[(i32, i32); 3]; 8] = &[
    &[(0, 1), (0, 2), (0, 3)],       // RIGHT (y increases)
    &[(-1, 0), (-2, 0), (-3, 0)],    // UP (x decreases)
    &[(0, -1), (0, -2), (0, -3)],    // LEFT (y decreases)
    &[(1, 0), (2, 0), (3, 0)],       // DOWN (x increases)
    &[(1, -1), (2, -2), (3, -3)],    // DIAGONAL DOWN LEFT
    &[(-1, -1), (-2, -2), (-3, -3)], // DIAGONAL UP LEFT
    &[(-1, 1), (-2, 2), (-3, 3)],    // DIAGONAL UP RIGHT
    &[(1, 1), (2, 2), (3, 3)],       // DIAGONAL DOWN RIGHT
];

fn get_potential_occurrences(matrix: &[Vec<char>], x: i32, y: i32) -> u32 {
    let mut occurrences: u32 = 0;
    for direction in DIRECTIONS {
        let [m, a, s] = direction;
        let m_pos = (x + m.0, y + m.1);
        let a_pos = (x + a.0, y + a.1);
        let s_pos = (x + s.0, y + s.1);

        // check if position is valid
        if m_pos.0 < 0
            || m_pos.0 as usize >= matrix.len()
            || m_pos.1 < 0
            || m_pos.1 as usize >= matrix[m_pos.0 as usize].len()
            || a_pos.0 < 0
            || a_pos.0 as usize >= matrix.len()
            || a_pos.1 < 0
            || a_pos.1 as usize >= matrix[a_pos.0 as usize].len()
            || s_pos.0 < 0
            || s_pos.0 as usize >= matrix.len()
            || s_pos.1 < 0
            || s_pos.1 as usize >= matrix[s_pos.0 as usize].len()
        {
            continue;
        }

        if matrix[m_pos.0 as usize][m_pos.1 as usize] == 'M'
            && matrix[a_pos.0 as usize][a_pos.1 as usize] == 'A'
            && matrix[s_pos.0 as usize][s_pos.1 as usize] == 'S'
        {
            occurrences += 1;
        }
    }

    occurrences
}

fn part_one(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    // convert to vec<vec<char>>
    let matrix = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut occurrences: u32 = 0;
    for x in 0..matrix.len() {
        for y in 0..matrix[x].len() {
            let char = matrix[x][y];
            if char == 'X' {
                // Need to get all potential occurrences of X
                occurrences += get_potential_occurrences(&matrix, x as i32, y as i32);
            }
        }
    }

    occurrences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(18, part_one(input));
        Ok(())
    }
}
