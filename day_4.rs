mod advent_of_code;

use std::cmp::Ordering;

use std::collections::HashMap;

use std::io;
use std::io::BufRead;

const CHECKSUM_SIZE: usize = 5;

fn compare_counts(a: &(char, usize), b: &(char, usize))
    -> Ordering
{
    let mut cmp = b.1.cmp(&a.1);
    if cmp == Ordering::Equal
    {
        cmp = a.0.cmp(&b.0);
    }
    return cmp;
}

fn compute_checksum(name: &str)
    -> String
{
    let mut counts: HashMap<char, usize> = HashMap::new();

    for letter in name.chars()
    {
        if letter == '-'
        {
            continue;
        }

        let count = counts.entry(letter).or_insert(0);
        *count += 1;
    }

    let mut sorted_counts: Vec<(char, usize)> = counts.into_iter().collect();
    sorted_counts.sort_by(compare_counts);

    let mut checksum = String::with_capacity(CHECKSUM_SIZE);

    for pair in sorted_counts[..CHECKSUM_SIZE].iter()
    {
        let (letter, _) = *pair;
        checksum.push(letter);
    }

    return checksum;
}

fn rot_n(letter: char, n: i32)
    -> char
{
    let origin = 'a' as u8;
    let letter_value = letter as u8 - origin;
    let rotated_value = ((letter_value as i32 + n) % 26) as u8;
    let new_letter = (rotated_value + origin) as char;
    return new_letter;
}

fn shift(text: &str, n: i32)
    -> String
{
    let mut string = String::with_capacity(text.len());
    for letter in text.chars()
    {
        if letter == '-'
        {
            string.push(' ');
        }
        else
        {
            string.push(rot_n(letter, n));
        }
    }
    return string;
}


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Room<'a>
{
    checksum: &'a str,
    name: &'a str,
    sector_id: i32,
}

impl<'a> Room<'a>
{
    fn decode(text: &'a str)
        -> Room<'a>
    {
        let last_dash = text.rfind('-').unwrap();
        let checksum_outer_size = CHECKSUM_SIZE + 2;

        return Room {
            name: text[.. last_dash].as_ref(),
            sector_id: text[last_dash + 1 .. text.len() - checksum_outer_size].parse::<i32>().unwrap(),
            checksum: text[text.len() - 1 - CHECKSUM_SIZE .. text.len() - 1].as_ref(),
        };
    }
}

#[cfg(test)]
mod test
{
    use compute_checksum;
    use rot_n;
    use Room;
    use shift;

    #[test]
    fn test_decode_room()
    {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]";
        let expected = Room {
            checksum: "abxyz",
            name: "aaaaa-bbb-z-y-x",
            sector_id: 123,
        };
        let actual = Room::decode(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_checksum_1()
    {
        let input = "aaaaa-bbb-z-y-x";
        let expected = "abxyz";
        let actual = compute_checksum(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_checksum_2()
    {
        let input = "a-b-c-d-e-f-g-h";
        let expected = "abcde";
        let actual = compute_checksum(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_checksum_3()
    {
        let input = "not-a-real-room";
        let expected = "oarel";
        let actual = compute_checksum(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_checksum_4()
    {
        let input = "totally-real-room";
        let expected = "loart";
        let actual = compute_checksum(input);
        assert_eq!(expected, actual);
    }

    fn assert_rot_n(input: char, n: i32, expected: char)
    {
        let actual = rot_n(input, n);
        assert_eq!(expected, actual);
    }

    fn assert_rot_26(input: char)
    {
        assert_rot_n(input, 26, input)
    }

    #[test]
    fn test_rot_2_a()
    {
        assert_rot_n('a', 2, 'c');
    }

    #[test]
    fn test_rot_2_q()
    {
        assert_rot_n('q', 2, 's');
    }

    #[test]
    fn test_rot_2_z()
    {
        assert_rot_n('z', 2, 'b');
    }

    #[test]
    fn test_rot_26_a()
    {
        assert_rot_26('a');
    }

    #[test]
    fn test_rot_26_q()
    {
        assert_rot_26('q');
    }

    #[test]
    fn test_rot_26_z()
    {
        assert_rot_26('z');
    }

    #[test]
    fn test_rot_32_a()
    {
        assert_rot_n('a', 32, 'g');
    }

    #[test]
    fn test_rot_32_q()
    {
        assert_rot_n('q', 32, 'w');
    }

    #[test]
    fn test_rot_32_z()
    {
        assert_rot_n('z', 32, 'f');
    }

    #[test]
    fn test_shift()
    {
        let expected = "de f";
        let actual = shift("ab-c", 3);
        assert_eq!(actual, expected);
    }
}

#[cfg(not(test))]
fn main()
{
    let stdin = io::stdin();
    let mut total_sector_id = 0;

    for wrapped_line in stdin.lock().lines()
    {
        let line = wrapped_line.unwrap();
        let room = Room::decode(&line);
        if room.checksum == compute_checksum(room.name)
        {
            total_sector_id += room.sector_id;

            if false
            {
                // part 2
                let name = shift(room.name, room.sector_id);
                if name == "northpole object storage"
                {
                    println!("{}: {}", name, room.sector_id);
                }
            }
        }
    }

    println!("{}", total_sector_id);
}
