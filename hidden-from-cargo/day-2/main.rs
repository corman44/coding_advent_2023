// Day2 Problem:

/*
Day 2: Cube Conundrum ---
You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many visitors up here; would you like to play a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.

You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration. However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.

Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?

To play, please identify yourself via one of these services:
*/

use std::env::current_dir;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

pub const MAX_RED: i32 = 12;
pub const MAX_GREEN: i32 = 13;
pub const MAX_BLUE: i32 = 14;

fn main() {
    let mut winning_ids: Vec<i32> = vec![];
    let mut all_games: Vec<Game> = vec![];
    let cwd = current_dir().unwrap();
    let input_fp = PathBuf::from_str((cwd.to_str().unwrap().to_owned() + "\\src\\2\\games.txt").as_str());
    // println!("input_fp: {:?}",input_fp.clone().unwrap());
    
    let raw_input = fs::read_to_string(input_fp.unwrap()).expect("File doesn't exist :'(");
    // println!("{}", raw_input);

    // parse games into Vec<Game>
    for game in raw_input.lines() {
        let colon_position = game.find(':').unwrap();
        let space_position = game.find(' ').unwrap();
        let game_id = &game[space_position..colon_position].trim().parse::<i32>().unwrap();
        let mut temp_sets: Vec<Set> = vec![];
        // println!("Game ID: {}", game_id);

        for sets in game.split(':').nth(1).unwrap().split(';') {
            let mut temp_set: Set = Set { red: 0, green: 0, blue: 0 };
            if sets.contains(',') {
                for set  in sets.split(',') {
                    let (color, number) = parse_color_number(set);
                    // println!("Color: {}", color);
                    if color == "red" {
                        // println!("found red!!");
                        temp_set.red = number;
                    } else if color == "green" {
                        temp_set.green = number;
                    } else if color == "blue" {
                        temp_set.blue = number;
                    }
                }
            } else {
                let (color, number) = parse_color_number(sets);
                if color == "red" {
                    // println!("found red");
                    temp_set.red = number;
                } else if color == "green" {
                    temp_set.green = number;
                } else if color == "blue" {
                    temp_set.blue = number;
                }
            }
            temp_sets.push(temp_set);
        }
        all_games.push( Game { id: *game_id, sets: temp_sets })
    }

    // println!("All Games: {:?}",all_games);

    for game in all_games.clone() {
        let mut possible = true;
        for set in game.sets {
            if !is_set_possible(set) {
                possible = false;
            }
        }
        if possible {
            winning_ids.push(game.id);
        }
    }

    println!("possible sum: {}", sum_ids(&winning_ids));
    println!("number possible: {}", winning_ids.len());

    let mut fewest_per_game: Vec<Fewest> = vec![];
    for game in all_games {
        let mut temp_set: Set = Set { red: 0, green: 0, blue: 0 };
        for set in game.sets {
            if set.red > temp_set.red {
                temp_set.red = set.red;
            }
            if set.green > temp_set.green{
                temp_set.green = set.green;
            }
            if set.blue > temp_set.blue {
                temp_set.blue = set.blue;
            }
        }
        fewest_per_game.push(Fewest { id: game.id, set: temp_set });
    }

    let mut power: Vec<i32> = vec![];
    for game in fewest_per_game {
        power.push(game.set.blue * game.set.red * game.set.green);
    }
    println!("sum of powers: {}", sum_ids(&power));
}

pub fn parse_color_number(color_in: &str) -> (&str, i32) {
    let trimmed = color_in.trim();
    let space_idx = trimmed.find(' ').unwrap();
    let number = trimmed[..space_idx].parse().unwrap();
    //let mut number = color_in.trim().split(' ').nth(0).unwrap().trim().parse::<i32>().unwrap();
    let color: &str = color_in.trim().split(' ').nth(1).unwrap().trim();
    return (color, number)
}

#[derive(Debug)]
pub struct Fewest {
    id: i32,
    set: Set,
}

#[derive(Debug, Clone)]
pub struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Debug, Clone)]
pub struct Game {
    id: i32,
    sets: Vec<Set>,
}

pub fn is_set_possible(set: Set) -> bool {
    if set.red > MAX_RED ||
        set.green > MAX_GREEN ||
        set.blue > MAX_BLUE {
            return false;
    } else {
        return true;
    }
}

pub fn sum_ids(ids: &Vec<i32>) -> i32{
    let mut final_tally: i32 = 0;
    for id in ids {
        final_tally += id;
    }
    return final_tally;
}