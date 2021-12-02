use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Vec2 = (i32, i32);

fn parse_command(s: &str) -> Vec2 {
    let mut parts = s.split(" ");
    let order = parts.next().unwrap();
    let amount = parts.next().unwrap().parse::<i32>().unwrap();

    match order {
        "forward" => (amount, 0),
        "down" => (0, amount),
        "up" => (0, -amount),
        _ => unreachable!(),
    }
}

fn parse(path: &'static str) -> Vec<Vec2> {
    let file = File::open(path).expect("Couldn't open data file");
    let reader = BufReader::new(file);
    reader.lines().map(|l| parse_command(&l.unwrap())).collect()
}

fn part1() {
    let data = parse("data/part1.txt");
    let (distance, depth) = data
        .iter()
        .fold((0, 0), |(acc_distance, acc_depth), (distance, depth)| {
            (acc_distance + distance, acc_depth + depth)
        });
    println!("{}", distance * depth);
}

fn part2() {
    let data = parse("data/part1.txt");
    let (distance, depth, _) = data.iter().fold(
        (0, 0, 0),
        |(acc_distance, acc_depth, acc_aim), (distance, aim)| {
            (
                acc_distance + distance,
                acc_depth + distance * acc_aim,
                acc_aim + aim,
            )
        },
    );
    println!("{}", distance * depth);
}

fn main() {
    part1();
    part2();
}
