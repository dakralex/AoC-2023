use crate::utils::aoc::AocSolution;

pub struct AocDay;

/// Solves the AoC 2023 Day 1 challenge, see [here](https://adventofcode.com/2023/day/1).
impl AocSolution<1> for AocDay {
    type ResponseType = u32;

    fn solve_first(&self, input: &str) -> Self::ResponseType {
        input
            .lines()
            .map(|line| {
                let first = line.chars().find(char::is_ascii_digit).unwrap_or('0');
                let last = line.chars().rfind(char::is_ascii_digit).unwrap_or('0');

                let number = format!("{}{}", first, last);

                number.parse::<u32>().unwrap()
            })
            .sum()
    }

    fn solve_second(&self, input: &str) -> Self::ResponseType {
        const NUM_MATCHES: [&str; 19] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", //
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ];

        input
            .lines()
            .map(|line| {
                let matches = NUM_MATCHES
                    .iter()
                    .flat_map(|num| line.match_indices(num).collect::<Vec<_>>())
                    .collect::<Vec<_>>();

                let (_, first) = matches
                    .iter()
                    .min_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs))
                    .unwrap();
                let (_, last) = matches
                    .iter()
                    .max_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs))
                    .unwrap();

                let number = format!(
                    "{}{}",
                    AocDay::match_number(first),
                    AocDay::match_number(last)
                );

                number.parse::<u32>().unwrap()
            })
            .sum()
    }
}

impl AocDay {
    fn match_number(number: &str) -> u32 {
        match number.parse() {
            Ok(number) => number,
            Err(why) => match number {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => panic!("Couldn't match number: {}", why),
            },
        }
    }
}
