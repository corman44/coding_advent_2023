
use tracing::{info, event, span, Level};
use core::num;
use std::collections::{HashMap, BTreeMap};


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    MirrorRight,
    MirrorLeft,
    VerticalSplit,
    HorizontalSplit,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Laser {
    location: (i32, i32),
    direction: Direction,
}

fn reflection_direction(dir_in: Direction, tile: Tile) -> (Direction, Option<Direction>) {
    let mut dir_out1: Direction;
    let mut dir_out2: Option<Direction> = None;

    match dir_in {
        Direction::Up => match tile {
            Tile::Empty => dir_out1 = dir_in,
            Tile::MirrorRight => dir_out1 = Direction::Right,
            Tile::MirrorLeft => dir_out1 = Direction::Left,
            Tile::VerticalSplit => dir_out1 = dir_in,
            Tile::HorizontalSplit => {
                dir_out1 = Direction::Left;
                dir_out2 = Some(Direction::Right);
            },
        },
        Direction::Right => match tile {
            Tile::Empty => dir_out1 = dir_in,
            Tile::MirrorRight => dir_out1 = Direction::Up,
            Tile::MirrorLeft => dir_out1 = Direction::Down,
            Tile::VerticalSplit => {
                dir_out1 = Direction::Up;
                dir_out2 = Some(Direction::Down);
            },
            Tile::HorizontalSplit => dir_out1 = dir_in,
        },
        Direction::Down => match tile {
            Tile::Empty => dir_out1 = dir_in,
            Tile::MirrorRight => dir_out1 = Direction::Left,
            Tile::MirrorLeft => dir_out1 = Direction::Right,
            Tile::VerticalSplit => dir_out1 = dir_in,
            Tile::HorizontalSplit => {
                dir_out1 = Direction::Left;
                dir_out2 = Some(Direction::Right);
            },
        },
        Direction::Left => match tile {
            Tile::Empty => dir_out1 = dir_in,
            Tile::MirrorRight => dir_out1 = Direction::Down,
            Tile::MirrorLeft => dir_out1 = Direction::Up,
            Tile::VerticalSplit => {
                dir_out1 = Direction::Up;
                dir_out2 = Some(Direction::Down);
            },
            Tile::HorizontalSplit => dir_out1 = dir_in,
        },
    };
    (dir_out1, dir_out2)
}


#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String> {
    info!("starting process()");

    let overall_map: BTreeMap<(i32, i32), Tile> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().chars()
                .enumerate()
                .map(move |(x, c)| {
                    let t = match c {
                        '/' => Tile::MirrorRight,
                        '\\' => Tile::MirrorLeft,
                        '|' => Tile::VerticalSplit,
                        '-' => Tile::HorizontalSplit,
                        _ => Tile::Empty,
                    };
                    ((x as i32,y as i32), t)
                })
        })
        .collect();
    let val = overall_map.iter().next_back().unwrap();
    let max_x = val.0.0;
    let max_y = val.0.1;

    // TODO: build vec of starting lasers
    let mut all_starting_lasers: Vec<Laser> = vec![];
    for i in 0..max_x+1 {
        all_starting_lasers.push( Laser { location: (i,0), direction: Direction::Down });
        all_starting_lasers.push( Laser { location: (i,max_y), direction: Direction::Up });
        all_starting_lasers.push( Laser { location: (0, i), direction: Direction::Right });
        all_starting_lasers.push( Laser { location: (max_x,i), direction: Direction::Left });
    }

    let mut highest_count = 0;

    for starting_laser in all_starting_lasers.iter() {
        // create starting laser and push to heatmap
        let mut heat_btmap: BTreeMap<(i32,i32), i32> = BTreeMap::new();
        let mut active_lasers: Vec<Laser> = vec![*starting_laser];
        heat_btmap.entry(active_lasers.clone().first().unwrap().location).or_insert(1);

        let mut heatmap_count = 0;
        let mut concurrent_checks = 0;

        loop  {
            let mut next_lasers: Vec<Laser> = vec![];
            if active_lasers.is_empty() {
                break;
            }

            for laser in active_lasers.iter() {
                let (curr_x, curr_y) = laser.location;
                let (dir1, dir2) = reflection_direction(laser.direction, *overall_map.get(&(curr_x,curr_y)).unwrap());
                let mut temp_x1: i32 = curr_x;
                let mut temp_y1: i32 = curr_y;
                let mut temp_x2: i32 = curr_x;
                let mut temp_y2: i32 = curr_y;

                let temp = heat_btmap.entry((curr_x,curr_y)).or_insert(0);
                *temp += 1;

                match dir1 {
                    Direction::Up => temp_y1 -= 1,
                    Direction::Right => temp_x1 += 1,
                    Direction::Down => temp_y1 +=1,
                    Direction::Left => temp_x1 -=1,
                };
                if !(temp_x1 < 0 || temp_x1 > max_x || temp_y1 < 0 || temp_y1 > max_y) {
                    next_lasers.push( Laser { location: (temp_x1, temp_y1), direction: dir1 });
                }

                if dir2.is_some() {
                    match dir2.unwrap() {
                        Direction::Up => temp_y2 -= 1,
                        Direction::Right => temp_x2 += 1,
                        Direction::Down => temp_y2 += 1,
                        Direction::Left => temp_x2 -= 1
                    };
                    if !(temp_x2 < 0 || temp_x2 > max_x || temp_y2 < 0 || temp_y2 > max_y) {
                        next_lasers.push(Laser { location: (temp_x2, temp_y2), direction: dir2.unwrap() });
                    }
                }
            }
            active_lasers = next_lasers;
            if heat_btmap.len() > heatmap_count {
                heatmap_count = heat_btmap.len();
                concurrent_checks = 20;
            } else if concurrent_checks <= 0 {
                if highest_count < heat_btmap.len() {
                    highest_count = heat_btmap.len();
                    println!("Latest highest: {}",highest_count);
                }
                break;
            } else {
                concurrent_checks -= 1;
            }
        }
    }
    println!("heat_map length: {}",highest_count);
    Ok(highest_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflection_fn() -> miette::Result<()> {
        assert_eq!((Direction::Right, None), reflection_direction(Direction::Right, Tile::Empty));
        assert_eq!((Direction::Left, None), reflection_direction(Direction::Left, Tile::HorizontalSplit));
        assert_eq!((Direction::Up, Some(Direction::Down)), reflection_direction(Direction::Right, Tile::VerticalSplit));
        assert_eq!((Direction::Left, Some(Direction::Right)), reflection_direction(Direction::Down, Tile::HorizontalSplit));
        assert_eq!(reflection_direction(Direction::Left, Tile::VerticalSplit), (Direction::Up, Some(Direction::Down)));
        Ok(())
    }

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        info!("starting test");
        let input = r#".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|...."#;
        assert_eq!("51", process(input)?);
        Ok(())
    }
}