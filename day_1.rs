mod advent_of_code;

use advent_of_code::get_line;
use advent_of_code::get_pieces;
use advent_of_code::parse_single_number;

use std::collections::HashSet;
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

#[derive(PartialEq, Eq, Hash)]
struct Location
{
    north: i32,
    east: i32,
}

pub fn main()
{
    let stdin = io::stdin();
    let pieces = get_pieces(get_line(stdin.lock()));

    let mut delta_east = 0;
    let mut delta_north = 0;

    let mut facing_direction = Direction::North;
    let mut locations = HashSet::new();

    let puzzle_a = false;

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

        //println!("{:?} : {:?}", piece, num);
        if puzzle_a
        {
            // part 1 simple handling:
            match facing_direction {
                Direction::North => delta_north += num,
                Direction::East => delta_east += num,
                Direction::South => delta_north -= num,
                Direction::West => delta_east -= num,
            }
        }
        else
        {
            // part 2 extension:
            for _ in 0..num
            {
                match facing_direction {
                    Direction::North => delta_north += 1,
                    Direction::East => delta_east += 1,
                    Direction::South => delta_north -= 1,
                    Direction::West => delta_east -= 1,
                }

                let location = Location { north: delta_north, east: delta_east };
                if locations.contains(&location)
                {
                    break;
                }
                else
                {
                    locations.insert(location);
                }

                //print_location(delta_north, delta_east);
            }
        }
    }

    print_location(delta_north, delta_east);
}

fn print_location(north: i32, east: i32)
{
    let total = north.abs() + east.abs();
    println!("{:?} North, {:?} East: {:?} Total", north, east, total);
}
