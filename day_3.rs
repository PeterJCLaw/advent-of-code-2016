mod advent_of_code;

use advent_of_code::get_numbers;

use std::io;
use std::io::BufRead;

fn main()
{
    let stdin = io::stdin();
    let mut n_possible = 0;
    for line in stdin.lock().lines()
    {
        let mut numbers = get_numbers(line.unwrap());
        numbers.sort();
        if numbers[0] >= 0 && numbers[0] + numbers[1] > numbers[2]
        {
            n_possible += 1;
        }
    }
    println!("{}", n_possible);
}
