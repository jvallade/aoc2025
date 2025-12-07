use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::character::complete::{char, one_of};
use nom::combinator::map;
use nom::multi::many1;
use nom::multi::separated_list1;

#[derive(Debug)]
struct Cell {
    occupied: bool,
}

fn cell(input: &str) -> IResult<&str, Cell> {
    alt((
        map(char('.'), |_| Cell { occupied: false }),
        map(char('@'), |_| Cell { occupied: true }),
    ))
    .parse(input)
}

fn cell_row(input: &str) -> IResult<&str, Vec<Cell>> {
    many1(cell).parse(input)
}

fn grid(input: &str) -> IResult<&str, Vec<Vec<Cell>>> {
    separated_list1(char('\n'), cell_row).parse(input)
}

fn part1(cells: &Vec<Vec<Cell>>) -> u64 {
    let mut res = 0;
    for i in 0..cells.len() {
        for j in 0..cells[0].len() {
            if !cells[i][j].occupied {
                // no need to check
                continue;
            }

            let mut count = 0;
            let positions_to_check = vec![
                (i.wrapping_sub(1), j.wrapping_sub(1)),
                (i.wrapping_sub(1), j),
                (i.wrapping_sub(1), j.wrapping_add(1)),
                (i, j.wrapping_sub(1)),
                (i, j.wrapping_add(1)),
                (i.wrapping_add(1), j.wrapping_sub(1)),
                (i.wrapping_add(1), j),
                (i.wrapping_add(1), j.wrapping_add(1)),
            ];
            for (x, y) in positions_to_check {
                if let Some(c) = cells.get(x) {
                    if let Some(cc) = c.get(y) {
                        // dbg!(cc);
                        if cc.occupied {
                            count += 1;
                        }
                    }
                }
            }
            if count < 4 {
                res += 1;
            }
        }
    }
    res
}
fn cells_to_be_cleared(cells: &Vec<Vec<Cell>>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for i in 0..cells.len() {
        for j in 0..cells[0].len() {
            if !cells[i][j].occupied {
                // no need to check
                continue;
            }

            let mut count = 0;
            let positions_to_check = vec![
                (i.wrapping_sub(1), j.wrapping_sub(1)),
                (i.wrapping_sub(1), j),
                (i.wrapping_sub(1), j.wrapping_add(1)),
                (i, j.wrapping_sub(1)),
                (i, j.wrapping_add(1)),
                (i.wrapping_add(1), j.wrapping_sub(1)),
                (i.wrapping_add(1), j),
                (i.wrapping_add(1), j.wrapping_add(1)),
            ];
            for (x, y) in positions_to_check {
                if let Some(c) = cells.get(x) {
                    if let Some(cc) = c.get(y) {
                        if cc.occupied {
                            count += 1;
                        }
                    }
                }
            }
            if count < 4 {
                res.push((i, j));
            }
        }
    }
    res
}
fn part2(cells: &mut Vec<Vec<Cell>>) -> usize {
    let mut res = 0;

    while true {
        let accessible_cells = cells_to_be_cleared(cells);
        if accessible_cells.len() == 0 {
            break;
        }
        res += accessible_cells.len();
        for (x, y) in accessible_cells {
            let c = cells.get_mut(x).expect("error").get_mut(y).expect("error");
            c.occupied = false;
        }
    }

    res
}

pub fn run(input: &str) {
    let (_, mut cells) = grid(input).expect("Could not parse the input banks");
    println!("=> part1 : {}", part1(&cells));
    println!("=> part2 : {}", part2(&mut cells));
}

#[cfg(test)]
mod tests {
    use super::{grid, part1, part2};

    #[test]
    fn test() {
        let data = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let (_, mut cells) = grid(data).expect("Could not parse the input banks");

        assert_eq!(part1(&cells), 13);
        assert_eq!(part2(&mut cells), 43);
    }
}
