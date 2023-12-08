extern crate nom;

use std::env::current_dir;
use std::fs;
use std::os::raw;
use std::path::PathBuf;
use std::str::FromStr;
use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1, digit1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser, number, combinator::map_res,
};
use nom_supreme::ParserExt;

fn main() {
    let cwd = current_dir().unwrap();
    let input_fp = PathBuf::from_str((cwd.to_str().unwrap().to_owned() + "\\src\\6\\input").as_str());    
    let raw_input = fs::read_to_string(input_fp.unwrap()).expect("File doesn't exist :'(");

    let numbers: Vec<(&str, Vec<i64>)> = raw_input
        .lines()
        .into_iter()
        .map(|s| parse_nums(s).unwrap())
        .collect();

    let (_, times) = &numbers[0];
    let (_, distance) = &numbers[1];

    let overall_time = times.join("");

    let result = times.into_iter()
        .zip(distance)
        .map(|(time, dist)| {
            (0..*time)
                .filter_map(|speed| {
                    let my_dist = (time - speed) * speed;
                    (my_dist > *dist)
                        .then_some(my_dist)
                })
                .count()
        })
        .product::<usize>();

    println!("{}",result);
}

fn parse_big_num(input: &str) -> IResult<&str, i64> {
    is_not("0123456789")
        .precedes(separated_list1(space1, digit1).map(
            |list| {
                list.join("")
                    .parse::<i64>()
                    .expect("a valid number")
            },
        ))
        .parse(input)
}

fn parse_nums(input: &str) -> IResult<&str, Vec<i64>> {
    is_not("0123456789")
        .precedes( separated_list1(space1, complete::i64))
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_nums("test:  12   34  56").unwrap(), ("", Vec::from([12,34,56])));
    }
}