use std::env::current_dir;
use std::fmt::format;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let cwd = current_dir().unwrap();
    let input_fp = PathBuf::from_str((cwd.to_str().unwrap().to_owned() + "\\src\\5\\test").as_str());
    
    let raw_input = fs::read_to_string(input_fp.unwrap()).expect("File doesn't exist :'(");

    let mut input_lines = raw_input.lines();
    let first_line = &input_lines.next().unwrap();
    let seeds_str = first_line.split(':').nth(1).unwrap().trim().split(' ');
    let seeds: Vec<i64> = seeds_str.into_iter()
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    let _= input_lines.next(); //skip next empty line
    println!(" printing line: {}",input_lines.nth(0).unwrap());

    let mut all_maps: Vec<Mapping> = vec![];
    let mut temp_maps: Vec<DestinationMap> = vec![];
    // parse input into maps
    for current_line in input_lines.clone() {
        if current_line.contains("map") {
            // parse source & dest names
            let temp_source_name = current_line.split('-').nth(0).unwrap();
            let temp_dest_name = current_line.split('-').nth(2).unwrap().trim().split(' ').nth(0).unwrap();
            
            // parse each source and dest mapping
            let mut temp_mapping: Mapping = Mapping { source_name: temp_source_name.to_string(), dest_name: temp_dest_name.to_string(), maps: vec![]};
            // println!("Mapping: {:?}", temp_mapping);

            input_lines.next();
            // println!("temp_lines: {:?}", input_lines);
            for each in input_lines.by_ref() {
                // let mut curr_line = &input_lines.next().unwrap();
                if each.is_empty() {
                    // println!("Breaking on: {}", each);
                    break;
                }
                if each.contains(char::is_numeric) {
                    let parsed_nums: Vec<i64> = each
                        .trim()
                        .split(' ')
                        .map(|x| {
                            x.trim()
                                .parse::<i64>()
                                .unwrap()})
                        .collect();
                    temp_mapping.maps.push( DestinationMap { 
                        source_start: *parsed_nums.iter().nth(1).unwrap(),
                        dest_start: *parsed_nums.iter().nth(0).unwrap(),
                        len: *parsed_nums.iter().nth(2).unwrap() });
                }
            }
            println!("pushing mapping: {:?}", temp_mapping);
            all_maps.push(temp_mapping);
        }
    }
    // println!("Map: {:?}",all_maps);

    for seed in seeds {
        let _ = all_maps.clone().iter().fold(seed, |acc: i64, m| {
            println!("{} -> {} ", m.source_name, m.dest_name);
            let temp = map_seed_to_destination(acc, &m.maps);
            temp
        });
        // println!("Seed: {}, final location: {}", seed, location);
    }

}

/*
Expecting Output from test:
Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
*/

#[derive(Debug, Clone)]
pub struct Mapping {
    source_name: String,
    dest_name: String,
    maps: Vec<DestinationMap>,
}

// DestinationMap define
#[derive(Debug, Clone)]
pub struct DestinationMap {
    source_start: i64,
    dest_start: i64,
    len: i64,
}

pub fn map_seed_to_destination(seed: i64, dest_map: &Vec<DestinationMap>) -> i64 {
    let mut map_val: Option<i64> = None;

    for map in dest_map {
        if seed >= map.source_start && seed < map.source_start + map.len - 1 {
            map_val = Some(map.dest_start + (seed - map.source_start));
        }
    }

    if map_val == None {
        println!("Map is none: {}", seed);
        return seed
    } else {
        println!("Map existed: {} -> {}", seed, map_val.unwrap());
        return map_val.unwrap()
    }
}