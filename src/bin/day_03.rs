use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Step<'a> {
    pub direction: &'a str,
    pub distance: i32,
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut data: HashMap<(i32, i32), [bool; 2]> = HashMap::new();

    for (line_idx, line) in lines.iter().enumerate() {
        let mut endpoints: Vec<(i32, i32)> = Vec::new();
        endpoints.push((0, 0));

        let steps: Vec<Step> = line
            .split(",")
            .map(|v| {
                let (dir, amt) = v.trim().split_at(1);
                Step {
                    direction: dir,
                    distance: amt.parse().unwrap(),
                }
            })
            .collect();

        for step in steps {
            let last = endpoints.last().unwrap().clone();

            match step.direction {
                "U" => {
                    let start = last.1 + 1;
                    let end = last.1 + step.distance;

                    for idx in start..=end {
                        let val = data.entry((last.0, idx)).or_insert([false; 2]);
                        val[line_idx] = true;
                    }

                    endpoints.push((last.0, end));
                }
                "D" => {
                    let start = last.1 - step.distance;
                    let end = last.1 - 1;

                    for idx in start..=end {
                        let val = data.entry((last.0, idx)).or_insert([false; 2]);
                        val[line_idx] = true;
                    }

                    endpoints.push((last.0, start));
                }
                "L" => {
                    let start = last.0 - step.distance;
                    let end = last.0 - 1;

                    for idx in start..=end {
                        let val = data.entry((idx, last.1)).or_insert([false; 2]);
                        val[line_idx] = true;
                    }

                    endpoints.push((start, last.1));
                }
                "R" => {
                    let start = last.0 + 1;
                    let end = last.0 + step.distance;

                    for idx in start..=end {
                        let val = data.entry((idx, last.1)).or_insert([false; 2]);
                        val[line_idx] = true;
                    }

                    endpoints.push((end, last.1));
                }
                _ => panic!(),
            };
        }
    }

    let mut dists = data
        .iter()
        .filter(|(_, v)| v[0] && v[1])
        .map(|(k, _)| k.0.abs() + k.1.abs())
        .collect::<Vec<i32>>();

    dists.sort();

    dists.first().unwrap().clone()
}

fn part2(lines: &Vec<String>) -> i32 {
    // We want to store both whether each line has visited a point as well as
    // how long it took to get there the first time.
    let mut data: HashMap<(i32, i32), [(bool, i32); 2]> = HashMap::new();

    for (line_idx, line) in lines.iter().enumerate() {
        let mut i = 1;
        let mut endpoints: Vec<(i32, i32)> = Vec::new();
        endpoints.push((0, 0));

        let steps: Vec<Step> = line
            .split(",")
            .map(|v| {
                let (dir, amt) = v.trim().split_at(1);
                Step {
                    direction: dir,
                    distance: amt.parse().unwrap(),
                }
            })
            .collect();

        for step in steps {
            let last = endpoints.last().unwrap().clone();

            match step.direction {
                "U" => {
                    let start = last.1 + 1;
                    let end = last.1 + step.distance;

                    for idx in start..=end {
                        let val = data
                            .entry((last.0, idx))
                            .or_insert([(false, std::i32::MAX); 2]);

                        val[line_idx].0 = true;

                        if val[line_idx].1 == std::i32::MAX {
                            val[line_idx].1 = i;
                        }

                        i += 1;
                    }

                    endpoints.push((last.0, end));
                }
                "D" => {
                    let start = last.1 - step.distance;
                    let end = last.1 - 1;

                    for idx in start..=end {
                        let val = data
                            .entry((last.0, idx))
                            .or_insert([(false, std::i32::MAX); 2]);

                        val[line_idx].0 = true;

                        if val[line_idx].1 == std::i32::MAX {
                            val[line_idx].1 = i;
                        }

                        i += 1;
                    }

                    endpoints.push((last.0, start));
                }
                "L" => {
                    let start = last.0 - step.distance;
                    let end = last.0 - 1;

                    for idx in start..=end {
                        let val = data
                            .entry((idx, last.1))
                            .or_insert([(false, std::i32::MAX); 2]);

                        val[line_idx].0 = true;

                        if val[line_idx].1 == std::i32::MAX {
                            val[line_idx].1 = i;
                        }

                        i += 1;
                    }

                    endpoints.push((start, last.1));
                }
                "R" => {
                    let start = last.0 + 1;
                    let end = last.0 + step.distance;

                    for idx in start..=end {
                        let val = data
                            .entry((idx, last.1))
                            .or_insert([(false, std::i32::MAX); 2]);

                        val[line_idx].0 = true;

                        if val[line_idx].1 == std::i32::MAX {
                            val[line_idx].1 = i;
                        }

                        i += 1;
                    }

                    endpoints.push((end, last.1));
                }
                _ => panic!(),
            };
        }
    }

    let mut dists = data
        .iter()
        .filter(|(_, v)| v[0].0 && v[1].0)
        .map(|(_, v)| v[0].1 + v[1].1)
        .collect::<Vec<i32>>();

    dists.sort();

    dists.first().unwrap().clone()
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
