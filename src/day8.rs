use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

use nom::IResult;
use nom::Parser;
use nom::character::complete::{char, i64};
use nom::multi::separated_list1;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Box {
    x: i64,
    y: i64,
    z: i64,
}

impl Box {
    fn distance(&self, other: &Self) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}

#[derive(Debug, PartialEq)]
struct Distance {
    boxes: (Box, Box),
    distance: f64,
}
impl Eq for Distance {}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(cmp) = self.distance.partial_cmp(&other.distance) {
            cmp
        } else {
            self.boxes.0.cmp(&other.boxes.0)
        }
    }
}

fn normalize(b1: Box, b2: Box) -> (Box, Box) {
    if b1 <= b2 { (b1, b2) } else { (b2, b1) }
}

fn position(input: &str) -> IResult<&str, Box> {
    let (remainder, coords) = separated_list1(char(','), i64).parse(input)?;
    Ok((
        remainder,
        Box {
            x: *coords.first().expect("X not found"),
            y: *coords.get(1).expect("Y not found"),
            z: *coords.get(2).expect("Z not found"),
        },
    ))
}
fn positions(input: &str) -> IResult<&str, Vec<Box>> {
    separated_list1(char('\n'), position).parse(input)
}

fn distances(boxes: &[Box]) -> BinaryHeap<Reverse<Distance>> {
    // Compute all the distances
    let mut distances = BinaryHeap::new();
    let mut seen: HashSet<(Box, Box)> = HashSet::new();
    boxes.iter().for_each(|b1| {
        boxes.iter().for_each(|b2| {
            if b1 != b2 {
                let pair = normalize(b1.clone(), b2.clone());
                if seen.insert(pair.clone()) {
                    let distance = Distance {
                        boxes: pair,
                        distance: b1.distance(b2),
                    };
                    distances.push(Reverse(distance));
                }
            }
        });
    });
    distances
}

fn part1(iteration: u64, boxes: &[Box]) -> u64 {
    let mut distances = distances(boxes);

    // Create the circuits
    let mut circuits: Vec<HashSet<Box>> = Vec::new();
    for _ in 0..iteration {
        if let Some(d) = distances.pop() {
            let b1 = d.0.boxes.0;
            let b2 = d.0.boxes.1;
            let mut possible = circuits
                .iter_mut()
                .filter(|c| c.contains(&b1) || c.contains(&b2))
                .collect::<Vec<_>>();
            match possible.len() {
                1 => {
                    possible[0].insert(b1);
                    possible[0].insert(b2);
                }
                2 => {
                    let second = possible.pop().unwrap();
                    possible[0].extend(second.iter().cloned());
                    second.clear();
                }
                0 => {
                    let mut circuit = HashSet::new();
                    circuit.insert(b1);
                    circuit.insert(b2);
                    circuits.push(circuit);
                }
                _ => {
                    println!("Unexpected possible length");
                    dbg!(&possible);
                }
            };
        }
    }
    circuits.sort_by_key(|c1| c1.len());
    let mut res = 1;
    for _ in 0..3 {
        res *= circuits.pop().unwrap().len();
    }
    res as u64
}

fn part2(boxes: &[Box]) -> i64 {
    let mut distances = distances(boxes);

    let mut circuits: Vec<HashSet<Box>> = Vec::new();
    loop {
        if let Some(d) = distances.pop() {
            let b1 = d.0.boxes.0.clone();
            let b2 = d.0.boxes.1.clone();
            let mut possible = circuits
                .iter_mut()
                .filter(|c| c.contains(&b1) || c.contains(&b2))
                .collect::<Vec<_>>();
            match possible.len() {
                1 => {
                    possible[0].insert(b1);
                    possible[0].insert(b2);
                }
                2 => {
                    let second = possible.pop().unwrap();
                    possible[0].extend(second.iter().cloned());
                    second.clear();
                }
                0 => {
                    let mut circuit = HashSet::new();
                    circuit.insert(b1);
                    circuit.insert(b2);
                    circuits.push(circuit);
                }
                _ => {
                    println!("Unexpected possible length");
                    dbg!(&possible);
                }
            };
            if circuits.iter().any(|c| c.len() == boxes.len()) {
                return d.0.boxes.0.x * d.0.boxes.1.x;
            }
        }
    }
}

pub fn run(input: &str) {
    let (_, boxes) = positions(input).expect("Could not parse input problems");
    println!("=> part1 : {}", part1(1000, &boxes));
    println!("=> part2 : {}", part2(&boxes));
}

#[cfg(test)]
mod tests {

    use super::{part1, part2, positions};

    #[test]
    fn test() {
        let data = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        let (_, boxes) = positions(data).expect("Could not parse the input data");
        assert_eq!(part1(10, &boxes), 40);
        assert_eq!(part2(&boxes), 25272);
    }
}
