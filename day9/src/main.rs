use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Heightmap {
    data: Vec<Vec<i32>>,
}

impl Heightmap {
    fn size(self: &Self) -> (usize, usize) {
        (self.data[0].len(), self.data.len())
    }

    fn adjacent_points(self: &Self, pos: (usize, usize)) -> [Option<(usize, usize)>; 4] {
        [
            if pos.0 > 0 { Some((pos.0 - 1, pos.1)) } else { None },
            if pos.1 > 0 { Some((pos.0, pos.1 - 1)) } else { None },
            if pos.0 < self.size().0 - 1 { Some((pos.0 + 1, pos.1)) } else { None },
            if pos.1 < self.size().1 - 1 { Some((pos.0, pos.1 + 1)) } else { None },
        ]
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
        .data
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, value)| {
                    heightmap.adjacent_points((*x, y)).iter().all(|point| {
                        if let Some((x2, y2)) = *point {
                            **value < heightmap.data[y2][x2]
                        } else {
                            true
                        }
                    })
                })
                .map(|(_, value)| *value + 1)
                .sum::<i32>()
        })
        .sum();

    println!("{}", risk_level);
}

fn part2() {
    let entries = parse("data/part1.txt");
}

fn main() {
    part1();
    part2();
}
