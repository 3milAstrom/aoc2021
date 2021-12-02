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

struct SimplePosition {
    x: i32,
    depth: i32
}

struct Position {
    x: i32,
    depth: i32,
    aim: i32,
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
                    "forward" => vec.push(Instruction {command: Commands::Forward, value: number}),
                    "down" => vec.push(Instruction {command: Commands::Down, value: number}),
                    "up" => vec.push(Instruction {command: Commands::Up, value: -1 * number}),
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
    let result = commands.iter().fold(SimplePosition {x: 0, depth: 0}, |acc, instruction | {
        match instruction.command {
            Commands::Down | Commands::Up => SimplePosition {depth: acc.depth + &instruction.value, ..acc},
            Commands::Forward => SimplePosition {x: acc.x + &instruction.value, ..acc},
        }
    });

    println!("Current depth: {}, Current horizontal: {}, multiple:  {}", result.depth, result.x, result.depth*result.x)
}

fn part2() {
    let commands = read_input();
    let result = commands.iter().fold(Position {x: 0, depth: 0, aim: 0}, |acc, instruction | {
        match instruction.command {
            Commands::Down | Commands::Up => Position {aim: acc.aim + &instruction.value, ..acc},
            Commands::Forward => {
                let new_depth =  acc.depth + (&instruction.value * acc.aim);
                Position {x: acc.x + &instruction.value, depth: new_depth,  ..acc}
            }
        }
    });

    println!("Current depth: {}, Current horizontal: {}, multiple:  {}", result.depth, result.x, result.depth*result.x)
}

fn main() {
    part1(); //2120749
    part2(); //2138382217
}
