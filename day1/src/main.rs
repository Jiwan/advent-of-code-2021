use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn parse(path : &'static str) -> Vec<i32>
{
    let file = File::open(path).expect("Couldn't open data file");
    let reader = BufReader::new(file);
    
    reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap()).collect()
}

fn part1() 
{
    let data = parse("data/part1.txt");
    let increase_count = data.windows(2).map(|w| w[0] < w[1]).filter(|v| *v).count();
    println!("{}", increase_count);
}

fn part2()
{
    let data = parse("data/part1.txt");
    let increase_count = data.windows(4).map(|w| w.first() < w.last()).filter(|v| *v).count();
    println!("{}", increase_count);
}

fn main() {
    part1();
    part2();
}

