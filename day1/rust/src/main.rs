type Input = (Vec<i32>, Vec<i32>);
type Output = i32;

fn main() -> anyhow::Result<()> {
    let mut input = load_input("../example.txt")?;

    input.0.sort();
    input.1.sort();

    println!("Part 1: {}", solve_one(&input));
    println!("Part 2: {}", solve_two(&input));

    Ok(())
}

fn solve_one((left, right): &Input) -> Output {
    left.iter()
        .zip(right.iter())
        .map(|(&l, &r)| (l - r).abs())
        .sum::<i32>()
}

fn solve_two((left, right): &Input) -> Output {
    left.iter()
        .map(|&vl| {
            right
                .iter()
                .fold(0, |acc, &vr| if vl == vr { acc + 1 } else { acc })
                * vl
        })
        .sum::<i32>()
}

fn load_input(path: &str) -> anyhow::Result<Input> {
    let content = std::fs::read_to_string(path)?;
    let (left, right) = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(" ")
                .filter(|s| !s.is_empty())
                .map(|word| word.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .fold((Vec::new(), Vec::new()), |(mut left, mut right), values| {
            left.push(values[0]);
            right.push(values[1]);
            (left, right)
        });
    Ok((left, right))
}
