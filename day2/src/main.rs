use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt;

enum Commands {
    Forward,
    Down,
    Up
}

struct Instruction {
    command: Commands,
    value: i32
}

struct Position {
    x: i32,
    depth: i32
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_input() -> Vec<Instruction> {
    let mut vec: Vec<Instruction> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let split = ip.split(" ").collect::<Vec<&str>>();
                let number = split[1].parse::<i32>().unwrap();
                match split[0] {
                    "forward" => vec.push(Instruction {command: Commands::Forward, value: split[1].parse::<i32>().unwrap()}),
                    "down" => vec.push(Instruction {command: Commands::Down, value: split[1].parse::<i32>().unwrap()}),
                    "up" => vec.push(Instruction {command: Commands::Up, value: split[1].parse::<i32>().unwrap()}),
                    _ => panic!("Strange command")
                }
            }
        }
        vec
    } else {
        panic!("Could not read file")
    }
}

fn part1() {
    let commands = read_input();
    let result = commands.iter().fold(Position {x: 0, depth: 0}, |acc, instruction | {
        match instruction.command {
            Commands::Up => Position {depth: acc.depth - &instruction.value, ..acc},
            Commands::Down => Position {depth: acc.depth + &instruction.value, ..acc},
            Commands::Forward => Position {x: acc.x + &instruction.value, ..acc},
        }
    });

    println!("Current depth: {}, Current horizontal: {}, multiple:  {}", result.depth, result.x, result.depth*result.x)
}

fn main() {
    part1();
    println!("Hello, world!");
}
