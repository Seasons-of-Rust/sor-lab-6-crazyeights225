use std::env;
use std::fmt;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub enum ExitCodes {
    Success,
    GracefulExit,
    MemOutOfBounds,
    OpOutBounds,
    InvalidOp,
}

impl fmt::Display for ExitCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExitCodes::Success => write!(f, "Success"),
            ExitCodes::GracefulExit => write!(f, "Graceful Exit"),
            ExitCodes::MemOutOfBounds => write!(f, "Memory Read Out of Bounds"),
            ExitCodes::OpOutBounds => write!(f, "Opcode Out of Bounds"),
            ExitCodes::InvalidOp => write!(f, "Operation Not Implemented"),
        }
    }
}

pub struct GravityAssistParser {
    pub ip: usize,
    pub instructions: Vec<i32>,
}

impl GravityAssistParser {
    pub fn update_mem(&mut self, z: usize, v: i32) -> ExitCodes {
        if z > self.instructions.len() {
            return ExitCodes::MemOutOfBounds;
        }
        self.instructions[z] = v;
        ExitCodes::Success
    }
}

pub fn compute(gap: &mut GravityAssistParser, op: i32, x: i32, y: i32, z: usize) -> ExitCodes {
    let a = (&gap.instructions).get(x as usize);
    let b = (&gap.instructions).get(y as usize);
    match (op, a, b) {
        (_, None, _) => ExitCodes::MemOutOfBounds,
        (_, _, None) => ExitCodes::MemOutOfBounds,
        (1, Some(m), Some(n)) => {
            let s = (*m) + (*n);
            gap.update_mem(z, s)
        }
        (2, Some(m), Some(n)) => {
            let p = (*m) * (*n);
            gap.update_mem(z, p)
        }
        _ => ExitCodes::InvalidOp,
    }
}

pub fn step(gap: &mut GravityAssistParser) -> ExitCodes {
    let x1 = gap.instructions.get(gap.ip);
    let x2 = gap.instructions.get(gap.ip + 1);
    let x3 = gap.instructions.get(gap.ip + 2);
    let x4 = gap.instructions.get(gap.ip + 3);
    gap.ip += 4;
    match (x1, x2, x3, x4) {
        (Some(99), _, _, _) => ExitCodes::GracefulExit,
        (Some(o), Some(x), Some(y), Some(z)) => {
            let z0: usize = *z as usize;
            let o0: i32 = *o;
            let x0: i32 = *x;
            let y0: i32 = *y;
            compute(gap, o0, x0, y0, z0)
        }
        _ => ExitCodes::OpOutBounds,
    }
}

pub fn run(gap: &mut GravityAssistParser) -> ExitCodes {
    loop {
        let ec = step(gap);
        match ec {
            ExitCodes::Success => (),
            _ => return ec,
        }
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
        .next() // Only take the first line
        .unwrap() // Unwrap the option of whether there was a next line
        .unwrap() // Unwrap the string result of the lines
        .split(',') // Split by commas
        .map(|number| {
            number
                .parse() // Parse the number
                .expect("could not parse number") // Panic with a message if it fails
        })
        .collect();

    let mut ga_parser = GravityAssistParser {
        ip: 0,
        instructions: numbers,
    };
    ga_parser.update_mem(1, 12);
    ga_parser.update_mem(2, 2);

    let ec = run(&mut ga_parser);
    match ec {
        ExitCodes::GracefulExit => {
            let x = ga_parser.instructions.get(0).unwrap();
            println!("Result: {}", x);
        }
        _ => {
            println!("[!] ERROR: {}", ec);
        }
    }
}
