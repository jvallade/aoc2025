use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::character::complete::{char, u64 as parse_u64};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::pair;

#[derive(Debug)]
enum RotationDirection {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    direction: RotationDirection,
    steps: u64,
}

fn rotation(input: &str) -> IResult<&str, Rotation> {
    map(pair(direction, parse_u64), |(direction, steps)| Rotation {
        direction,
        steps,
    })
    .parse(input)
}

fn direction(input: &str) -> IResult<&str, RotationDirection> {
    alt((
        map(char('L'), |_| RotationDirection::Left),
        map(char('R'), |_| RotationDirection::Right),
    ))
    .parse(input)
}

fn rotations(input: &str) -> IResult<&str, Vec<Rotation>> {
    separated_list0(char('\n'), rotation).parse(input)
}

fn compute_code(rotations: &[Rotation]) -> u32 {
    let mut position: i64 = 50;
    let mut code = 0;

    rotations.iter().for_each(|r| {
        match r.direction {
            RotationDirection::Left => position -= r.steps as i64,
            RotationDirection::Right => position += r.steps as i64,
        }
        while position < 0 {
            position += 100
        }
        while position > 99 {
            position -= 100
        }
        if position == 0 {
            code += 1
        }
    });
    code
}

fn compute_code_2(rotations: &[Rotation]) -> u32 {
    let mut position: i64 = 50;
    let mut code = 0;

    rotations.iter().for_each(|r| {
        let mut steps = r.steps as i64;
        while steps > 99 {
            code += 1;
            steps -= 100;
        }

        let started_from_zero = position == 0;
        match r.direction {
            RotationDirection::Left => position -= steps,
            RotationDirection::Right => position += steps,
        }
        if position > 99 {
            code += 1;
            position -= 100;
        } else if position < 0 {
            if !started_from_zero {
                code += 1;
            }
            position += 100;
        } else if position == 0 {
            code += 1;
        }
    });
    code
}

pub fn run(input: &str) {
    let (_, rotations) = rotations(input).expect("Could not parse the input rotations");
    println!("=> part1 : {}", compute_code(&rotations));
    println!("=> part2 : {}", compute_code_2(&rotations));
}

#[cfg(test)]
mod tests {

    use super::{compute_code, compute_code_2, rotations};

    #[test]
    fn test() {
        let data = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        let (_, rots) = rotations(data).expect("Could not parse the input rotations");
        assert_eq!(compute_code(&rots), 3);
        assert_eq!(compute_code_2(&rots), 6);

        let data = "R1000\nL1000";
        let (_, rots) = rotations(data).expect("Could not parse the input rotations");
        assert_eq!(compute_code_2(&rots), 20);
    }
}
