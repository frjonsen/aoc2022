fn draw(position: i32, register: i32) {
    if ((register - 1)..=(register + 1)).contains(&position) {
        print!("#")
    } else {
        print!(".")
    }

    if position != 0 && (position + 1) % 40 == 0 {
        println!()
    }
}

fn run(input: &str) {
    let mut cycle = 0;
    let mut register_x = 1;

    for instruction in input.lines().map(str::trim) {
        draw(cycle % 40, register_x);
        cycle += 1;

        if instruction == "noop" {
            continue;
        }

        draw(cycle % 40, register_x);
        cycle += 1;

        let (_, value) = instruction.split_once(' ').unwrap();
        let value = value.parse::<i32>().unwrap();
        register_x += value;
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    run(input);
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn test_sample() {
        let input = include_str!("../sample.txt");
        run(input);
    }
}
