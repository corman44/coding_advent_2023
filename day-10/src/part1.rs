use std::{error::Error, str::FromStr, iter::Map, collections::BTreeMap};

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

fn get_next_pos(
    curr_x: &i32,
    curr_y: &i32,
    direction: &Direction,
    map: &BTreeMap<(i32,i32), Tile>
) -> Result<((i32,i32), Direction), &'static str> {
    match direction {
        Direction::North => {
            match map.get(&(*curr_x, *curr_y)).unwrap() {
                Tile::Vertical => { Ok(((*curr_x,curr_y+1), Direction::North)) },
                Tile::NEBend => { Ok(((curr_x+1,*curr_y), Direction::West)) },
                Tile::NWBend => { Ok(((curr_x-1,*curr_y), Direction::East)) },
                _ => Err("shouldn't reach here")
            }
        },
        Direction::South => {
            match map.get(&(*curr_x, *curr_y)).unwrap() {
                Tile::Vertical => { Ok(((*curr_x,curr_y-1), Direction::South)) },
                Tile::SEBend => { Ok(((curr_x+1,*curr_y), Direction::West)) },
                Tile::SWBend => { Ok(((curr_x-1,*curr_y), Direction::East)) },
                _ => Err("shouldn't reach here")
            }
        },
        Direction::East => {
            match map.get(&(*curr_x, *curr_y)).unwrap() {
                Tile::Horizontal => { Ok(((curr_x-1,*curr_y), Direction::East)) },
                Tile::NEBend => { Ok(((*curr_x,curr_y-1), Direction::South)) },
                Tile::SEBend => { Ok(((*curr_x,curr_y+1), Direction::North)) },
                _ => Err("shouldn't reach here")
            }
        },
        Direction::West => {
            match map.get(&(*curr_x, *curr_y)).unwrap() {
                Tile::Horizontal => { Ok(((curr_x+1,*curr_y), Direction::West)) },
                Tile::SWBend => { Ok(((*curr_x,curr_y+1), Direction::North)) },
                Tile::NWBend => { Ok(((*curr_x,curr_y-1), Direction::South)) },
                _ => Err("shouldn't reach here")
            }
        },
    }
}

fn move_from_first_pos(x: &i32, y: &i32, map: &BTreeMap<(i32,i32), Tile>) -> Result<((i32, i32), Direction), &'static str> {
    // check each direction and return if finding valid move (include direction)

    // check North
    if map.get(&(*x,y-1)).unwrap() == &Tile::Vertical ||
        map.get(&(*x,y-1)).unwrap() == &Tile::SEBend ||
        map.get(&(*x,y-1)).unwrap() == &Tile::SWBend {
        return Ok(((*x,y-1), Direction::South))
    }
    // check East
    if map.get(&(x+1,*y)).unwrap() == &Tile::Horizontal ||
        map.get(&(x+1,*y)).unwrap() == &Tile::NWBend ||
        map.get(&(x+1,*y)).unwrap() == &Tile::SWBend {
            return Ok(((x+1,*y), Direction::West))
    }
    // check South
    if map.get(&(*x,y+1)).unwrap() == &Tile::Horizontal ||
        map.get(&(*x,y+1)).unwrap() == &Tile::NWBend ||
        map.get(&(*x,y+1)).unwrap() == &Tile::SWBend {
            return Ok(((*x,y+1), Direction::North))
    } else {
        return Err("can't find first position");
    }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String> {

    // create map from input Vec<Vec<Map<(x,y), Tile>
    let overall_map = input.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(x, c)| {
                    // println!("char: {}",c);
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
    // println!("Start: {:?}", start);

    let mut counter = 0;
    // move once
    let  ((mut current_x, mut current_y), mut next_dir) = move_from_first_pos(&start_x, &start_y, &overall_map).unwrap();
    // println!("1st move (X,Y) Tile: ({}, {}), {:?}, {:?}", &current_x, &current_y, overall_map.get(&(current_x,current_y)).unwrap(), &next_dir);
    while current_x != start_x || current_y != start_y {
        ((current_x, current_y), next_dir) = get_next_pos(&current_x, &current_y, &next_dir, &overall_map).unwrap();
        // println!("Debug: {}, {}, {:?}, {:?}", &current_x, &current_y, overall_map.get(&(current_x,current_y)).unwrap(), &next_dir);
        counter += 1;
    }

    if (counter % 2 == 1) {
        return Ok(format!("{}", counter/2 + 1))
    } else {
        return Ok(format!("{}", counter/2))
    }

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
        let input1 = "7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ";
        assert_eq!("8", process(input1).unwrap());
        Ok(())
    }
}