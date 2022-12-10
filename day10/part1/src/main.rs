fn run(input: &str) -> i32 {
    let mut cycle = 0;
    let mut signal_strength_sum = 0;
    let mut register_x = 1;

    for instruction in input.lines().map(str::trim) {
        cycle += 1;
        if cycle == 20 || (cycle + 20) % 40 == 0 {
            signal_strength_sum += register_x * cycle;
        }

        if instruction == "noop" {
            continue;
        }

        let (_, value) = instruction.split_once(' ').unwrap();

        let value = value.parse::<i32>().unwrap();
        cycle += 1;

        if cycle == 20 || (cycle + 20) % 40 == 0 {
            signal_strength_sum += register_x * cycle;
        }
        register_x += value;
    }

    signal_strength_sum
}

fn main() {
    let input = include_str!("../../input.txt");
    let res = run(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn test_sample() {
        let input = include_str!("../sample.txt");
        let res = run(input);
        assert_eq!(13140, res);
    }
}
