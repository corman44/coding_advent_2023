use itertools::Itertools;
use nom::InputIter;


#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String> {

    let empty_idx = input.lines().find_position(|line| line.trim().is_empty()).unwrap();
    println!("{:?}", empty_idx);

    let first_pattern = input.lines()
        .into_iter()
        .collect_vec();



    Ok("1".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#";
        assert_eq!("405", process(input)?);
        Ok(())
    }
}