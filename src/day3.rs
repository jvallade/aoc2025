use nom::IResult;
use nom::Parser;
use nom::character::complete::{char, one_of};
use nom::combinator::map;
use nom::multi::many1;
use nom::multi::separated_list1;

fn bank(input: &str) -> IResult<&str, Vec<u8>> {
    many1(map(one_of("0123456789"), |c| {
        c.to_digit(10).expect("Could not parse u8 from char") as u8
    }))
    .parse(input)
}

fn banks(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    separated_list1(char('\n'), bank).parse(input)
}

fn part1(banks: &Vec<Vec<u8>>) -> u64 {
    let mut res: u64 = 0;
    for bank in banks {
        // search for the max first digit
        // it can't be on the last position
        let mut max_first: u8 = 0;
        let mut max_first_position = 0;
        for i in 0..bank.len() - 1 {
            if bank.get(i).unwrap() > &max_first {
                max_first = *bank.get(i).unwrap();
                max_first_position = i;
            }
        }
        let max_second = bank[max_first_position + 1..]
            .iter()
            .max()
            .expect("Could not find the max value");

        res += (10 * max_first + max_second) as u64
    }

    res
}

fn part2(banks: &Vec<Vec<u8>>) -> u64 {
    let mut res: u64 = 0;
    for bank in banks {
        let mut last_max_index = 0;
        let mut last_possible_index = bank.len() - 11;
        let mut bank_res = 0;
        for i in 0..12 {
            let (id_max, max) = bank[last_max_index..last_possible_index]
                .iter()
                .enumerate()
                .max_by(|(i1, val1), (i2, val2)| {
                    val1.cmp(val2).then(i2.cmp(i1)) // Compare values, then prefer smaller index
                })
                .expect("Max not found");
            last_max_index += id_max + 1;
            last_possible_index += 1;
            bank_res += *max as u64 * 10u64.pow(11 - i)
        }
        res += bank_res
    }
    res
}

pub fn run(input: &str) {
    let (_, r_ids) = banks(input).expect("Could not parse the input banks");
    println!("=> part1 : {}", part1(&r_ids));
    println!("=> part2 : {}", part2(&r_ids));
}

#[cfg(test)]
mod tests {
    use super::{banks, part1, part2};

    #[test]
    fn test() {
        let data = "987654321111111
811111111111119
234234234234278
818181911112111";

        let (_, banks) = banks(data).expect("Could not parse the input banks");

        assert_eq!(part1(&banks), 357);
        assert_eq!(part2(&banks), 3121910778619);
    }
}
