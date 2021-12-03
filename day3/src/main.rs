use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

struct Results {
    gamma: Vec<char>,
    epsilon: Vec<char>
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_input() -> Vec<Vec<char>> {
    let mut vec: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                vec.push(ip.chars().collect());
            }
        }
        vec
    } else {
        panic!("Could not read file")
    }
}
fn part1() {
    let bits = read_input();
    let bits_length = bits.len();
    let bit_length = bits[0].len();
    let results: Results = (0..bit_length).fold(Results {gamma: Vec::new(), epsilon: Vec::new()}, |mut acc, y| {
        let (zero, one): (Vec<char>, Vec<char>) = (0..bits_length).map(|x| {
            bits[x][y]
        }).partition(|&c| {
            c == '0'
        });
        if zero.len() > one.len() {
            acc.gamma.push('0');
            acc.epsilon.push('1');
        } else {
            acc.gamma.push('1');
            acc.epsilon.push('0');
        }
        acc
    });
    let gamma_string = String::from_iter(results.gamma);
    let epsilon_string = String::from_iter(results.epsilon);
    let gamma = isize::from_str_radix(&*&gamma_string, 2).unwrap();
    let epsilon = isize::from_str_radix(&*&epsilon_string, 2).unwrap();
    println!("Gamma {}", gamma);
    println!("Epsilon {}", epsilon);
    println!("Gamma * Epsilon {}", gamma * epsilon)

}
fn main() {
    part1();
}
