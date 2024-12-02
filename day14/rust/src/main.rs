type Input = Vec<i32>;
type Output = i32;

fn main() -> anyhow::Result<()> {
    let input = load_input("../input.txt")?;

    println!("Part 1: {}", solve_one(&input));
    println!("Part 2: {}", solve_two(&input));

    Ok(())
}

fn solve_one(input: &Input) -> Output {
    42
}

fn solve_two(input: &Input) -> Output {
    42
}

fn load_input(path: &str) -> anyhow::Result<Input> {
    let content = std::fs::read_to_string(path)?;

    let data = Vec::new();

    Ok(data)
}
