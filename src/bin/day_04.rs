fn part1_test_adjacent(vals: &Vec<i32>) -> bool {
    for i in 0..(vals.len() - 1) {
        if vals[i] == vals[i + 1] {
            return true;
        }
    }

    false
}

fn part2_test_adjacent(vals: &Vec<i32>) -> bool {
    // First we check the edges.
    if ((vals[0] == vals[1]) && (vals[1] != vals[2]))
        || ((vals[4] == vals[5]) && (vals[3] != vals[4]))
    {
        return true;
    }

    // Now have a 2-element sliding window with look ahead and behind.
    for i in 1..(vals.len() - 2) {
        if (vals[i - 1] != vals[i]) && (vals[i] == vals[i + 1]) && (vals[i] != vals[i + 2]) {
            return true;
        }
    }

    // If everything above fails...
    false
}

fn part1_test_decrease(vals: &Vec<i32>) -> bool {
    for i in 0..(vals.len() - 1) {
        if vals[i] > vals[i + 1] {
            return false;
        }
    }

    true
}

fn part1(low: i32, high: i32) -> i32 {
    let mut count = 0;

    for i in low..=high {
        let vals = i
            .to_string()
            .chars()
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if part1_test_adjacent(&vals) && part1_test_decrease(&vals) {
            count += 1;
        }
    }

    count
}

fn part2(low: i32, high: i32) -> i32 {
    let mut count = 0;

    for i in low..=high {
        let vals = i
            .to_string()
            .chars()
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if part2_test_adjacent(&vals) && part1_test_decrease(&vals) {
            count += 1;
        }
    }

    count
}

fn main() {
    let low = 307237;
    let high = 769058;

    println!("Part 1 Solution: {}", part1(low, high));
    println!("Part 2 Solution: {}", part2(low, high));
}
