fn main() {
    let puzzle_input = include_str!("input.txt");

    solve_part1(puzzle_input);

    solve_part2(puzzle_input);
}

fn solve_part1(puzzle_input: &str) {

    fn is_cube_count_valid(color: &str, count: usize) -> bool {
        match color {
            "red" => count <= 12,
            "green" => count <= 13,
            "blue" => count <= 14,
            _ => panic!(),
        }
    }

    let mut sum: usize = 0;

    for line in puzzle_input.lines() {
        let (game_str, pulls_str) = {
            let mut parts = line.split(":").take(2);

            (parts.next().unwrap(), parts.next().unwrap())
        };

        let game_id: usize = game_str["Game ".len()..].parse().unwrap();

        let is_valid_game = pulls_str
            .split(";")
            .map(|pull_str| pull_str.split(","))
            .flatten()
            .all(|take_str| {
                let mut parts = take_str.trim().split(" ");

                let count: usize = parts.next().unwrap().parse().unwrap();
                let color = parts.next().unwrap();

                is_cube_count_valid(color, count)
            });

        if is_valid_game {
            sum += game_id;
        }
    }

    println!("Solution for part 1 is {}", sum);

}

fn solve_part2 (puzzle_input: &str) {
    let mut power_sum: usize = 0;

    for line in puzzle_input.lines() {

        let (_, pulls_str) = {
            let mut parts = line.split(":").take(2);

            (parts.next().unwrap(), parts.next().unwrap())
        };

        let takes = pulls_str
            .split(";")
            .map(|pull_str| pull_str.split(","))
            .flatten();

        let mut counts: [usize; 3] = [0, 0, 0];

        for take_str in takes {

            let mut parts = take_str.trim().split(" ");

            let count: usize = parts.next().unwrap().parse().unwrap();
            let color = parts.next().unwrap();

            let count_index = match color {
                "red" => 0,
                "blue" => 1,
                "green" => 2,
                _ => panic!()
            };

            counts[count_index] = std::cmp::max(count, counts[count_index]);
        }

        let game_power: usize = counts.into_iter().product();

        power_sum += game_power;
    }

    println!("Solution for part 2 is {}", power_sum);
}
