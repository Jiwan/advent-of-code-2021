use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse(path: &'static str) -> (Vec<u32>, u32) {
    let file = File::open(path).expect("Couldn't open data file");
    let mut lines = BufReader::new(file).lines().peekable();

    let bitcount = lines.peek().unwrap().as_ref().unwrap().len() as u32;
    let numbers = lines.map(|l| u32::from_str_radix(&l.unwrap(), 2).unwrap()).collect();

    (numbers, bitcount)
}

fn part1() {
    let (numbers, bitcount) = parse("data/part1.txt");
    
    let mut gamma_rate : u32 = 0;

    for i in 0..bitcount
    {
        let set_bit_count = numbers.iter().filter(|&number| number & (1 << i) != 0).count();
        let set_bit = (set_bit_count  > numbers.len() / 2) as u32;
        gamma_rate = gamma_rate + (set_bit << i);
    }
    
    let epsilon_rate = !gamma_rate & ((1 << bitcount) - 1);

    println!("{}", gamma_rate * epsilon_rate)
}

fn part2() {

}

fn main() {
    part1();
    part2();
}
