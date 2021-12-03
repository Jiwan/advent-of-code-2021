use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse(path: &'static str) -> (Vec<u32>, u32) {
    let file = File::open(path).expect("Couldn't open data file");
    let mut lines = BufReader::new(file).lines().peekable();

    let bitcount = lines.peek().unwrap().as_ref().unwrap().len() as u32;
    let numbers = lines
        .map(|l| u32::from_str_radix(&l.unwrap(), 2).unwrap())
        .collect();

    (numbers, bitcount)
}

enum Comparison
{
    GreaterOrEqual,
    Lower
}

fn most_common_value(numbers: &[u32], bit_pos: u32, comparison: &Comparison) -> u32 {
    let set_bit_count = numbers
        .iter()
        .filter(|&number| number & (1 << bit_pos) != 0)
        .count();

    let result = match comparison {
       Comparison::GreaterOrEqual => set_bit_count * 2 >= numbers.len(),
       Comparison::Lower => set_bit_count * 2 < numbers.len()
    };

    result as u32
}

fn part1() {
    let (numbers, bitcount) = parse("data/part1.txt");

    let mut gamma_rate: u32 = 0;

    for bit_pos in 0..bitcount {
        let mcv = most_common_value(&numbers, bit_pos, &Comparison::GreaterOrEqual);
        gamma_rate += mcv << bit_pos;
    }

    let epsilon_rate = !gamma_rate & ((1 << bitcount) - 1);

    println!("{}", gamma_rate * epsilon_rate)
}

fn prune(numbers: &[u32], bitcount: u32, comparison: Comparison) -> u32 {
    let mut v = numbers.to_vec();

    for bit_pos in (0..bitcount).rev() {
        let mvc = most_common_value(&v, bit_pos, &comparison);
        v.retain(|&number| (number & (1 << bit_pos)) == (mvc << bit_pos));

        if v.len() == 1 {
            return v[0];
        }
    }

    unreachable!()
}

fn part2() {
    let (numbers, bitcount) = parse("data/part1.txt");

    let oxygen_generator_rating = prune(&numbers, bitcount, Comparison::GreaterOrEqual);
    let co2_scrubber_rating = prune(&numbers, bitcount, Comparison::Lower);

    println!("{}, {}, {}", oxygen_generator_rating, co2_scrubber_rating, oxygen_generator_rating * co2_scrubber_rating)
}

fn main() {
    part1();
    part2();
}