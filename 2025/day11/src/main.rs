use itertools::Itertools;
use petgraph::algo::all_simple_paths;
use petgraph::stable_graph::NodeIndex;
use petgraph::Graph;
use rayon::prelude::*;
use std::hash::RandomState;
use std::time::Instant;
use std::{collections::HashMap, fs::read_to_string, vec};

fn main() {
    let content = read_to_string("./day11/files/input").expect("Invalid file");
    let devices = parse(&content);

    let part1 = find_path("you", "out", &vec![], &devices);
    println!("Sum for part 1 is {part1}");

    let part2 = find_path("svr", "out", &vec!["fft", "dac"], &devices);
    println!("Sum for part 2 is {part2}");
}

fn parse(input: &str) -> (Graph<&str, ()>, HashMap<&str, NodeIndex>) {
    let mut graph = Graph::new();
    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();

    input
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let (begin, others) = line.split_once(":").unwrap();

            let source: NodeIndex = if let Some(node) = nodes.get(begin) {
                *node
            } else {
                let dest = graph.add_node(begin);
                nodes.insert(begin, dest);
                dest
            };

            others.split_whitespace().for_each(|destination| {
                let dest: NodeIndex = if let Some(node) = nodes.get(destination) {
                    *node
                } else {
                    let dest = graph.add_node(destination);
                    nodes.insert(destination, dest);
                    dest
                };

                graph.add_edge(source, dest, ());
            })
        });

    (graph, nodes)
}

fn find_path(
    from: &str,
    to: &str,
    via: &[&str],
    devices: &(Graph<&str, ()>, HashMap<&str, NodeIndex>),
) -> usize {
    let (graph, nodes) = devices;

    let required_step = {
        let mut required_step = vec![from];
        for step in via {
            required_step.push(step);
        }
        required_step.push(to);
        required_step
    };

    required_step
        .iter()
        .tuple_windows()
        .par_bridge()
        .map(|(from, to)| {
            println!("from {from} to {to}");
            let start = Instant::now();
            let count = all_simple_paths::<Vec<_>, _, RandomState>(
                &graph,
                *nodes.get(from).unwrap(),
                *nodes.get(to).unwrap(),
                0,
                None,
            )
            .par_bridge()
            .count();
            let end = Instant::now() - start;
            println!("{end:?} to count {count} from {from} to {to}");
            count
        })
        .product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA_1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    static TEST_DATA_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_example_part_1() {
        let devices = parse(TEST_DATA_1);

        let count = find_path("you", "out", &vec![], &devices);

        assert_eq!(count, 5);
    }

    #[test]
    fn test_example_part_2() {
        let devices = parse(TEST_DATA_2);

        let count = find_path("svr", "out", &vec!["fft", "dac"], &devices);

        assert_eq!(count, 2);
    }
}
