use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Heightmap {
    data: Vec<Vec<i32>>,
}

impl Heightmap {
    fn size(&self) -> (usize, usize) {
        (self.data[0].len(), self.data.len())
    }

    #[rustfmt::skip]
    fn adjacent_points(&self, pos: (usize, usize)) -> [Option<(usize, usize)>; 4] {
        [
            if pos.0 > 0 { Some((pos.0 - 1, pos.1)) } else { None },
            if pos.1 > 0 { Some((pos.0, pos.1 - 1)) } else { None },
            if pos.0 < self.size().0 - 1 { Some((pos.0 + 1, pos.1)) } else { None },
            if pos.1 < self.size().1 - 1 { Some((pos.0, pos.1 + 1)) } else { None },
        ]
    }

    fn get_height(&self, pos: (usize, usize)) -> i32 {
        self.data[pos.1][pos.0]
    }

    fn find_low_points(&self) -> Vec<(usize, usize)> {
        self.data
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(move |(x, value)| {
                        self.adjacent_points((*x, y))
                            .iter()
                            .all(|point| point.map_or(true, |p| **value < self.get_height(p)))
                    })
                    .map(move |(x, _)| (x, y))
            })
            .flatten()
            .collect()
    }
}

fn parse(path: &'static str) -> Heightmap {
    let file = File::open(path).expect("Couldn't open data file");
    let lines = BufReader::new(file).lines();

    let data = lines
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    Heightmap { data }
}

fn part1() {
    let heightmap = parse("data/part1.txt");

    let risk_level: i32 = heightmap
        .find_low_points()
        .iter()
        .map(|p| heightmap.get_height(*p) + 1)
        .sum();
    println!("{}", risk_level);
}

#[derive(Default)]
struct BasinCrawler {
    visited_point: HashSet<(usize, usize)>,
}

impl BasinCrawler {
    fn new() -> Self {
        Default::default()
    }

    fn crawl(&mut self, pos: (usize, usize), heightmap: &Heightmap) -> usize {
        if self.visited_point.contains(&pos) {
            return 0;
        }

        self.visited_point.insert(pos);

        if heightmap.get_height(pos) >= 9 {
            return 0;
        }

        let mut size = 1;
        for p in heightmap.adjacent_points(pos).iter().flatten() {
            size += self.crawl(*p, heightmap);
        }

        size
    }
}

fn part2() {
    let heightmap = parse("data/part1.txt");

    let basin: usize = heightmap
        .find_low_points()
        .iter()
        .map(|p| {
            let mut crawler = BasinCrawler::new();
            crawler.crawl(*p, &heightmap)
        })
        .sorted()
        .rev()
        .take(3)
        .product();

    println!("{}", basin);
}

fn main() {
    part1();
    part2();
}
