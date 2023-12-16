use std::collections::{HashMap, BTreeMap};



#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    MirrorRight,
    MirrorLeft,
    VerticalSplit,
    HorizontalSplit,
}

#[derive(Debug)]
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

    let mut heat_map: Vec<(i32,i32)> = vec![];
    let mut lasers: Vec<Option<Laser>> = vec![];
    let starting_laser = Laser { location: (0,0), direction: Direction::Right };
    lasers.push(Some(starting_laser));

    // loop through each laser while all of them are not None
    while !lasers.iter().all(|l| l.is_none()) {
        let _ = lasers.iter()
            .map(|laser| {
                if laser.is_some() {
                    // maybe trade Vec<Option<Laser>> for HashMap or BTreeMap?
                    // TODO: get 1 or 2 lasers + direction from the incoming laser
                    // TODO: change current laser direction
                    // TODO: push 2nd laser if it splits
                }
            });
    }

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