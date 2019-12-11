use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_file(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().collect::<Result<_, _>>().unwrap()
}

fn part1(lines: &Vec<String>) -> f32 {
    lines.iter()
        .map(|v| v.parse::<f32>().unwrap())
        .map(|v| (v / 3.0).floor() - 2.0)
        .fold(0.0, |acc, v| acc + v)
}

fn part2(lines: &Vec<String>) -> f32 {
    lines.iter()
        .map(|v| v.parse::<f32>().unwrap())
        .map(|v| {
            let mut total = 0.0;
            let mut last = v;

            loop {
                last = (last / 3.0).floor() - 2.0;

                if last <= 0.0 {
                    break;
                }

                total += last;
            }

            total
        })
        .fold(0.0, |acc, v| acc + v)
}

fn main() {
    let lines = parse_file("./input/input_01.txt");

    println!("Part 1 Solution: {}", part1(&lines));
    println!("Part 2 Solution: {}", part2(&lines));
}
