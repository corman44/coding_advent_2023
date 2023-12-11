use std::{str::FromStr, iter::Map, collections::BTreeMap};

use itertools::Itertools;
use multidimension::{Index,Array};

#[derive(Debug,Clone,PartialEq,Eq)]
enum Space {
    Empty,
    Galaxy,
}

#[tracing::instrument]
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

    let empty_rows: Vec<usize> = overall_map.clone()
        .into_iter()
        .enumerate()
        .filter(|(_, row )| !row.contains(&Space::Galaxy))
        .map(|(row_num, _)| row_num)
        .collect();
    println!("empty_rows: {:?}",empty_rows);

    let mut extend_count = 0;
    let _ = empty_rows.iter()
        .map(|row_num| {
            let _ = &overall_map.insert(row_num + extend_count, vec![]);
            extend_count += 1;
        });
    println!("extended rows map:{:?}",overall_map);

    // TODO: find empty cols
    // let better_map = Array::from(overall_map);
    let empty_cols: Vec<usize> = overall_map.clone().iter()
        .enumerate()
        .filter(move |(_, col)| col.contains(&Space::Galaxy))
        .map(|(col_num, _)| col_num)
        .collect();

    // TODO: expand galaxy

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