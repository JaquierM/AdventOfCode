use std::{collections::HashMap, fs::read_to_string, vec};

fn main() {
    let content = read_to_string("./day11/files/input").expect("Invalid file");
    let devices_v2 = parse_v2(&content);

    let part1 = find_path_v2("you", "out", &vec![], &devices_v2);
    println!("Sum for part 1 is {part1}");

    let part2 = find_path_v2("svr", "out", &vec!["fft", "dac"], &devices_v2);
    println!("Sum for part 2 is {part2}");
}

fn parse_v2(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();

    input
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let (begin, others) = line.split_once(":").unwrap();
            nodes.insert(begin, others.split_whitespace().collect());
        });

    nodes
}

fn find_path_v2(from: &str, to: &str, via: &[&str], devices: &HashMap<&str, Vec<&str>>) -> usize {
    walk_dfs(
        from,
        to,
        via,
        devices,
        &vec![false; via.len()],
        &mut HashMap::new(),
    )
}

fn walk_dfs<'a>(
    from: &str,
    to: &str,
    via: &[&str],
    devices: &HashMap<&str, Vec<&'a str>>,
    pass: &[bool],
    mem: &mut HashMap<(&'a str, Vec<bool>), usize>,
) -> usize {
    if from == to {
        return match pass.iter().all(|x| *x) {
            true => 1,
            false => 0,
        };
    }

    devices
        .get(from)
        .unwrap()
        .iter()
        .map(|next| {
            let new_pass: Vec<bool> = pass
                .iter()
                .zip(via)
                .map(|(passed, wanted)| *passed || wanted == next)
                .collect();

            if let Some(existing) = mem.get(&(*next, new_pass.clone())) {
                return *existing;
            }

            let count = walk_dfs(next, to, via, devices, &new_pass, mem);

            mem.insert((next, new_pass), count);

            count
        })
        .sum()
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
        let devices = parse_v2(TEST_DATA_1);
        let count = find_path_v2("you", "out", &vec![], &devices);
        assert_eq!(count, 5);
    }

    #[test]
    fn test_example_part_2() {
        let devices = parse_v2(TEST_DATA_2);
        let count = find_path_v2("svr", "out", &vec!["fft", "dac"], &devices);
        assert_eq!(count, 2);
    }
}
