/* -------------------------------- Statement ------------------------------- */
// Given a square grid of characters in the range ascii[a-z],
// rearrange elements of each row alphabetically, ascending.
// Determine if the columns are also in ascending alphabetical order, top to bottom.
// Return YES if they are or NO if they are not.

// Each string consists of lowercase letters in the range ascii[a-z]
// n of grid in 0..=100

/* ------------------------------------ x ----------------------------------- */

// Check rows for ascending order else rearrange
// Check cols for ascending order; if true for all cols print YES

fn solution(grid: &[String]) -> String {
    let yes = "YES".to_string();
    let no = "NO".to_string();
    let mut grid = grid.to_vec();

    let rearranges = check_rows(&grid);
    if !rearranges.is_empty() {
        rearrange_row(&mut grid, &rearranges);
    }

    if check_columns(&grid) {
        yes
    } else {
        no
    }
}

/// Only sorts strings with either just lowercase chars or just uppercase chars
fn sort_string(input: &str) -> String {
    if input.is_empty() {
        return input.to_string();
    }
    let mut row = input.chars().map(|c| c as u8).collect::<Vec<u8>>();
    row.sort();
    row.iter().map(|&v| v as char).collect::<String>()
}

fn rearrange_row(grid: &mut [String], rearranges: &[usize]) {
    for &index in rearranges {
        let row = grid.get(index).unwrap();
        *grid.get_mut(index).unwrap() = sort_string(&row);
    }
}

fn check_rows(grid: &[String]) -> Vec<usize> {
    let mut result = Vec::new();
    for (rid,row) in grid.iter().enumerate() {
        let mut prev = 0;
        for (cid, c) in row.chars().enumerate() {
            if cid == 0 {
                prev = c as u8;
                continue;
            }
            let curr = c as u8;
            if curr < prev {
                result.push(rid);
            }
            prev = curr;
        }
    }
    result
}

/// grid is n x n matrix
fn check_columns(grid: &[String]) -> bool {
    let mut j = 0;
    let mut prev = 0;
    let num_cols = grid.get(0).unwrap().len();
    let num_rows = grid.len();

    let grid = grid
        .iter()
        .map(|row| row.chars().map(|c| c as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    while j != num_cols {
        for i in 0..num_rows {
            if i == 0 {
                prev = grid[0][j];
                continue;
            }
            if grid[i][j] < prev {
                return false;
            }
        }
        j+=1;
    }

    true
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_string_sort() {
        assert_eq!("abcdef", sort_string("acbefd"));
        assert_eq!("ABCDEF", sort_string("ADCBFE"));
    }

    #[test]
    fn test_solution() {
        let grid = [
            "ebacd".to_owned(),
            "fghij".to_owned(),
            "olmkn".to_owned(),
            "trpqs".to_owned(),
            "xywuv".to_owned(),
        ];

        assert_eq!(solution(&grid), "YES".to_owned());
    }

    #[test]
    fn test_multiple_runs(){
        let grid = [
            "abc".to_owned(),
            "lmp".to_owned(),
            "qrt".to_owned(),
        ];
        assert_eq!(solution(&grid), "YES".to_owned());
    
        let grid = [
            "mpxz".to_owned(),
            "abcd".to_owned(),
            "wlmf".to_owned(),
        ];
        assert_eq!(solution(&grid), "NO".to_owned());
        
        let grid = [
            "abc".to_owned(),
            "hjk".to_owned(),
            "mpq".to_owned(),
            "rtv".to_owned(),
        ];

        assert_eq!(solution(&grid), "YES".to_owned());
    }
}
