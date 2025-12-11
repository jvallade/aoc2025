use nom::IResult;
use nom::Parser;
use nom::character::complete::{char, u64};
use nom::multi::many1;
use nom::multi::separated_list1;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct FoodRange {
    start: u64,
    end: u64,
}

fn range(input: &str) -> IResult<&str, FoodRange> {
    let (remainder, range) = separated_list1(char('-'), u64).parse(input)?;
    Ok((
        remainder,
        FoodRange {
            start: *range
                .get(0)
                .expect("Did not find the first element of the range"),
            end: *range
                .get(1)
                .expect("Did not find the second element of the range"),
        },
    ))
}

fn food_database(input: &str) -> IResult<&str, (Vec<FoodRange>, Vec<u64>)> {
    let (remainder, ranges) = separated_list1(char('\n'), range).parse(input)?;
    // remove all the new lines char
    let (remainder, _) = many1(char('\n')).parse(remainder)?;
    let (remainder, food_ids) = separated_list1(char('\n'), u64).parse(remainder)?;
    Ok((remainder, (ranges, food_ids)))
}

fn part1(ranges: &[FoodRange], food_ids: &[u64]) -> u64 {
    let mut cpt = 0;
    for id in food_ids {
        for range in ranges {
            if *id >= range.start && *id <= range.end {
                cpt += 1;
                break;
            }
        }
    }
    cpt
}
fn part2(ranges: &mut [FoodRange]) -> u64 {
    ranges.sort();
    let mut res = 0;
    let mut iter = ranges.iter_mut().peekable();
    while let Some(range) = iter.next() {
        // dbg!(&range);
        if let Some(next) = iter.peek_mut() {
            // next is inside the current range
            // we change the end of the next so that it checks that no other range is inside of the
            // current
            if next.end <= range.end {
                next.end = range.end;
                res += next.start - range.start;

            // next starts before the end of the current range
            } else if next.start <= range.end {
                res += next.start - range.start;

            // nominal case
            } else {
                res += range.end - range.start + 1;
            }
        } else {
            // last range
            res += range.end - range.start + 1;
        }
        // dbg!(res);
    }
    res
}

pub fn run(input: &str) {
    let (_, (mut ranges, food_ids)) =
        food_database(input).expect("Could not parse the input banks");
    println!("=> part1 : {}", part1(&ranges, &food_ids));
    println!("=> part2 : {}", part2(&mut ranges));
}

#[cfg(test)]
mod tests {
    use super::{food_database, part1, part2};

    #[test]
    fn test() {
        let data = "3-5
10-14
16-20
12-18
9-21

1
5
8
11
17
32";

        let (_, (mut ranges, food_ids)) =
            food_database(data).expect("Could not parse the input banks");
        // dbg!(&ranges);
        // dbg!(&food_ids);

        assert_eq!(part1(&ranges, &food_ids), 3);
        assert_eq!(part2(&mut ranges), 16);
    }
}
