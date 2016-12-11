mod advent_of_code;

use advent_of_code::get_numbers;

use std::io;
use std::io::BufRead;

fn is_valid(mut numbers: Vec<i32>)
    -> bool
{
    if numbers.len() != 3
    {
        panic!("Non-triangle vector passed: {:?}.", numbers);
    }
    numbers.sort();
    return numbers[0] >= 0 && numbers[0] + numbers[1] > numbers[2];
}

fn main()
{
    let stdin = io::stdin();
    let mut n_possible = 0;

    let mut rows = Vec::new();

    for line in stdin.lock().lines()
    {
        rows.push(get_numbers(line.unwrap()));
    }

    let mut maybe_triangles: Vec<Vec<i32>>;
    if false
    {
        // part 1 -- a row is assumed to be a triangle
        maybe_triangles = rows;
    }
    else
    {
        // part 2 -- triangles go "downwards"
        maybe_triangles = Vec::with_capacity(rows.len());
        for _ in 0..rows.len()
        {
            maybe_triangles.push(Vec::with_capacity(3));
        }
        for r in 0..(rows.len() / 3)
        {
            for r_ in 0..3
            {
                let ref row = rows[r * 3 + r_];
                for c in 0..3
                {
                    maybe_triangles[r + c].push(row[c]);
                }
            }
        }
    }

    for numbers in maybe_triangles
    {
        //println!("{:?}", numbers);
        if is_valid(numbers)
        {
            n_possible += 1;
        }
    }

    println!("{}", n_possible);
}
