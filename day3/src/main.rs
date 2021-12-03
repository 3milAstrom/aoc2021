use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

fn rate(values: Vec<Vec<char>>, y:usize) -> (usize, usize) {
    let (zero, one): (Vec<char>, Vec<char>) = (0..values.len()).map(|x| {
        values[x][y]
    }).partition(|&c| {
        c == '0'
    });
    (zero.len(),one.len())
}

fn part1() -> Results {
    let bits = read_input();
    let bit_length = bits[0].len();
    let results: Results = (0..bit_length).fold(Results {gamma: Vec::new(), epsilon: Vec::new()}, |mut acc, y| {
        let (zero,one) = rate(bits.clone(), y);
        if zero > one {
            acc.gamma.push('0');
            acc.epsilon.push('1');
        } else {
            acc.gamma.push('1');
            acc.epsilon.push('0');
        }
        acc
    });
    let gamma_string = String::from_iter(&results.gamma);
    let epsilon_string = String::from_iter(&results.epsilon);
    let gamma = isize::from_str_radix(&*&gamma_string, 2).unwrap();
    let epsilon = isize::from_str_radix(&*&epsilon_string, 2).unwrap();
    println!("Gamma {}", gamma);
    println!("Epsilon {}", epsilon);
    println!("Gamma * Epsilon {}", gamma * epsilon);

    results
}

fn filter(values: Vec<Vec<char>>, zero: usize, one: usize, bigger: char, smaller: char, y: usize) -> Vec<Vec<char>> {
    let tmp = values.clone();
    if zero > one {
        tmp.into_iter().filter(|x| {
            x[y] == bigger
        }).collect::<Vec<Vec<char>>>()
    } else {
        tmp.into_iter().filter(|x| {
        x[y] == smaller
        }).collect::<Vec<Vec<char>>>()
    }
}

fn part2() {
    let bits = read_input();
    let bit_length = bits[0].len();
    let oxygen_vec: Vec<Vec<char>> = (0..bit_length).fold(bits.clone(), |acc, y| {
        if acc.len() == 1 { return acc }
        let (zero,one) = rate(acc.clone(), y);
        filter(acc, zero, one, '0', '1', y)
    });

    let co2_vec: Vec<Vec<char>> = (0..bit_length).fold(bits.clone(), |acc, y| {
        if acc.len() == 1 { return acc }
        let (zero,one) = rate(acc.clone(), y);
        filter(acc, zero, one, '1', '0', y)
    });

    let oxy_string = String::from_iter(&oxygen_vec[0]);
    let co2_string = String::from_iter(&co2_vec[0]);
    let oxygen = isize::from_str_radix(&*&oxy_string, 2).unwrap();
    let co2 = isize::from_str_radix(&*&co2_string, 2).unwrap();

    println!("Oxygen {}, co2 {}, multiple {}", oxygen, co2, oxygen * co2)
}

fn main() {
    part1();
    part2();
}
