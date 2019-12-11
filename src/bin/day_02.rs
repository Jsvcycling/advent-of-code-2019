use std::fs::File;
use std::io::Read;

fn part1(noun: i32, verb: i32, values: &mut Vec<i32>) -> i32 {
    // Initialization values
    values[1] = noun;
    values[2] = verb;
    
    let mut op_idx = 0;

    loop {
        let src1_pos = values[op_idx + 1] as usize;
        let src2_pos = values[op_idx + 2] as usize;
        let dst_pos = values[op_idx + 3] as usize;
        
        match values[op_idx] {
            1 => {
                // Perform an addition operation
                values[dst_pos] = values[src1_pos] + values[src2_pos];
            },
            2 => {
                // Perform a multiplication operation
                values[dst_pos] = values[src1_pos] * values[src2_pos];
            },
            99 => break,
            _ => panic!()
        };

        op_idx += 4;
    }

    values[0]
}

fn part2(target: i32, values: &Vec<i32>) -> i32 {
    for noun in 1..100 {
        for verb in 1..100 {
            let output = part1(noun, verb, &mut values.clone());

            if output == target {
                return 100 * noun + verb;
            }
        }
    }

    return -1;
}

fn main() {
    let mut file = File::open("./input/input_02.txt").unwrap();
    let mut buffer = String::new();
    
    file.read_to_string(&mut buffer).unwrap();

    let values: Vec<i32> = buffer.split(",")
        .map(|v| v.trim().parse::<i32>().unwrap())
        .collect();

    println!("Part 1 Solution: {}", part1(12, 2, &mut values.clone()));
    println!("Part 2 Solution: {}", part2(19690720, &values));
}
