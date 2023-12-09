fn main() {
    let puzzle_input = include_str!("input.txt");

    let res = solve_part1(puzzle_input);

    println!("Solution of part 1 is {}", res);

    let res2 = solve_part2(puzzle_input);

    println!("Solution of part 1 is {}", res2);
}

fn solve_part1(puzzle_input: &str) -> i64 {
    let mut res: i64 = 0;

    for line in puzzle_input.lines() {
        let numbers: Vec<i32> = line.split(" ").map(|n| n.parse().unwrap()).collect();

        let extrapolation: i32 = extrapolate_report(&numbers).into();

        res += extrapolation as i64;
    }

    res
}

fn solve_part2(puzzle_input: &str) -> i64 {
    let mut res: i64 = 0;

    for line in puzzle_input.lines() {
        let mut numbers: Vec<i32> = line.split(" ").map(|n| n.parse().unwrap()).collect();

        numbers.reverse();

        let extrapolation: i32 = extrapolate_report(&numbers).into();

        res += extrapolation as i64;
    }

    res
}

fn extrapolate_report(history: &Vec<i32>) -> i32 {

    if history.clone().into_iter().all(|d| d == 0) {
        return 0;
    }

    let diff: Vec<i32> = history.into_iter().zip(history.into_iter().skip(1)).map(|(a, b)| b - a).collect();

    let extrapolation = history.last().unwrap().clone() + extrapolate_report(&diff);

    extrapolation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extrapolate_report_works() {

        let res = extrapolate_report(&vec![0, 3, 6, 9, 12, 15]);

        assert_eq!(18, res);

        let res2 = extrapolate_report(&vec![1, 3, 6, 10, 15, 21]);

        assert_eq!(28, res2);
    }

    #[test]
    fn example1_works() {
        let puzzle_input = include_str!("example_input.txt");

        let res = solve_part1(puzzle_input);

        assert_eq!(114, res);
    }

    #[test]
    fn example2_works() {
        let puzzle_input = "10  13  16  21  30  45";

        let res = solve_part2(puzzle_input);

        assert_eq!(964, res);
    }
}
