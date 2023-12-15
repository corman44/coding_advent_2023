use std::{str::FromStr, iter::Map, collections::BTreeMap};

use itertools::Itertools;
use glam::IVec2;

use tracing::{info, span, Level};

#[derive(Debug,Clone,PartialEq,Eq)]
enum Space {
    Empty,
    Galaxy,
}

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
) -> miette::Result<String> {
    
    let mut overall_map: Vec<Vec<Space>> = input.lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(move |val|{
                    let ret_val;
                    if val == '#' {
                        ret_val = Space::Galaxy;
                    } else {
                        ret_val = Space::Empty;
                    }
                    ret_val
                })
                .collect::<Vec<Space>>()
        })
        .collect_vec();
    println!("Galaxy Rows: {}", &overall_map.len());
    println!("Galaxy Cols: {}", &overall_map[0].len());
    println!("Original Galaxy: {:?}", &overall_map);

    let empty_rows: Vec<usize> = overall_map.clone()
        .into_iter()
        .enumerate()
        .filter(|(_, row )| !row.contains(&Space::Galaxy))
        .map(|(row_num, _)| row_num)
        .collect();

    let mut columns = vec![];
    let range = 0..input.lines().enumerate().fold(0, |acc, _| acc + 1)-1;
    for idx in range {
        columns.push(
            input.lines()
                .into_iter()
                .map(|row| {
                    let s = row.trim().chars().nth(idx).unwrap();
                    let ret_val;
                    if s == '#' {
                        ret_val = Space::Galaxy;
                    } else {
                        ret_val = Space::Empty;
                    }
                    ret_val
                })
                .collect::<Vec<Space>>()
        );
    }

    let empty_cols: Vec<usize> = columns.clone().iter()
        .enumerate()
        .filter(move |(_, col)| !col.contains(&Space::Galaxy))
        .map(|(col_num, _)| col_num)
        .collect();
    println!("Empty Cols: {:?}",empty_cols);

    let mut extend_count = 0;
    let _ = empty_rows.iter()
        .map(|row_num| {
            let _ = overall_map.insert(row_num + extend_count, vec![]);
            extend_count += 1;
        });
    println!("Empty Rows: {:?}", empty_rows);

    let mut extend_count = 0;
    let _ = empty_cols.iter()
        .map(|row_num| {
            let _ = overall_map.insert(row_num + extend_count, vec![]);
            extend_count += 1;
        });
    println!("Galaxy Rows: {}", overall_map.len());
    println!("Galaxy Cols: {}", overall_map[0].len());
    println!("Expanded Galaxy: {:?}", overall_map);


    // TODO: find closest galaxy to each galaxy

    // TODO: return sum of distances

    return Ok("0".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";
        assert_eq!("374", process(input)?);
        Ok(())
    }
}