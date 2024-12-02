type Input = Vec<Vec<i32>>;
type Output = i32;

fn main() -> anyhow::Result<()> {
    let input = load_input("../input.txt")?;

    println!("Part 1: {}", solve_one(&input));
    println!("Part 2: {}", solve_two(&input));

    Ok(())
}

fn solve_one(input: &Input) -> Output {
    input.iter().map(check_report).sum()
}

fn solve_two(input: &Input) -> Output {
    input
        .iter()
        .map(|report| {
            if check_report(report) == 1 {
                return 1;
            }
            for index in 0..report.len() - 1 {
                let difference = (report[index] - report[index + 1]).abs();
                if difference < 1 || difference > 3 {
                    for i in index..=index + 1 {
                        let mut new_report = report.clone();
                        new_report.remove(i);
                        if check_report(&new_report) == 1 {
                            return 1;
                        }
                    }
                }
            }
            for index in 0..report.len() - 2 {
                let right_difference = report[index + 2] - report[index + 1];
                let left_difference = report[index + 1] - report[index];
                if (right_difference > 0 && left_difference < 0)
                    || (right_difference < 0 && left_difference > 0)
                {
                    for i in index..index + 3 {
                        let mut new_report = report.clone();
                        new_report.remove(i);
                        if check_report(&new_report) == 1 {
                            return 1;
                        }
                    }
                }
            }
            0
        })
        .sum()
}

fn check_report(report: &Vec<i32>) -> i32 {
    let changes: Vec<i32> = report
        .windows(2)
        .map(|window| -1 * (window[0] - window[1]))
        .collect();

    if !changes.iter().map(|&v| v.abs()).all(|v| v >= 1 && v <= 3) {
        return 0;
    }

    if changes.iter().any(|&v| v > 0) && changes.iter().any(|&v| v < 0) {
        return 0;
    }

    1
}

fn load_input(path: &str) -> anyhow::Result<Input> {
    let content = std::fs::read_to_string(path)?;

    let data = content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    Ok(data)
}
