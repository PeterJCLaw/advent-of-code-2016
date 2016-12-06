mod advent_of_code;

use advent_of_code::get_line;
use advent_of_code::get_pieces;
use advent_of_code::parse_single_number;

use std::io;

use std::str::FromStr;

enum Direction {
    North,
    East,
    South,
    West,
}

fn turn_left(current_direction: Direction)
    -> Direction
{
    match current_direction {
        Direction::North => Direction::West,
        Direction::West => Direction::South,
        Direction::South => Direction::East,
        Direction::East => Direction::North,
    }
}

fn turn_right(current_direction: Direction)
    -> Direction
{
    match current_direction {
        Direction::West => Direction::North,
        Direction::South => Direction::West,
        Direction::East => Direction::South,
        Direction::North => Direction::East,
    }
}

pub fn main()
{
    let stdin = io::stdin();
    let pieces = get_pieces(get_line(stdin.lock()));

    let mut delta_east = 0;
    let mut delta_north = 0;

    let mut facing_direction = Direction::North;

    for piece in pieces
    {
        if piece.starts_with('R')
        {
            facing_direction = turn_right(facing_direction);
        }
        else if piece.starts_with('L')
        {
            facing_direction = turn_left(facing_direction);
        }
        else
        {
            panic!("Unexpected input ({:?}) does not start with L or R.");
        }

        let rest = String::from_str(&piece[1..]).unwrap();
        let num = parse_single_number(rest);

        match facing_direction {
            Direction::North => delta_north += num,
            Direction::East => delta_east += num,
            Direction::South => delta_north -= num,
            Direction::West => delta_east -= num,
        }
    }

    let total = delta_north + delta_east;
    println!("{:?} North, {:?} East: {:?} Total", delta_north, delta_east, total);
}
