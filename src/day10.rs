use std::{error::Error, fmt::Display};

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, one_of, u64},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::delimited,
};
use rayon::prelude::*;

fn search_shortest_combination(init: &Lights, target: &Lights, buttons: &[Button]) -> u64 {
    let lights_to_toggle = init.diff(target).expect("Could not perform diff on lights");
    if lights_to_toggle.is_empty() {
        return 0;
    }

    if buttons.is_empty() {
        return u64::MAX;
    }

    let possible_starts = buttons
        .iter()
        .filter(|b| b.numbers.contains(&lights_to_toggle[0]))
        .collect::<Vec<_>>();

    let mut min_path = u64::MAX;
    for possible_start in possible_starts {
        let remaining_buttons = buttons
            .iter()
            .filter(|b| **b != *possible_start)
            .cloned()
            .collect::<Vec<_>>();

        let mut current = (*init).clone();
        current.apply_button(possible_start);

        let res = search_shortest_combination(&current, target, &remaining_buttons);
        if res < min_path {
            min_path = res;
        }
    }
    min_path.saturating_add(1)
}

fn part1(machines: &[(Lights, Vec<Button>, Joltage)]) -> u64 {
    machines
        .into_par_iter()
        .map(|(lights, buttons, _)| {
            search_shortest_combination(&Lights::new(lights.status.len()), lights, buttons)
        })
        .sum()
}

fn part2() -> u64 {
    todo!()
}

pub fn run(input: &str) {
    let (_, machines) = machines(input).expect("Could not parse input problems");
    println!("=> part1 : {}", part1(&machines));
    println!("=> part2 : {}", part2());
}

// === PARSERS ===

fn lights(input: &str) -> IResult<&str, Lights> {
    let light = map(one_of(".#"), |c| match c {
        '.' => LightStatus::Off,
        '#' => LightStatus::On,
        _ => unreachable!("one_of ensures only '.' or '#'"),
    });
    let (remainder, lights) = delimited(tag("["), many1(light), tag("]")).parse(input)?;
    Ok((remainder, Lights { status: lights }))
}
fn button(input: &str) -> IResult<&str, Button> {
    let (remainder, button) =
        delimited(tag("("), separated_list1(char(','), u64), tag(")")).parse(input)?;
    Ok((remainder, Button { numbers: button }))
}
fn buttons(input: &str) -> IResult<&str, Vec<Button>> {
    separated_list1(char(' '), button).parse(input)
}
fn joltage(input: &str) -> IResult<&str, Joltage> {
    let (remainder, numbers) =
        delimited(tag("{"), separated_list1(char(','), u64), tag("}")).parse(input)?;
    Ok((remainder, Joltage { numbers }))
}
fn machine(input: &str) -> IResult<&str, (Lights, Vec<Button>, Joltage)> {
    let (remainder, lights) = lights(input)?;
    let (remainder, _) = char(' ').parse(remainder)?;
    let (remainder, buttons) = buttons(remainder)?;
    let (remainder, _) = char(' ').parse(remainder)?;
    let (remainder, joltage) = joltage(remainder)?;
    Ok((remainder, (lights, buttons, joltage)))
}
fn machines(input: &str) -> IResult<&str, Vec<(Lights, Vec<Button>, Joltage)>> {
    separated_list1(char('\n'), machine).parse(input)
}

// === Data structures ===

#[derive(Debug, Clone)]
struct Lights {
    status: Vec<LightStatus>,
}

impl Lights {
    fn new(len: usize) -> Self {
        Lights {
            status: vec![LightStatus::Off; len],
        }
    }
    fn apply_button(&mut self, button: &Button) {
        self.status.iter_mut().enumerate().for_each(|(i, s)| {
            if button.numbers.contains(&(i as u64)) {
                s.toggle()
            }
        });
    }
    fn diff(&self, other: &Self) -> Result<Vec<u64>, LightDiffError> {
        if self.status.len() != other.status.len() {
            Err(LightDiffError)
        } else {
            Ok(self
                .status
                .iter()
                .enumerate()
                .filter(|(i, s)| **s != other.status[*i])
                .map(|(i, _)| i as u64)
                .collect())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum LightStatus {
    On,
    Off,
}

impl LightStatus {
    fn toggle(&mut self) {
        match self {
            LightStatus::On => *self = LightStatus::Off,
            LightStatus::Off => *self = LightStatus::On,
        }
    }
}

#[derive(Debug)]
struct LightDiffError;
impl Error for LightDiffError {}
impl Display for LightDiffError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not perform diff of lights")
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Button {
    numbers: Vec<u64>,
}

#[derive(Debug)]
struct Joltage {
    numbers: Vec<u64>,
}

#[cfg(test)]
mod tests {

    use crate::day10::{button, joltage, lights, machine, machines};

    use super::{part1, part2};

    #[test]
    fn example_data() {
        let data = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let (_, machines) = machines(data).expect("Could not parse the input data");
        assert_eq!(machines.len(), 3);
        assert_eq!(part1(&machines), 7);
        // assert_eq!(part2(), 24);
    }

    #[test]
    fn parsers() {
        lights("[.##.]").expect("could not parse lights");
        button("(3)").expect("could not parse button");
        button("(1,3)").expect("could not parse button");
        joltage("{3,5,4,7}").expect("could not parse joltage");
        machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")
            .expect("could not parse machine");
    }
}
