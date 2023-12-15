

fn hash_char(value: &u32, input: &char) -> u32 {
    (*value + *input as u32) * 17 % 256
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String> {

    let sum = input.split(',')
        .into_iter()
        .fold(0, |acc: u32, each|{
            each.chars()
                .into_iter()
                .fold(0, |acc2, val| {
                    hash_char(&acc2, &val)
                }) + acc
         });
    return Ok(format!("{}",sum));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() -> miette::Result<()> {
        let mut input = 'r';
        assert_eq!(146, hash_char(&0, &input));
        input = 's';
        assert_eq!(163, hash_char(&0, &input));
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("1320", process(input)?);
        Ok(())
    }
}