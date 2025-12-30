use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, one_of, u64, usize},
    multi::{many0, many1, separated_list1},
};

fn part1(shapes: &[Shape], trees: &[Tree]) -> u64 {
    // Count the shapes that fits below the tree
    let trees_that_fits: Vec<_> = trees
        .iter()
        .filter(|t| {
            let area = t.size.0 * t.size.1;
            let req_area: u64 = t
                .gifts
                .iter()
                .enumerate()
                .map(|(j, g)| {
                    shapes[j]
                        .chars
                        .iter()
                        .flatten()
                        .filter(|&&c| c == '#')
                        .count() as u64
                        * *g
                })
                .sum();
            req_area <= area
        })
        .collect();
    println!(
        "Remaining trees to consider : {} / {}",
        trees_that_fits.len(),
        trees.len()
    );
    trees_that_fits.len() as u64
}

fn part2() -> u64 {
    0
}

pub fn run(input: &str) {
    let (remainder, shapes) = shapes(input).expect("Could not parse the shapes");
    let (remainder, trees) = trees(remainder).expect("Could not parse the trees");
    println!("=> part1 : {}", part1(&shapes, &trees));
    println!("=> part2 : {}", part2());
}

// === PARSERS ===

fn shape(input: &str) -> IResult<&str, Shape> {
    (
        usize,
        tag(":\n"),
        separated_list1(char('\n'), many1(one_of(".#"))),
    )
        .map(|(index, _, chars)| Shape { index, chars })
        .parse(input)
}

fn shapes(input: &str) -> IResult<&str, Vec<Shape>> {
    separated_list1(tag("\n\n"), shape).parse(input)
}

fn tree(input: &str) -> IResult<&str, Tree> {
    (
        many0(char('\n')),
        u64,
        char('x'),
        u64,
        tag(": "),
        separated_list1(char(' '), u64),
    )
        .map(|(_, x, _, y, _, gifts)| Tree {
            size: (x, y),
            gifts,
        })
        .parse(input)
}

fn trees(input: &str) -> IResult<&str, Vec<Tree>> {
    separated_list1(char('\n'), tree).parse(input)
}

// === Data structures ===

#[derive(Debug, Clone)]
struct Shape {
    index: usize,
    chars: Vec<Vec<char>>,
}

struct Tree {
    size: (u64, u64),
    gifts: Vec<u64>,
}

#[cfg(test)]
mod tests {

    use crate::day12::{shapes, trees};

    use super::{part1, part2};

    const DATA: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn example_data() {
        let (remainder, shapes) = shapes(DATA).expect("Could not parse the shapes");
        let (remainder, trees) = trees(remainder).expect("Could not parse the trees");

        assert_eq!(part1(&shapes, &trees), 2);
        // assert_eq!(part2(), 33);
    }

    #[test]
    fn parsers() {
        let (remainder, shapes) = shapes(DATA).expect("Could not parse the shapes");
        let (remainder, trees) = trees(remainder).expect("Could not parse the trees");
        assert_eq!(shapes.len(), 6);
        assert_eq!(trees.len(), 3)
    }
}
