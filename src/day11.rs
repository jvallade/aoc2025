use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, char},
    multi::separated_list1,
};

fn search_output(start: &str, target: &str, devices: &HashMap<String, &Device>) -> u64 {
    let mut res = 0;
    if let Some(current) = devices.get(start) {
        current.outputs.iter().for_each(|out| {
            if out == target {
                res += 1;
            } else {
                res += search_output(out, target, devices)
            }
        });
    }
    res
}

type Graph = HashMap<String, Device>;

fn iterative_dfs(start: &str, target: &str, devices: &Graph) -> Vec<Vec<String>> {
    let mut reachable: HashMap<String, bool> = HashMap::new();

    fn is_reachable(
        node: &str,
        target: &str,
        devices: &HashMap<String, Device>,
        reachable: &mut HashMap<String, bool>,
    ) -> bool {
        if node == target {
            return true;
        }

        if let Some(&result) = reachable.get(node) {
            return result;
        }

        // Simple DFS to check reachability
        let mut visited = HashSet::new();
        let mut stack = vec![node];

        while let Some(n) = stack.pop() {
            if visited.contains(n) {
                continue;
            }
            visited.insert(n);

            if n == target {
                reachable.insert(node.to_string(), true);
                return true;
            }

            if let Some(device) = devices.get(n) {
                for neighbor in &device.outputs {
                    stack.push(neighbor.as_str());
                }
            }
        }

        reachable.insert(node.to_string(), false);
        false
    }

    let mut all_paths = Vec::new();
    let mut stack: VecDeque<(String, Vec<String>)> = VecDeque::new();

    stack.push_back((start.to_string(), vec![start.to_string()]));

    while let Some((node, path)) = stack.pop_back() {
        if node == target {
            all_paths.push(path);
            continue; // Don't explore further from target
        }

        if let Some(neighbors) = devices.get(&node) {
            for neighbor in neighbors.outputs.iter().rev() {
                // Avoid cycles by checking if neighbor is already in path
                if path.contains(neighbor) {
                    continue;
                }

                if !is_reachable(neighbor, target, devices, &mut reachable) {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(neighbor.clone());
                stack.push_back((neighbor.to_owned(), new_path));
            }
        }
    }

    all_paths
}

fn part1(devices: &[Device]) -> u64 {
    let mut devices_map = HashMap::new();
    devices.iter().for_each(|d| {
        devices_map.insert(d.name.clone(), d);
    });

    search_output("you", "out", &devices_map)
}

fn part2(devices: &[Device]) -> u64 {
    let mut devices_map = HashMap::new();
    devices.iter().for_each(|d| {
        devices_map.insert(d.name.clone(), (*d).clone());
    });

    // I noticed that there is no path from dac to fft
    // so I focused on searching paths going first to
    // fft and then to dac
    println!("searching paths from dac");
    let mut res = iterative_dfs("dac", "out", &devices_map).len();
    println!("searching paths from fft to dac");
    res *= iterative_dfs("fft", "dac", &devices_map).len();
    println!("searching paths from svr to fft");
    res *= iterative_dfs("svr", "fft", &devices_map).len();
    res as u64
}

pub fn run(input: &str) {
    let (_, devices) = devices(input).expect("Could not parse input problems");
    println!("=> part1 : {}", part1(&devices));
    println!("=> part2 : {}", part2(&devices));
}

// === PARSERS ===

fn device(input: &str) -> IResult<&str, Device> {
    let (remainder, (name, _, outputs)) =
        (alpha1, tag(": "), separated_list1(char(' '), alpha1)).parse(input)?;
    Ok((
        remainder,
        Device {
            name: name.to_string(),
            outputs: outputs.iter().map(|&s| s.to_string()).collect(),
        },
    ))
}
fn devices(input: &str) -> IResult<&str, Vec<Device>> {
    separated_list1(char('\n'), device).parse(input)
}

// === Data structures ===

#[derive(Debug, Clone)]
struct Device {
    name: String,
    outputs: Vec<String>,
}

#[cfg(test)]
mod tests {

    use crate::day11::{Device, devices};

    use super::{part1, part2};

    #[test]
    fn example_data() {
        let data = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

        let (_, dvs) = devices(data).expect("Could not parse the input data");
        assert_eq!(dvs.len(), 10);
        assert_eq!(part1(&dvs), 5);

        let data = "svr: aaa bbb
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
        let (_, mut dvs) = devices(data).expect("Could not parse the input data");
        assert_eq!(dvs.len(), 13);
        dvs.push(Device {
            name: "out".to_string(),
            outputs: Vec::new(),
        });
        assert_eq!(part2(&dvs), 2);
    }

    #[test]
    fn parsers() {}
}
