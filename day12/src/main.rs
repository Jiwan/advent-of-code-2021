use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse(path: &'static str) -> HashMap<String, Vec<String>> {
    let file = File::open(path).expect("Couldn't open data file");
    let lines = BufReader::new(file).lines();

    let mut vertices = lines
        .map(|line| {
            let s = line.unwrap();
            let split = s.split_once("-").unwrap();
            (String::from(split.0), String::from(split.1))
        })
        .into_group_map();
    let mut reverse_lookup = Vec::<(String, String)>::new();

    for (k, values) in &vertices {
        for v in values {
            reverse_lookup.push((v.clone(), k.clone()));
        }
    }

    for (k, v) in reverse_lookup {
        vertices.entry(k).or_default().push(v);
    }

    vertices
}

fn craw_graph_impl<'a>(
    current_edge: &'a str, 
    current_path : &mut Vec<&'a str>, 
    discovered_paths: &mut Vec<Vec<&'a str>>, 
    graph : &'a HashMap<String, Vec<String>>,
    mut disallow_double_visit : bool
) {

    let already_visited = current_edge.chars().all(char::is_lowercase) && current_path.contains(&current_edge);

    if already_visited  {
        if current_edge == "start" || disallow_double_visit {
            return;
        }

        disallow_double_visit = true;
    }

    current_path.push(current_edge);

    if current_edge == "end" {
        discovered_paths.push(current_path.clone());
        current_path.pop();
        return;
    }

    for adjacent_edges in graph.get(current_edge).unwrap() {
        craw_graph_impl(adjacent_edges, current_path, discovered_paths, graph, disallow_double_visit);
    }

    current_path.pop();
}

fn crawl_graph(graph : &HashMap<String, Vec<String>>, disallow_double_visits : bool) -> Vec<Vec<&str>> {
    let mut discovered_paths = vec![];

    let mut current_path = vec![];

    craw_graph_impl("start", &mut current_path, &mut discovered_paths, graph, disallow_double_visits);

    discovered_paths
}

fn part1() {
    let graph = parse("data/part1.txt");
    let discovered_paths = crawl_graph(&graph, true);
    println!("{:?}", discovered_paths.len());
}

fn part2() {
    let graph = parse("data/part1.txt");
    let discovered_paths = crawl_graph(&graph, false);
    println!("{:?}", discovered_paths.len());
}

fn main() {
    part1();
    part2();
}
