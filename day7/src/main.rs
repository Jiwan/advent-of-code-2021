use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse(path: &'static str) -> Vec<usize> {
    let file = File::open(path).expect("Couldn't open data file");
    let line = BufReader::new(file).lines().next();
}

fn part1() {
    let fishes  = parse("data/sample.txt");
}

fn part2() {
}

fn main() {
    part1();
    part2();
}
