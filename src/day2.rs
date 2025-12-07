use nom::IResult;
use nom::Parser;
use nom::character::complete::{char, u64 as parse_u64};
use nom::multi::separated_list0;

#[derive(Debug)]
struct IdRange {
    first: u64,
    last: u64,
}

impl IdRange {
    fn get_invalid_ids(&self) -> Vec<u64> {
        let mut res = Vec::new();
        for id in self.first..self.last + 1 {
            if !is_valid(&id) {
                res.push(id);
            }
        }
        res
    }

    fn get_invalid_ids2(&self) -> Vec<u64> {
        let mut res = Vec::new();
        for id in self.first..self.last + 1 {
            if !is_valid2(&id) {
                res.push(id);
            }
        }
        res
    }
}

fn is_valid(id: &u64) -> bool {
    let id_str = format!("{}", id);
    if id_str.len() % 2 != 0 {
        return true;
    }
    let half = id_str.len() / 2;
    let chars: Vec<char> = id_str.chars().collect();
    for i in 0..half {
        if chars[i] != chars[half + i] {
            return true;
        }
    }
    false
}

fn is_valid2(id: &u64) -> bool {
    let id_str = format!("{}", id);
    let mut pattern_size = id_str.len() - 1;
    while pattern_size >= 1 {
        if id_str.len() % pattern_size == 0 {
            let pattern_repetition = id_str.len() / pattern_size;
            let pattern = &id_str[..pattern_size];
            let mut pattern_found = true;
            for i in 1..pattern_repetition {
                if pattern != &id_str[i * pattern_size..(i + 1) * pattern_size] {
                    pattern_found = false;
                    break;
                }
            }
            if pattern_found {
                return false;
            }
        }
        pattern_size -= 1;
    }
    true
}

fn range(input: &str) -> IResult<&str, IdRange> {
    let (remainder, ids) = separated_list0(char('-'), parse_u64).parse(input)?;
    Ok((
        remainder,
        IdRange {
            first: *ids.get(0).expect("did not find the first ID"),
            last: *ids.get(1).expect("did not find the last ID"),
        },
    ))
}

fn ranges(input: &str) -> IResult<&str, Vec<IdRange>> {
    separated_list0(char(','), range).parse(input)
}

fn part1(ranges_ids: &Vec<IdRange>) -> u64 {
    let mut res = 0;
    for range in ranges_ids {
        res += range.get_invalid_ids().iter().sum::<u64>();
    }
    res
}

fn part2(ranges_ids: &Vec<IdRange>) -> u64 {
    let mut res = 0;
    for range in ranges_ids {
        res += range.get_invalid_ids2().iter().sum::<u64>();
    }
    res
}

pub fn run(input: &str) {
    let (_, r_ids) = ranges(input).expect("Could not parse the input rotations");
    println!("=> part1 : {}", part1(&r_ids));
    println!("=> part2 : {}", part2(&r_ids));
}

#[cfg(test)]
mod tests {
    use crate::day2::{part1, part2};

    use super::ranges;

    #[test]
    fn test() {
        let data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let (_, ranges_ids) = ranges(data).expect("Could not parse the input ranges");

        assert_eq!(part1(&ranges_ids), 1227775554);
        assert_eq!(part2(&ranges_ids), 4174379265);
    }
}
