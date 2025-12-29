use std::{error::Error, fmt::Display};

use cached::UnboundCache;
use cached::proc_macro::cached;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, one_of, u64},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::delimited,
};
use rayon::prelude::*;
use z3::{Optimize, SatResult, ast::Int};

#[cached(
    ty = "UnboundCache<String, u64>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{:?}{:?}{:?}", init, target, buttons) }"#
)]
fn search_shortest_lights(init: &Lights, target: &Lights, buttons: &[Button]) -> u64 {
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

        let res = search_shortest_lights(&current, target, &remaining_buttons);
        if res < min_path {
            min_path = res;
        }
    }
    min_path.saturating_add(1)
}

// This works but does not converge quickly enough
fn search_shortest_joltages(init: &Joltage, target: &Joltage, buttons: &[Button]) -> u64 {
    let counter_to_increase = init.diff(target).expect("Could not perform diff");
    if counter_to_increase.is_empty() {
        return 0;
    }

    let possible_buttons: Vec<_> = buttons
        .iter()
        // all the counter increased by this button are to be increased
        .filter(|b| b.numbers.iter().all(|n| counter_to_increase.contains(n)))
        .cloned()
        .collect();

    let mut min_path = u64::MAX;
    for possible_button in possible_buttons {
        let mut current = (*init).clone();
        current.apply_button(&possible_button);

        let res = search_shortest_joltages(&current, target, buttons);
        if res < min_path {
            min_path = res;
        }
    }
    min_path.saturating_add(1)
}

// Z3 solutions
fn solver_part1(target: &Lights, buttons: &[Button]) -> u64 {
    todo!()
}

fn solver_part2(target: &Joltage, buttons: &[Button]) -> u64 {
    // Create X vector = number of press on each button
    let x: Vec<_> = (0..buttons.len())
        .map(|i| Int::new_const(format!("x{i}")))
        .collect();

    // Create a Z3 optimizer
    let opt = Optimize::new();

    // All the solutions must be positive
    x.iter().for_each(|xi| opt.assert(&xi.ge(0)));

    // Implement A.X = B
    for (j, joltage) in target.numbers.iter().enumerate() {
        let sum: Int = buttons
            .iter()
            .enumerate()
            // consider only buttons listing the current joltage
            .filter(|(_, b)| b.numbers.contains(&(j as u64)))
            // mapping to the corresponding button press
            .map(|(i, _)| x[i].clone())
            .sum();
        // implement the constraint that the sum of presses on valid buttons
        // equals the expected joltage
        opt.assert(&sum.eq(*joltage));
    }

    // Define the objective
    let obj: Int = x.iter().sum();
    opt.minimize(&obj);

    // Check if a solution exists
    let mut res = u64::MAX;
    match opt.check(&[]) {
        SatResult::Sat => {
            let model = opt.get_model().unwrap();
            let solution: Vec<u64> = x
                .iter()
                .map(|xi| model.eval(xi, true).unwrap().as_u64().unwrap())
                .collect();
            res = solution.iter().sum();
            // println!("Minimal norm solution: {solution:?}");
            // println!("Norm: {res}");
        }
        SatResult::Unsat => println!("No solution exists."),
        SatResult::Unknown => println!("Z3 could not determine satisfiability."),
    }
    res
}

fn part1(machines: &[(Lights, Vec<Button>, Joltage)]) -> u64 {
    machines
        .into_par_iter()
        .map(|(lights, buttons, _)| {
            search_shortest_lights(&Lights::new(lights.status.len()), lights, buttons)
        })
        .sum()
}

fn part2(machines: &[(Lights, Vec<Button>, Joltage)]) -> u64 {
    machines
        .into_par_iter()
        .map(|(_, buttons, joltages)| solver_part2(joltages, buttons))
        .sum()
}

pub fn run(input: &str) {
    let (_, machines) = machines(input).expect("Could not parse input problems");
    println!("=> part1 : {}", part1(&machines));
    println!("=> part2 : {}", part2(&machines));
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
fn machine(input: &str) -> IResult<&str, Machine> {
    let (remainder, lights) = lights(input)?;
    let (remainder, _) = char(' ').parse(remainder)?;
    let (remainder, buttons) = buttons(remainder)?;
    let (remainder, _) = char(' ').parse(remainder)?;
    let (remainder, joltage) = joltage(remainder)?;
    Ok((remainder, (lights, buttons, joltage)))
}
fn machines(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(char('\n'), machine).parse(input)
}

// === Data structures ===

type Machine = (Lights, Vec<Button>, Joltage);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
    fn diff(&self, other: &Self) -> Result<Vec<u64>, DiffError> {
        if self.status.len() != other.status.len() {
            Err(DiffError)
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
struct DiffError;
impl Error for DiffError {}
impl Display for DiffError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not perform diff of lights")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Button {
    numbers: Vec<u64>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Joltage {
    numbers: Vec<u64>,
}

impl Joltage {
    fn new(len: usize) -> Self {
        Self {
            numbers: vec![0; len],
        }
    }

    fn diff(&self, other: &Self) -> Result<Vec<u64>, DiffError> {
        if self.numbers.len() != other.numbers.len() {
            Err(DiffError)
        } else {
            Ok(self
                .numbers
                .iter()
                .enumerate()
                .filter(|(i, s)| **s != other.numbers[*i])
                .map(|(i, _)| i as u64)
                .collect())
        }
    }

    fn apply_button(&mut self, button: &Button) {
        self.numbers.iter_mut().enumerate().for_each(|(i, s)| {
            if button.numbers.contains(&(i as u64)) {
                *s += 1;
            }
        });
    }
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
        assert_eq!(part2(&machines), 33);
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
