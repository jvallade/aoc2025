use std::collections::VecDeque;

use nom::IResult;
use nom::Parser;
use nom::character::complete::one_of;
use nom::character::complete::{multispace1, space1, u64};
use nom::combinator::map;
use nom::multi::separated_list1;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

fn problems(input: &str) -> IResult<&str, Vec<Problem>> {
    let mut number_lines = Vec::new();
    let mut remainder = input;
    while let Ok((new_remainder, numbers)) = separated_list1(space1::<_, ()>, u64).parse(remainder)
    {
        remainder = new_remainder;
        let (new_remainder, _) = multispace1::<_, ()>.parse(remainder).expect("");
        remainder = new_remainder;
        number_lines.push(numbers);
    }

    let (new_remainder, signs) = separated_list1(
        space1,
        map(one_of("+*"), |c| match c {
            '+' => Some(Operation::Add),
            '*' => Some(Operation::Multiply),
            _ => None,
        }),
    )
    .parse(remainder)?;

    let mut problems = Vec::new();
    for i in 0..signs.len() {
        let op = signs.get(i).unwrap().unwrap();
        let mut problem = Problem {
            numbers: Vec::new(),
            operation: op,
        };
        for line in &number_lines {
            let num = line.get(i).unwrap();
            problem.numbers.push(*num);
        }
        problems.push(problem)
    }

    Ok((new_remainder, problems))
}

fn problems2(input: &str) -> IResult<&str, Vec<Problem>> {
    let mut number_lines = Vec::new();
    let mut sign_line = None;
    for l in input.lines() {
        if l.contains('+') {
            sign_line = Some(l);
            break;
        }
        number_lines.push(l);
    }
    let sign_line = sign_line.expect("did not find the sign line");
    let input_width = sign_line.len();

    // parse sign line
    let (_, signs) = separated_list1(
        space1,
        map(one_of("+*"), |c| match c {
            '+' => Some(Operation::Add),
            '*' => Some(Operation::Multiply),
            _ => None,
        }),
    )
    .parse(sign_line)?;

    // parse numbers and create the problems
    let mut signs = VecDeque::from(signs);
    let mut problems = Vec::new();
    let mut numbers = Vec::new();
    for i in 0..input_width {
        let mut num = String::new();
        for line in &number_lines {
            num.push(line.chars().nth(i).unwrap());
        }
        if let Ok(n) = num.trim().parse::<u64>() {
            numbers.push(n);
        } else {
            problems.push(Problem {
                numbers: numbers.clone(),
                operation: signs.pop_front().unwrap().unwrap(),
            });
            numbers.clear();
        }
    }
    problems.push(Problem {
        numbers: numbers,
        operation: signs.pop_front().unwrap().unwrap(),
    });

    Ok(("", problems))
}

fn part1(problems: &[Problem]) -> u64 {
    let mut res = 0;
    for problem in problems {
        res += match problem.operation {
            Operation::Add => problem.numbers.iter().sum::<u64>(),
            Operation::Multiply => problem.numbers.iter().product::<u64>(),
        }
    }
    res
}

fn part2(problems: &[Problem]) -> u64 {
    let mut res = 0;
    for problem in problems {
        res += match problem.operation {
            Operation::Add => problem.numbers.iter().sum::<u64>(),
            Operation::Multiply => problem.numbers.iter().product::<u64>(),
        }
    }
    res
}

pub fn run(input: &str) {
    let (_, problems) = problems(input).expect("Could not parse input problems");
    println!("=> part1 : {}", part1(&problems));
    let (_, problems) = problems2(input).expect("Could not parse input problems");
    println!("=> part2 : {}", part2(&problems));
}

#[cfg(test)]
mod tests {

    use super::{part1, part2, problems, problems2};

    #[test]
    fn test() {
        let data = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        let (_, problems) = problems(data).expect("Could not parse the input data");
        assert_eq!(part1(&problems), 4277556);

        let (_, problems) = problems2(data).expect("Could not parse the input data");
        dbg!(&problems);
        assert_eq!(part2(&problems), 3263827);
        // dbg!(&ranges);
        // dbg!(&food_ids);

        // assert_eq!(part1(&ranges, &food_ids), 3);
        // assert_eq!(part2(&mut ranges), 16);
    }
}
