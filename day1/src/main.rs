// Solutions for day 1 of ACO23

fn main() {
    let puzzle_input = include_str!("input.txt");

    solve_part1(puzzle_input);
    solve_part2(puzzle_input);
}

fn solve_part1(puzzle_input: &str) {
    let res: u32 = puzzle_input
        .lines()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
            lib::calculate_calibration_values(digits)
        })
        .sum();

    println!("Part 1 solution is {}", res);
}

fn solve_part2(puzzle_input: &str) {
    let res: u32 = puzzle_input
        .lines()
        .map(|line| {
            let processed_line = lib::replace_spelled_out_digits(line);
            let digits: Vec<char> = processed_line.chars().filter(|c| c.is_numeric()).collect();
            lib::calculate_calibration_values(digits)
        })
        .sum();

    println!("Part 2 solution is {}", res);
}

mod lib {

    pub fn calculate_calibration_values(digits: Vec<char>) -> u32 {
        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();

        let calibration_value: u32 = format!("{}{}", first_digit, last_digit).parse().unwrap();

        return calibration_value;
    }

    const NEEDLES: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    fn needle_as_digit(needle: &str) -> &str {
        match needle {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => panic!(),
        }
    }

    pub fn replace_spelled_out_digits(line: &str) -> String {
        let mut processed_line = line.to_string();

        while let Some((start_index, first_needle)) =
            find_next_spelled_out_digit(&processed_line, NEEDLES.to_vec())
        {
            processed_line.replace_range(
                start_index..start_index + 1,
                needle_as_digit(&first_needle).into(),
            );
        }

        processed_line
    }

    pub fn find_next_spelled_out_digit(line: &str, needles: Vec<&str>) -> Option<(usize, String)> {
        let mut search_results: Vec<(usize, String)> = needles
            .into_iter()
            .map(|needle| {
                line.find(needle)
                    .and_then(|start_index| Some((start_index, needle.to_string())))
            })
            .filter_map(std::convert::identity)
            .collect();

        // Sort by start index
        search_results.sort_by_key(|item| item.0);

        if let Some(first_match) = search_results.first() {
            return Some((first_match.0, first_match.1.clone()));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::lib::*;

    #[test]
    fn find_next_spelled_out_digit_works() {
        const NEEDLES: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        const EXAMPLE: &str = "eightwothree";

        let search_res = find_next_spelled_out_digit(EXAMPLE, NEEDLES.to_vec()).unwrap();

        assert_eq!("eight", search_res.1);
    }

    #[test]
    fn example_part_two_works() {
        const EXAMPLE_STRING: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

        let calibration_values: Vec<u32> = EXAMPLE_STRING
            .lines()
            .map(|line| {
                let processed_line = replace_spelled_out_digits(line);
                let digits: Vec<char> = processed_line.chars().filter(|c| c.is_numeric()).collect();
                calculate_calibration_values(digits)
            })
            .collect();

        assert_eq!(calibration_values[0], 29);
        assert_eq!(calibration_values[1], 83);
        assert_eq!(calibration_values[2], 13);
        assert_eq!(calibration_values[3], 24);
        assert_eq!(calibration_values[4], 42);
        assert_eq!(calibration_values[5], 14);
        assert_eq!(calibration_values[6], 76);

        let sum: u32 = calibration_values.into_iter().sum();

        assert_eq!(sum, 281);
    }
}
