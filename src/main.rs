use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(value_parser = clap::value_parser!(u8).range(1..))]
    day: u8,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    let dispatch = [
        day1::run,
        day2::run,
        day3::run,
        day4::run,
        day5::run,
        day6::run,
        day7::run,
        day8::run,
        day9::run,
    ];

    let dispatch_index = args.day as usize - 1;
    if dispatch_index < dispatch.len() {
        println!("Running day {}", args.day);
        let input = fs::read_to_string(format!("./resources/input{}", args.day))
            .expect("Could not load the input file");
        dispatch[dispatch_index](&input);
    } else {
        println!("Day {} not yet implemented !", args.day);
    }
}
