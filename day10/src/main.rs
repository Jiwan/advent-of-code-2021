use itertools::Itertools;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse(path: &'static str) -> Vec<String> {
    let file = File::open(path).expect("Couldn't open data file");
    let lines = BufReader::new(file).lines();
    lines.flatten().collect()
}

const PAIRS: [(char, char); 4] = [('<', '>'), ('(', ')'), ('[', ']'), ('{', '}')];

const ERROR_SCORE: [(char, i32); 4] = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)];

const COMPLETION_SCORE: [(char, i32); 4] = [(')', 1), (']', 2), ('}', 3), ('>', 4)];

fn get_error_score(c: char) -> i32 {
    ERROR_SCORE.iter().find(|(c2, _)| c == *c2).unwrap().1
}

fn get_completion_score(c: char) -> i32 {
    COMPLETION_SCORE.iter().find(|(c2, _)| c == *c2).unwrap().1
}

enum ChunkParsingResult {
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn parse_chunk(chunk: &str) -> ChunkParsingResult {
    let mut state: Vec<char> = Vec::new();

    for c in chunk.chars() {
        if let Some((_, p2)) = PAIRS.iter().find(|(p1, _)| *p1 == c) {
            state.push(*p2);
        } else if let Some(expected) = state.pop() {
            if c != expected {
                return ChunkParsingResult::Corrupted(c);
            }
        } else {
            return ChunkParsingResult::Corrupted(c);
        }
    }

    ChunkParsingResult::Incomplete(state)
}

fn part1() {
    let chunks = parse("data/part1.txt");

    let result = chunks
        .iter()
        .map(|s| parse_chunk(s))
        .filter_map(|r| match r {
            ChunkParsingResult::Corrupted(c) => Some(get_error_score(c)),
            _ => None,
        })
        .sum::<i32>();

    println!("{}", result);
}

fn part2() {
    let chunks = parse("data/part1.txt");

    let result = chunks
        .iter()
        .map(|s| parse_chunk(s))
        .filter_map(|r| match r {
            ChunkParsingResult::Incomplete(v) => Some(v.iter().rev().fold(0u64, |result, c| {
                (result * 5) + (get_completion_score(*c) as u64)
            })),
            _ => None,
        })
        .sorted()
        .collect::<Vec<_>>();

    println!("{:?}", result[result.len() / 2]);
}

fn main() {
    part1();
    part2();
}
