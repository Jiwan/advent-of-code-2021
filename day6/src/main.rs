use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse(path: &'static str) -> Vec<usize> {
    let file = File::open(path).expect("Couldn't open data file");
    let line = BufReader::new(file).lines().next();
    line.unwrap().unwrap().split(",").map(|s| s.parse::<usize>().unwrap()).collect()
}

fn simulate_fishes(fishes : &[usize], days : i32) -> usize
{
    let mut age_groups = vec![0usize; 9];

    for fish in fishes {
        age_groups[*fish] += 1;
    }

    for _ in 0..days {
        let respawning_fish_count = age_groups[0];

        for i in 0..8 {
            age_groups[i] = age_groups[i + 1];
        }

        age_groups[6] += respawning_fish_count;
        age_groups[8] = respawning_fish_count; 
    }

    age_groups.iter().sum()
}

fn part1() {
    let fishes  = parse("data/part1.txt");
    println!("{}", simulate_fishes(&fishes, 80));
}

fn part2() {
    let fishes  = parse("data/part1.txt");
    println!("{}", simulate_fishes(&fishes, 256));
}

fn main() {
    part1();
    part2();
}
