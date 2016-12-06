use std::io::BufRead;
use std::iter::FromIterator;
use std::str::FromStr;

pub fn get_line<T>(mut input: T)
    -> String
    where T: BufRead
{
    let mut buffer = String::new();
    input.read_line(&mut buffer).unwrap();
    buffer
}

pub fn parse_single_number(input: String)
    -> i32
{
    input.trim().parse::<i32>().unwrap()
}

#[allow(dead_code)]
pub fn get_single_number<T>(input: T)
    -> i32
    where T: BufRead
{
    parse_single_number(get_line(input))
}

#[allow(dead_code)]
pub fn get_numbers(line: String)
    -> Vec<i32>
{
    Vec::from_iter(
        line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
    )
}

#[allow(dead_code)]
pub fn get_pieces(line: String)
    -> Vec<String>
{
    Vec::from_iter(
        line.split_whitespace()
            .map(|x| x.trim_right_matches(','))
            .map(|s| String::from_str(s).unwrap())
    )
}
