use std::env::current_dir;
use std::fmt::format;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    let cwd = current_dir().unwrap();
    let input_fp = PathBuf::from_str((cwd.to_str().unwrap().to_owned() + "\\src\\1\\input").as_str());
    
    let raw_input = fs::read_to_string(input_fp.unwrap()).expect("File doesn't exist :'(");

    let sum = raw_input.lines().fold(0, |acc,l| {
        let first_num_location = l.find(char::is_numeric).unwrap();
        let last_num_location = l.rfind(char::is_numeric).unwrap();
        let first_dig = &l.chars().nth(first_num_location).unwrap();
        let second_dig = &l.chars().nth(last_num_location).unwrap();
        let num = format!("{}{}",first_dig,second_dig).parse::<i32>().unwrap();
        acc + num
    });
    println!("{}",sum);
}