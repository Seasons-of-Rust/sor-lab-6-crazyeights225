use std::env;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn compute_fuel(x: i32) -> i32 {
    if x > 0 {
        ((x / 3) as i32) - 2
    } else {
        0
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./day1 <filename>");
    }
    let fname = &args[1];

    let numbers: Vec<i32> = BufReader::new(File::open(fname).expect("file not found"))
        .lines() // Go through each line
        .map(|line| {
            line.expect("could not parse line") // Unwrap the result of the line
                .parse() // Try to parse it to what we expect (i32 from the annotation)
                .expect("could not parse number") // Unwrap the result of the parse
        })
        .collect();

    let sum: i32 = numbers.iter().map(|x| compute_fuel(*x)).sum();
    println!("Sum: {}", sum);
}
