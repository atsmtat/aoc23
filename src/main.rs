use std::{env, fs, io};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() -> io::Result<()> {
    let mut args = env::args();
    let file_name = args.nth(1).expect("missing filename");
    let input = fs::read_to_string(file_name)?;

    let day = args.next().expect("missing day");

    match day.as_str() {
        "1" => {
            println!("part1 answer: {}", day1::part_one(&input));
            println!("part2 answer: {}", day1::part_two(&input));
        }
        "2" => {
            println!("part1 answer: {}", day2::part_one(&input));
            println!("part2 answer: {}", day2::part_two(&input));
        }
        "3" => {
            println!("part1 answer: {}", day3::part_one(&input));
            println!("part2 answer: {}", day3::part_two(&input));
        }
        "4" => {
            println!("part1 answer: {}", day4::part_one(&input));
            println!("part2 answer: {}", day4::part_two(&input));
        }
        "5" => {
            println!("part1 answer: {}", day5::part_one(&input));
            println!("part2 answer: {}", day5::part_two(&input));
        }
        "6" => {
            println!("part1 answer: {}", day6::part_one(&input));
            println!("part2 answer: {}", day6::part_two(&input));
        }
        "7" => {
            println!("part1 answer: {}", day7::part_one(&input));
            println!("part2 answer: {}", day7::part_two(&input));
        }
        "8" => {
            println!("part1 answer: {}", day8::part_one(&input));
            println!("part2 answer: {}", day8::part_two(&input));
        }
        _ => {
            panic!("unimplemented!");
        }
    }
    Ok(())
}
