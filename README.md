# Summer of Rust Lab 6: Advent of Code

This week, we'll learn about traits, generics, and closures. We'll also go back
to take another look at some things about error handling.

In this week's lab, we'll be working on code puzzles from Advent of Code. Read
on to find out more!

## Advent of Code

![](https://cdn.discordapp.com/attachments/968184771102507031/983478110764818492/unknown.png)

Advent of Code is an Advent calendar of programming puzzles that takes place
every year for the first 25 days of December. You can read more about it here:
https://adventofcode.com/2021/about

For this week's lab, we'll be doing questions from the first and second days of
2019. Each day has two parts, and to read the instruction for the second part,
you need to complete the first. Each day also comes with a puzzle input that you
use to get a final result.

Make sure to log into the site, or you won't be able to see your puzzle input!

### 2019-12-01

Here is the link for day 1: https://adventofcode.com/2019/day/1

For both of these problems, we'll be working in this directory. To create a new
Rust project, you can run:

```bash
cargo new <project-name>
```

You'll need to read from a file for this question, so here's some starter code
to help you with that:

```rust
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let numbers: Vec<i32> = BufReader::new(File::open("input.txt").expect("file not found"))
        .lines() // Go through each line
        .map(|line| {
            line
                .expect("could not parse line") // Unwrap the result of the line
                .parse() // Try to parse it to what we expect (i32 from the annotation)
                .expect("could not parse number") // Unwrap the result of the parse
        })
        .collect();
}
```

### 2019-12-02

Here is the link for day 2: https://adventofcode.com/2019/day/2

The input for this day is a bit different, so here are the changes required to
load it:

```rust
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let numbers: Vec<i32> = BufReader::new(File::open("input.txt").expect("file not found"))
        .lines() // Go through each line
        .next() // Only take the first line
        .unwrap() // Unwrap the option of whether there was a next line
        .unwrap() // Unwrap the string result of the lines
        .split(",") // Split by commas
        .map(|number| {
            number
                .parse() // Parse the number
                .expect("could not parse number") // Panic with a message if it fails
        })
        .collect();
}

```

### That's all!

See you next week 🏖️

## License

The Summer of Rust Labs is duel-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
