mod advent_of_code;

use advent_of_code::get_numbers;

use std::collections::HashMap;

use std::io;
use std::io::BufRead;

fn compute_checksum(name: str)
    -> str
{
    let mut counts = HashMap::new();

    return "nope";
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Room
{
    checksum: str,
    name: str,
    sector_id: i32,
}

impl Room
{
    fn decode(text: String)
        -> Room
    {
        return Room { checksum: "" };
    }
}

#[cfg(test)]
mod test
{
    use Room;

    #[test]
    fn test_decode_room()
    {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]";
        let expected = Room {
            checksum: "abxyz",
            name: "aaaaa-bbb-z-y-x",
            sector_id: 123,
        };
        let actual = Room.decode(input);
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
}

#[cfg(not(test))]
fn main()
{
    let stdin = io::stdin();
    let mut total_sector_id = 0;

    for line in stdin.lock().lines()
    {
        let room = Room::decode(line.unwrap());
        if room.checksum == compute_checksum(room.name)
        {
            total_sector_id += room.sector_id;
        }
    }

    println!("{}", total_sector_id);
}
