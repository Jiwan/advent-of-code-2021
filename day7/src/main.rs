use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse(path: &'static str) -> Vec<i32> {
    let file = File::open(path).expect("Couldn't open data file");
    let line = BufReader::new(file).lines().next();
    line.unwrap()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn part1() {
    let mut crabs = parse("data/part1.txt");
    crabs.sort();
    let midpoint = crabs[crabs.len() / 2];
    let cost = crabs.iter().map(|x| (x - midpoint).abs()).sum::<i32>();

    println!("{}", cost);
}

fn compute_cost(slice: &[i32], pos: i32, costs: &mut HashMap<i32, i32>) -> i32 {
    *costs.entry(pos).or_insert(
        slice
            .iter()
            .map(|x| {
                let n = (x - pos).abs();
                (n * (n + 1)) / 2
            })
            .sum(),
    )
}

fn scan(slice: &[i32]) -> i32 {
    let mut costs_map = HashMap::new();

    let mut pos: i32 = slice[slice.len() / 2];
    loop {
        let cost = compute_cost(&slice, pos, &mut costs_map);

        let right_cost = compute_cost(&slice, pos + 1, &mut costs_map);
        let left_cost = compute_cost(&slice, pos - 1, &mut costs_map);

        if cost <= left_cost && cost <= right_cost {
            return cost;
        } else if cost > left_cost {
            pos = pos - 1;
        } else if cost > right_cost {
            pos = pos + 1;
        }
    }
}

fn part2() {
    let mut crabs = parse("data/part1.txt");
    crabs.sort();
    let cost = scan(&crabs);

    println!("{}", cost);
}

fn main() {
    part1();
    part2();
}
