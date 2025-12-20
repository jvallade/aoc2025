use std::{
    collections::{HashMap, HashSet},
    str::Lines,
};

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut current_indexes: HashSet<_> = lines
        .next()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == 'S')
        .map(|(i, _)| i)
        .collect();

    let mut res = 0;
    for line in lines {
        let deflectors: Vec<_> = line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '^')
            .map(|(i, _)| i)
            .collect();
        let mut new_indexes = HashSet::new();
        for index in current_indexes.iter() {
            if deflectors.contains(index) {
                res += 1;
                new_indexes.insert(index - 1);
                new_indexes.insert(index + 1);
            } else {
                new_indexes.insert(*index);
            }
        }
        current_indexes = new_indexes;
    }
    res
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    index: usize,
    line_number: u64,
}

impl Position {
    fn left(&self) -> Self {
        Position {
            index: self.index - 1,
            line_number: self.line_number + 1,
        }
    }

    fn right(&self) -> Self {
        Position {
            index: self.index + 1,
            line_number: self.line_number + 1,
        }
    }

    fn center(&self) -> Self {
        Position {
            index: self.index,
            line_number: self.line_number + 1,
        }
    }
}

fn resolve_subgraph(
    start_position: Position,
    lines: &mut Lines,
    known: &mut HashMap<Position, u64>,
) -> u64 {
    if let Some(res) = known.get(&start_position) {
        return *res;
    }
    let mut res = 0;
    if let Some(line) = lines.next() {
        if line
            .chars()
            .nth(start_position.index)
            .expect("unexpected index")
            == '^'
        {
            res += 1;
            res += resolve_subgraph(start_position.left(), &mut lines.clone(), known);
            res += resolve_subgraph(start_position.right(), &mut lines.clone(), known);
        } else {
            res += resolve_subgraph(start_position.center(), &mut lines.clone(), known);
        }
    }
    known.insert(start_position, res);
    res
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let start_index = lines
        .next()
        .unwrap()
        .find('S')
        .expect("did not find starting point");

    resolve_subgraph(
        Position {
            index: start_index,
            line_number: 0,
        },
        &mut lines,
        &mut HashMap::new(),
    ) + 1
}

pub fn run(input: &str) {
    println!("=> part1 : {}", part1(&input));
    println!("=> part2 : {}", part2(&input));
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};

    #[test]
    fn test() {
        let data = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        assert_eq!(part1(&data), 21);
        assert_eq!(part2(&data), 40);
    }
}
