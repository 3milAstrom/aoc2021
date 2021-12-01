use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct DepthCounter {
    increases: i32,
    last_depth: i32
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_input() -> Vec<i32> {
    let mut vec = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                vec.push(ip.parse::<i32>().unwrap());
            }
        }
        vec
    } else {
        panic!("Could not read file")
    }
}

fn main() {
    let numbers = read_input();
    // println!("{:?}", numbers);
    let first_depth = numbers[0];

    let result = numbers.iter().skip(1).fold(DepthCounter {increases:0 , last_depth: first_depth}, |acc, &x | {
        if x > acc.last_depth {
            DepthCounter {last_depth: x, increases: acc.increases + 1}
        } else {
            DepthCounter {last_depth: x, ..acc}
        }
    });

    println!("Number of increasing {}", result.increases)
}
