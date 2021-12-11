use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct DumboOctopusesMap {
    data: Vec<Vec<i32>>,
}

impl DumboOctopusesMap {
    fn size(&self) -> (usize, usize) {
        (self.data[0].len(), self.data.len())
    }

    #[rustfmt::skip]
    fn adjacent_points(&self, pos: (usize, usize)) -> [Option<(usize, usize)>; 8] {
        [
            if pos.0 > 0 { Some((pos.0 - 1, pos.1)) } else { None },
            if pos.1 > 0 { Some((pos.0, pos.1 - 1)) } else { None },
            if pos.0 < self.size().0 - 1 { Some((pos.0 + 1, pos.1)) } else { None },
            if pos.1 < self.size().1 - 1 { Some((pos.0, pos.1 + 1)) } else { None },
            if pos.0 > 0 && pos.1 > 0 { Some((pos.0 - 1, pos.1 - 1)) } else { None },
            if pos.0 < (self.size().0 - 1) && pos.1 > 0 { Some((pos.0 + 1, pos.1 - 1)) } else { None },
            if pos.0 > 0 && pos.1 < (self.size().1 - 1) { Some((pos.0 - 1, pos.1 + 1)) } else { None },
            if pos.0 < (self.size().0 - 1) && pos.1 < (self.size().1 - 1) { Some((pos.0 + 1, pos.1 + 1)) } else { None },
        ]
    }

    fn get_energy(&mut self, pos: (usize, usize)) -> &mut i32 {
        &mut self.data[pos.1][pos.0]
    }

    fn increase_energy(&mut self, pos: (usize, usize)) {
        let energy = self.get_energy((pos.0, pos.1));

        if *energy > -1 {
            *energy += 1;
        }
    }

    fn execute_round(&mut self) -> u32 {
        let mut amount_of_flash = 0;

        for y in 0..self.size().1 {
            for x in 0..self.size().0 {
                self.increase_energy((x, y));
            }
        }

        loop {
            let mut has_been_updated = false;
            for y in 0..self.size().1 {
                for x in 0..self.size().0 {
                    let energy = self.get_energy((x, y));

                    if *energy > 9 {
                        *energy = -1;
                        has_been_updated = true;

                        self.adjacent_points((x, y))
                            .iter()
                            .flatten()
                            .for_each(|p| self.increase_energy(*p));
                    }
                }
            }

            if !has_been_updated {
                break;
            }
        }

        for y in 0..self.size().1 {
            for x in 0..self.size().0 {
                let energy = self.get_energy((x, y));

                if *energy == -1 {
                    *energy = 0;
                    amount_of_flash += 1;
                }
            }
        }

        amount_of_flash
    }

    fn execute_rounds(&mut self, n: u32) -> u32 {
        let mut amount_of_flash = 0;

        for _ in 0..n {
            amount_of_flash += self.execute_round();
        }

        amount_of_flash
    }
}

fn parse(path: &'static str) -> DumboOctopusesMap {
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
    DumboOctopusesMap { data }
}

fn part1() {
    let mut octopuses_map = parse("data/part1.txt");
    let result = octopuses_map.execute_rounds(100);
    println!("{}", result);
}

fn part2() {
    let mut octopuses_map = parse("data/part1.txt");
    let mut synchronized_round = 1;

    loop {
        if octopuses_map.execute_round() == (octopuses_map.size().0 * octopuses_map.size().1) as u32
        {
            println!("{}", synchronized_round);
            break;
        } else {
            synchronized_round += 1;
        }
    }
}

fn main() {
    part1();
    part2();
}
