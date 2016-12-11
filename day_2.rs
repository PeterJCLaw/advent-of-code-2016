mod advent_of_code;

use std::cmp::{max,min};
use std::io;
use std::io::BufRead;

// assume square
const KEYPAD_SIZE: i32 = 3;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct KeyPadPosition
{
    row: i32,
    column: i32,
}

impl KeyPadPosition
{
    fn bounded(number: i32)
        -> i32
    {
        return max(0, min(number, KEYPAD_SIZE - 1));
    }

    fn get_key(&self)
        -> char
    {
        // TODO: can this be global static?
        let pad: [[char; 3]; 3] = [
            ['1', '2', '3'],
            ['4', '5', '6'],
            ['7', '8', '9'],
        ];
        return pad[self.row as usize][self.column as usize];
    }

    fn move_up(&mut self)
    {
        self.row = Self::bounded(self.row - 1);
    }

    fn move_down(&mut self)
    {
        self.row = Self::bounded(self.row + 1);
    }

    fn move_right(&mut self)
    {
        self.column = Self::bounded(self.column + 1);
    }

    fn move_left(&mut self)
    {
        self.column = Self::bounded(self.column - 1);
    }
}

#[cfg(test)]
mod test
{
    use KeyPadPosition;
    use KEYPAD_SIZE;

    #[test]
    fn test_bounded_up_from_first_row()
    {
        let mut pos = KeyPadPosition { row: 0, column: 1 };
        let expected = pos.clone();
        pos.move_up();
        assert_eq!(pos, expected);
    }

    #[test]
    fn test_bounded_left_from_first_column()
    {
        let mut pos = KeyPadPosition { row: 1, column: 0 };
        let expected = pos.clone();
        pos.move_left();
        assert_eq!(pos, expected);
    }

    #[test]
    fn test_bounded_down_from_last_row()
    {
        let mut pos = KeyPadPosition { row: KEYPAD_SIZE - 1, column: 1 };
        let expected = pos.clone();
        pos.move_down();
        assert_eq!(pos, expected);
    }

    #[test]
    fn test_bounded_right_from_last_col()
    {
        let mut pos = KeyPadPosition { row: 1, column: KEYPAD_SIZE - 1 };
        let expected = pos.clone();
        pos.move_right();
        assert_eq!(pos, expected);
    }

    #[test]
    fn test_bounded_up_from_middle()
    {
        let mut pos = KeyPadPosition { row: 1, column: 1 };
        let expected = KeyPadPosition { row: 0, column: 1 };
        pos.move_up();
        assert_eq!(pos, expected);
    }

    #[test]
    fn test_bounded_left_from_middle()
    {
        let mut pos = KeyPadPosition { row: 1, column: 1 };
        let expected = KeyPadPosition { row: 1, column: 0 };
        pos.move_left();
        assert_eq!(pos, expected);
    }

    #[test]
    fn test_bounded_down_from_middle()
    {
        let mut pos = KeyPadPosition { row: 1, column: 1 };
        let expected = KeyPadPosition { row: 2, column: 1 };
        pos.move_down();
        assert_eq!(pos, expected);
    }

    #[test]
    fn test_bounded_right_from_middle()
    {
        let mut pos = KeyPadPosition { row: 1, column: 1 };
        let expected = KeyPadPosition { row: 1, column: 2 };
        pos.move_right();
        assert_eq!(pos, expected);
    }

    #[test]
    fn test_get_key_1()
    {
        let pos = KeyPadPosition { row: 0, column: 0 };
        let actual = pos.get_key();
        assert_eq!(actual, '1');
    }

    #[test]
    fn test_get_key_3()
    {
        let pos = KeyPadPosition { row: 0, column: 2 };
        let actual = pos.get_key();
        assert_eq!(actual, '3');
    }

    #[test]
    fn test_get_key_4()
    {
        let pos = KeyPadPosition { row: 1, column: 0 };
        let actual = pos.get_key();
        assert_eq!(actual, '4');
    }
}

#[cfg(not(test))]
fn main()
{
    let stdin = io::stdin();
    let mut pad_position = KeyPadPosition { row: 1, column: 1 };

    for line in stdin.lock().lines()
    {
        for chr in line.unwrap().chars()
        {
            if chr == 'U'
            {
                pad_position.move_up();
            }
            else if chr == 'D'
            {
                pad_position.move_down();
            }
            else if chr == 'R'
            {
                pad_position.move_right();
            }
            else if chr == 'L'
            {
                pad_position.move_left();
            }
            else
            {
                panic!("Unexpected character '{:?}'.", chr);
            }
        }
        let key = pad_position.get_key();
        print!("{}", key);
    }
    println!("");
}
