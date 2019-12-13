use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(lines: &Vec<String>) -> i32 {
    let mut data: HashMap<(i32, i32), [bool; 2]> = HashMap::new();

    for (line_idx, line) in lines.iter().enumerate() {
        let mut endpoints: Vec<(i32, i32)> = Vec::new();
        endpoints.push((0, 0));

        let steps: Vec<String> = line.split(",").map(|v| String::from(v.trim())).collect();

        for step in steps {
            let (dir, amt_str) = step.split_at(1);
            let amt: i32 = amt_str.parse().unwrap();

            let last = endpoints.last().unwrap().clone();

            match dir {
                "U" => {
                    let start = last.1 + 1;
                    let end = last.1 + amt;

                    for idx in start..=end {
                        let val = data.entry((last.0, idx)).or_insert([false; 2]);
                        val[line_idx] = true;
                    }

                    endpoints.push((last.0, end));
                },
                "D" => {
                    let start = last.1 - amt;
                    let end = last.1 - 1;

                    for idx in start..=end {
                        let val = data.entry((last.0, idx)).or_insert([false; 2]);
                        val[line_idx] = true;
                    }

                    endpoints.push((last.0, start));
                },
                "L" => {
                    let start = last.0 - amt;
                    let end = last.0 - 1;

                    for idx in start..=end {
                        let val = data.entry((idx, last.1)).or_insert([false; 2]);
                        val[line_idx] = true;
                    }

                    endpoints.push((start, last.1));
                },
                "R" => {
                    let start = last.0 + 1;
                    let end = last.0 + amt;

                    for idx in start..=end {
                        let val = data.entry((idx, last.1)).or_insert([false; 2]);
                        val[line_idx] = true;
                    }

                    endpoints.push((end, last.1));
                },
                _ => panic!(),
            };
        }
    }

    let mut dists = data.iter()
        .filter(|(_, v)| v[0] && v[1])
        .map(|(k, _)| k.0.abs() + k.1.abs())
        .collect::<Vec<i32>>();

    dists.sort();

    println!("{:?}", dists);

    dists.first().unwrap().clone()
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
