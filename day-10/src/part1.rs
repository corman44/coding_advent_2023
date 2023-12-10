use std::{error::Error, str::FromStr, iter::Map, collections::BTreeMap};

use glam::IVec2;
use nom::Err;
use nom_locate::LocatedSpan;
use tracing::info;

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Tile {
    Vertical,
    Horizontal,
    NEBend,
    NWBend,
    SWBend,
    SEBend,
    Ground,
    Start,
}

impl FromStr for Tile {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("Matching {}", s);
        match s {
            "|" => Ok(Self::Vertical),
            "-" => Ok(Self::Horizontal),
            "L" => Ok(Self::NEBend),
            "J" => Ok(Self::NWBend),
            "7" => Ok(Self::SWBend),
            "F" => Ok(Self::SEBend),
            "." => Ok(Self::Ground),
            "S" => Ok(Self::Start),
            _ => Err("can't tranlate char"),
        }
    }
}


#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn map_tile_new_direction(tile: Tile, dir: Direction) -> Result<Direction, &'static str>  {
    match tile {
        Tile::Vertical => {
            if dir == Direction::North {Ok(Direction::South)}
            else { Ok(Direction::North) }
        },
        Tile::Horizontal => todo!(),
        Tile::NEBend => todo!(),
        Tile::NWBend => todo!(),
        Tile::SWBend => todo!(),
        Tile::SEBend => todo!(),
        Tile::Ground => Err("shouldn't reach"),
        Tile::Start => Err("shouldn't reach"),
    }
}

pub fn get_new_pos(
    curr_x: i32,
    curr_y: i32,
    direction: Direction,
    map: BTreeMap<(i32,i32), Tile>
) -> ((i32,i32), Direction) {
    match direction {
        Direction::North => todo!(),
        Direction::South => todo!(),
        Direction::East => todo!(),
        Direction::West => todo!(),
    }

    todo!("get new posiiton")
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String> {

    // create map from input Vec<Vec<Map<(x,y), Tile>
    let overall_map = input.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| {
                    ((x as i32, y as i32), Tile::from_str(c.to_string().as_str()).unwrap())
                })
        })
        .collect::<BTreeMap<(i32, i32), Tile>>();

    // create map of valid values
    let start: Vec<(i32,i32)>= overall_map.iter()
        .filter(|((_,_), tile)| **tile == Tile::Start)
        .map(|((x,y), _)| (*x,*y))
        .collect();
    let (start_x, start_y) = start[0];
    println!("Start: {:?}", start);

    // find all connecting pipes from started
    let mut loop_complete = false;
    let mut current_x = &start_x;
    let mut current_y = &start_y;
    let mut counter = 0;
    while !loop_complete {

        // change current_x and current_y


        // end condition
        if *current_x == start_x && *current_y == start_y {
            loop_complete = true;
        }
        counter += 1;
    }
    // find loop from connecting pipes from start

    // count steps from start

    Ok("()".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile() -> miette::Result<()> {

        let s = 'S';
        let l = 'L';
        let j = 'J';

        assert_eq!(Tile::from_str(s.to_string().as_str()), Ok(Tile::Start));
        assert_eq!(Tile::from_str(l.to_string().as_str()), Ok(Tile::NEBend));
        assert_eq!(Tile::from_str(j.to_string().as_str()), Ok(Tile::NWBend));

        Ok(())
    }

    #[test]
    fn test_mapping() -> miette::Result<()> {
        let input1 = ".....
        .S-7.
        .|.|.
        .L-J.
        .....";
        
        let expected_map1 = ".....
        .012.
        .1.3.
        .234.
        .....";

        assert_eq!(expected_map1, process(input1)?);
        Ok(())
    }
}