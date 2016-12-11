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

    fn moved_up(&self)
        -> KeyPadPosition
    {
        return KeyPadPosition {
            row: Self::bounded(self.row - 1),
            column: self.column,
        };
    }

    fn moved_down(&self)
        -> KeyPadPosition
    {
        return KeyPadPosition {
            row: Self::bounded(self.row + 1),
            column: self.column,
        };
    }

    fn moved_right(&self)
        -> KeyPadPosition
    {
        return KeyPadPosition {
            row: self.row,
            column: Self::bounded(self.column + 1),
        };
    }

    fn moved_left(&self)
        -> KeyPadPosition
    {
        return KeyPadPosition {
            row: self.row,
            column: Self::bounded(self.column - 1),
        };
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
        let expected = KeyPadPosition { row: 0, column: 1 };
        let actual = expected.moved_up();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bounded_left_from_first_column()
    {
        let expected = KeyPadPosition { row: 1, column: 0 };
        let actual = expected.moved_left();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bounded_down_from_last_row()
    {
        let expected = KeyPadPosition { row: KEYPAD_SIZE - 1, column: 1 };
        let actual = expected.moved_down();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bounded_right_from_last_col()
    {
        let expected = KeyPadPosition { row: 1, column: KEYPAD_SIZE - 1 };
        let actual = expected.moved_right();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bounded_up_from_middle()
    {
        let pos = KeyPadPosition { row: 1, column: 1 };
        let expected = KeyPadPosition { row: 0, column: 1 };
        let actual = pos.moved_up();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bounded_left_from_middle()
    {
        let pos = KeyPadPosition { row: 1, column: 1 };
        let expected = KeyPadPosition { row: 1, column: 0 };
        let actual = pos.moved_left();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bounded_down_from_middle()
    {
        let pos = KeyPadPosition { row: 1, column: 1 };
        let expected = KeyPadPosition { row: 2, column: 1 };
        let actual = pos.moved_down();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_bounded_right_from_middle()
    {
        let pos = KeyPadPosition { row: 1, column: 1 };
        let expected = KeyPadPosition { row: 1, column: 2 };
        let actual = pos.moved_right();
        assert_eq!(actual, expected);
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
                pad_position = pad_position.moved_up();
            }
            else if chr == 'D'
            {
                pad_position = pad_position.moved_down();
            }
            else if chr == 'R'
            {
                pad_position = pad_position.moved_right();
            }
            else if chr == 'L'
            {
                pad_position = pad_position.moved_left();
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
