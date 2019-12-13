use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(lines: &Vec<String>) -> i32 {
    let mut data: HashMap<(i32, i32), i32> = HashMap::new();

    for (line_idx, line) in lines.iter().enumerate() {
        let mut endpoints: Vec<(i32, i32)> = Vec::new();
        endpoints.push((0, 0));

        let steps: Vec<String> = line.split(",").map(|v| String::from(v.trim())).collect();

        for step in steps {
            let (dir, amt_str) = step.split_at(1);
            let amt = amt_str.parse::<i32>().unwrap();

            let last = endpoints.last().unwrap().clone();

            // Something is likely wrong in here...
            match dir {
                "U" => {
                    let start = last.1 + 1;
                    let end = start + amt;

                    for idx in start..end {
                        let val = data.entry((last.0, idx)).or_insert(0);
                        *val += 1;
                    }

                    endpoints.push((last.0, end));
                },
                "D" => {
                    let start = last.1 - amt - 1;
                    let end = start + amt;

                    for idx in start..end {
                        let val = data.entry((last.0, idx)).or_insert(0);
                        *val += 1;
                    }

                    endpoints.push((last.0, start));
                },
                "L" => {
                    let start = last.0 - amt - 1;
                    let end = start + amt;

                    println!("start = {}, end = {}", start, end);

                    for idx in start..end {
                        let val = data.entry((idx, last.1)).or_insert(0);
                        *val += 1;
                    }

                    endpoints.push((start, last.1));
                },
                "R" => {
                    let start = last.0 + 1;
                    let end = start + amt;

                    for idx in start..=end {
                        let val = data.entry((idx, last.1)).or_insert(0);
                        *val += 1;
                    }

                    endpoints.push((end, last.1));
                }
                _ => panic!(),
            };
        }

        println!("{:?}", endpoints);
    }

    data.iter()
        .filter(|(k, v)| **v > 1)
        .map(|(k, _)| k.0.abs() + k.1.abs())
        .filter(|v| *v > 0)
        .min()
        .unwrap()
        .clone()
}

fn part2(lines: &Vec<String>) -> i32 {
    0
}

fn main() {
    let file = File::open("./input/input_03.txt").unwrap();

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    println!("Part 1 Solution: {}", part1(&lines));
    println!("Part 2 Solution: {}", part2(&lines));
}
