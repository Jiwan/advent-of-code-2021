use std::convert::TryInto;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashSet;
use std::collections::HashMap;
use itertools::Itertools;

const DIGITS_SEGMENTS_COUNT : [usize; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
const UNIQUE_DIGITS : [usize; 4] = [1, 4, 7, 8];

#[derive(Debug)]
struct Entry {
    signals : [String; 10],
    output : [String; 4]
}

fn parse(path: &'static str) -> Vec<Entry> {
    let file = File::open(path).expect("Couldn't open data file");
    let lines = BufReader::new(file).lines();
    
    lines.map(
        |line| {
            let line = line.unwrap();
            let (signals, output) = line.split_once("|").unwrap();
            let signals = signals.trim();
            let output = output.trim();
            
            Entry {
                signals : signals.split(" ").map(String::from).collect::<Vec<_>>().try_into().unwrap(),
                output : output.split(" ").map(String::from).collect::<Vec<_>>().try_into().unwrap()
            }
        }
    ).collect()
}

fn part1() {
    let entries = parse("data/part1.txt");
    let unique_digits_segments_count = UNIQUE_DIGITS.iter().map(|x| DIGITS_SEGMENTS_COUNT[*x]).collect::<Vec<_>>();
    let test = entries.iter().map(|entry| &entry.output).flatten().filter(|x| unique_digits_segments_count.contains(&x.len())).count();

    println!("{:?}", test);
}

fn solve_entry(entry : &Entry) -> i32 {
    let signals = &entry.signals;
    let number_one_signal = signals.iter().find(|signal| signal.len() == 2).unwrap().chars().collect::<HashSet<_>>();
    let number_seven_signal = signals.iter().find(|signal| signal.len() == 3).unwrap().chars().collect::<HashSet<_>>();
    let number_four_signal = signals.iter().find(|signal| signal.len() == 4).unwrap().chars().collect::<HashSet<_>>();
    let number_eight_signal = signals.iter().find(|signal| signal.len() == 7).unwrap().chars().collect::<HashSet<_>>();

    let signals_with_five_segments = signals.iter().filter(|signal| signal.len() == 5).collect::<Vec<_>>();
    let signals_with_six_segments = signals.iter().filter(|signal| signal.len() == 6).collect::<Vec<_>>();

    let a_segment = *number_seven_signal.difference(&number_one_signal).next().unwrap();
    let b_segment = *number_four_signal.difference(&number_one_signal).find(|c| signals_with_five_segments.iter().filter(|s| s.contains(**c)).count() == 1).unwrap();
    let mut all_segments_of_four_that_are_not_d = number_one_signal.clone();
    all_segments_of_four_that_are_not_d.insert(b_segment);
    let d_segment = *number_four_signal.difference(&all_segments_of_four_that_are_not_d).next().unwrap();
    
    let c_segment = *number_one_signal.iter().find(|c| signals_with_six_segments.iter().filter(|s| s.contains(**c)).count() == 2).unwrap();
    let f_segment = *number_one_signal.iter().find(|c| **c != c_segment).unwrap();

    let a_b_c_d_f = HashSet::from([a_segment, b_segment, c_segment, d_segment, f_segment]);

    let number_five_signal = signals_with_five_segments.iter().find(|s| !s.contains(c_segment)).unwrap().chars().collect::<HashSet<_>>();

    let g_segment = *number_five_signal.difference(&a_b_c_d_f).next().unwrap();

    let mut a_b_c_d_f_g = a_b_c_d_f;
    a_b_c_d_f_g.insert(g_segment);

    let e_segment = *number_eight_signal.difference(&a_b_c_d_f_g).next().unwrap();

    let number_map = HashMap::<Vec<char>, i32>::from([
        ([a_segment, b_segment, c_segment, e_segment, f_segment, g_segment].iter().sorted().cloned().collect(), 0),
        (number_one_signal.iter().sorted().cloned().collect(), 1),
        ([a_segment, c_segment, d_segment, e_segment, g_segment].iter().sorted().cloned().collect(), 2),
        ([a_segment, c_segment, d_segment, f_segment, g_segment].iter().sorted().cloned().collect(), 3),
        (number_four_signal.iter().sorted().cloned().collect(), 4),
        (number_five_signal.iter().sorted().cloned().collect(), 5),
        ([a_segment, b_segment, d_segment, e_segment, f_segment, g_segment].iter().sorted().cloned().collect(), 6),
        (number_seven_signal.iter().sorted().cloned().collect(), 7),
        (number_eight_signal.iter().sorted().cloned().collect(), 8),
        ([a_segment, b_segment, c_segment, d_segment, f_segment, g_segment].iter().sorted().cloned().collect(), 9),
    ]);

    let mut total = 0;
    for (i, signal) in entry.output.iter().rev().enumerate() {
        let number : Vec<char> = signal.chars().sorted().collect();
        total += 10i32.pow(i as u32) * number_map.get(&number).unwrap();
    }

    total
}

fn part2() {
    let entries = parse("data/part1.txt");
    let result = entries.iter().map(solve_entry).sum::<i32>();
    println!("{}", result);
}

fn main() {
    part1();
    part2();
}
