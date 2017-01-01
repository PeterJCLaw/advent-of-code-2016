mod advent_of_code;

use std::cmp::{max,min};
use std::io;
use std::io::BufRead;

// assume square
#[cfg(not(test))]
const KEYPAD_SIZE: i32 = 5;

#[cfg(test)]
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
        -> Option<char>
    {
        if self.row < 0 || self.row >= KEYPAD_SIZE ||
        self.column < 0 || self.column >= KEYPAD_SIZE
        {
            return None;
        }

        let pad: [[Option<char>; 5]; 5];
        if KEYPAD_SIZE == 3
        {
            // part 1
            pad = [
                [Some('1'), Some('2'), Some('3'), None,      None],
                [Some('4'), Some('5'), Some('6'), None,      None],
                [Some('7'), Some('8'), Some('9'), None,      None],
                [None,      None,      None,      None,      None],
                [None,      None,      None,      None,      None],
            ];
        }
        else if KEYPAD_SIZE == 5
        {
            // part 2
            pad = [
                [None,      None,      Some('1'), None,      None],
                [None,      Some('2'), Some('3'), Some('4'), None],
                [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
                [None,      Some('A'), Some('B'), Some('C'), None],
                [None,      None,      Some('D'), None,      None],
            ];
        }
        else
        {
            panic!("Unexpected KEYPAD_SIZE: {:?}.", KEYPAD_SIZE);
        }
        return pad[self.row as usize][self.column as usize];
    }

    fn is_valid(&self)
        -> bool
    {
        return match self.get_key() {
            Some(_) => true,
            None => false,
        }
    }

    fn or_if_valid(&self, pos: KeyPadPosition)
        -> KeyPadPosition
    {
        if pos.is_valid()
        {
            return pos;
        }
        return *self;
    }

    fn moved_up(&self)
        -> KeyPadPosition
    {
        return self.or_if_valid(KeyPadPosition {
            row: Self::bounded(self.row - 1),
            column: self.column,
        });
    }

    fn moved_down(&self)
        -> KeyPadPosition
    {
        return self.or_if_valid(KeyPadPosition {
            row: Self::bounded(self.row + 1),
            column: self.column,
        });
    }

    fn moved_right(&self)
        -> KeyPadPosition
    {
        return self.or_if_valid(KeyPadPosition {
            row: self.row,
            column: Self::bounded(self.column + 1),
        });
    }

    fn moved_left(&self)
        -> KeyPadPosition
    {
        return self.or_if_valid(KeyPadPosition {
            row: self.row,
            column: Self::bounded(self.column - 1),
        });
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
    fn test_get_key_invalid()
    {
        let pos = KeyPadPosition { row: -1, column: 0 };
        let actual = pos.get_key();
        assert_eq!(actual, None);
    }

    #[test]
    fn test_get_key_1()
    {
        let pos = KeyPadPosition { row: 0, column: 0 };
        let actual = pos.get_key();
        assert_eq!(actual, Some('1'));
    }

    #[test]
    fn test_get_key_3()
    {
        let pos = KeyPadPosition { row: 0, column: 2 };
        let actual = pos.get_key();
        assert_eq!(actual, Some('3'));
    }

    #[test]
    fn test_get_key_4()
    {
        let pos = KeyPadPosition { row: 1, column: 0 };
        let actual = pos.get_key();
        assert_eq!(actual, Some('4'));
    }
}

#[cfg(not(test))]
fn main()
{
    let stdin = io::stdin();
    // part 1
    // let mut pad_position = KeyPadPosition { row: 1, column: 1 };
    // part 2
    let mut pad_position = KeyPadPosition { row: 2, column: 0 };

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
        match key {
            Some(k) => print!("{}", k),
            None => panic!("Managed to create an invalid position: {:?}.", pad_position),
        }
    }
    println!("");
}
