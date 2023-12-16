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

#[derive(Debug, Clone, Copy)]
struct Laser {
    location: (u32, u32),
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

    if dir_out2.is_none() {
        (dir_out1, None)
    } else {
        (dir_out1, dir_out2)
    }
}


#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String> {

    let overall_map: BTreeMap<(u32, u32), Tile> = input.lines()
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
                    ((x as u32,y as u32), t)
                })
        })
        .collect();

    let mut max_x: u32;
    let mut max_y: u32;
    let mut heat_map: Vec<(u32,u32)> = vec![];
    let mut lasers: Vec<Option<Laser>> = vec![];

    // create starting laser and push to heatmap
    let starting_laser = Laser { location: (0,0), direction: Direction::Right };
    lasers.push(Some(starting_laser));
    heat_map.push(lasers.clone().first().unwrap().unwrap().location);

    // loop through each laser while all of them are not None
    println!("Starting While Loop:");
    while !lasers.clone().iter().all(|l| l.is_none()) {
        let _ = lasers.clone().iter()
            .map(|mut laser| {
                if laser.is_some() {
                    // maybe trade Vec<Option<Laser>> for HashMap or BTreeMap?
                    let (curr_x, curr_y) = laser.clone().unwrap().location;
                    let (dir1, dir2) = reflection_direction(laser.as_ref().unwrap().direction, *overall_map.get(&(curr_x,curr_y)).unwrap());
                    laser.unwrap().direction = dir1;
                    let mut temp_x1: u32 = curr_x;
                    let mut temp_y1: u32 = curr_y;
                    let mut temp_x2: u32 = curr_x;
                    let mut temp_y2: u32 = curr_y;

                    heat_map.push((curr_x,curr_y));

                    match dir1 {
                        Direction::Up => temp_y1= curr_y,
                        Direction::Right => temp_x1 = curr_x + 1,
                        Direction::Down => temp_y1 = curr_y + 1,
                        Direction::Left => temp_x1 = curr_x - 1
                    };
                    if temp_x1 < 0 || temp_x1 > 9 || temp_y1 < 0 || temp_y1 > 9 {
                        laser = &None::<Laser>;
                    } else {
                        laser.unwrap().location.0 = temp_x1;
                        laser.unwrap().location.1 = temp_y1;
                    }

                    if dir2.is_some() {

                        match dir2.unwrap() {
                            Direction::Up => temp_y2 -= 1,
                            Direction::Right => temp_x2 += 1,
                            Direction::Down => temp_y2 += 1,
                            Direction::Left => temp_x2 -= 1
                        };
                        if temp_x1 < 0 || temp_x1 > 9 || temp_y1 < 0 || temp_y1 > 9 {
                            laser = &None::<Laser>;
                        } else {
                            lasers.push(Some(Laser { location: (temp_x2, temp_y2), direction: dir2.unwrap() }));
                        }
                    }
                }
            });
    }

    println!("heat_map: {:?}",heat_map);
    // TODO: calculate uniques in heat_map

    Ok("0".to_string())
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
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = r".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....";
        assert_eq!("46", process(input)?);
        Ok(())
    }
}