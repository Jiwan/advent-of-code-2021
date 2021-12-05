use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Point = (i32, i32);

#[derive(Debug)]
struct Line {
    begin: Point,
    end: Point,
}

fn parse(path: &'static str) -> Vec<Line> {
    let file = File::open(path).expect("Couldn't open data file");
    let lines = BufReader::new(file).lines();

    lines
        .map(|s| {
            let s = s.unwrap();
            let (begin, end) = s.split_once("->").unwrap();

            fn parse_vec2(s: &str) -> Point {
                let (x, y) = s.split_once(",").unwrap();
                (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
            }

            Line {
                begin: parse_vec2(begin.trim()),
                end: parse_vec2(end.trim()),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Heightmap {
    map: HashMap<Point, i32>,
}

impl Heightmap {
    fn from_lines(lines: &[Line], diagonals : bool) -> Self {
        let mut map = HashMap::new();

        for Line{begin, end} in lines {
            if !(diagonals || begin.0 == end.0 || begin.1 == end.1) {
                continue;
            }

            let dir = ((end.0 - begin.0).min(1).max(-1), (end.1 - begin.1).min(1).max(-1));
            let mut curr_pos = *begin; 

            while curr_pos != *end {
                (*map.entry((curr_pos.0, curr_pos.1)).or_default()) += 1;
                curr_pos.0 += dir.0;
                curr_pos.1 += dir.1;
            }

            (*map.entry((end.0, end.1)).or_default()) += 1;
        }

        Self { map }
    }

    fn print(&self, map_size: i32) {
        let m = self.map.clone();

        for y in 0..map_size {
            for x in 0..map_size {
                print!("{}", m.get(&(x, y)).unwrap_or(&0));
            }
            println!()
        }

    }
}

fn part1() {
    let lines = parse("data/part1.txt");
    let heightmap = Heightmap::from_lines(&lines, false);
    //heightmap.print(10);
    println!("{}", heightmap.map.iter().filter(|(_, v)| **v >= 2).count());
}

fn part2() {
    let lines = parse("data/part1.txt");
    let heightmap = Heightmap::from_lines(&lines, true);
    println!("{}", heightmap.map.iter().filter(|(_, v)| **v >= 2).count());
}

fn main() {
    part1();
    part2();
}
