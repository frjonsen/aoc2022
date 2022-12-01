#[derive(PartialEq, Eq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

struct Round {
    me: Shape,
    opponent: Shape,
}

impl TryInto<Shape> for &str {
    type Error = String;

    fn try_into(self) -> Result<Shape, Self::Error> {
        match self {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissor),
            rest => Err(format!("{rest} is not a known shape")),
        }
    }
}

fn parse_input(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(str::trim)
        .map(|r| {
            let choices: Vec<&str> = r.split_whitespace().collect();
            let opponent: Shape = choices[0].try_into().expect("Failed to convert to shape");
            let me: Shape = match (&opponent, choices[1]) {
                (Shape::Rock, "X") => Shape::Scissor,
                (Shape::Rock, "Y") => Shape::Rock,
                (Shape::Rock, "Z") => Shape::Paper,
                (Shape::Paper, "X") => Shape::Rock,
                (Shape::Paper, "Y") => Shape::Paper,
                (Shape::Paper, "Z") => Shape::Scissor,
                (Shape::Scissor, "X") => Shape::Paper,
                (Shape::Scissor, "Y") => Shape::Scissor,
                (Shape::Scissor, "Z") => Shape::Rock,
                _ => panic!("Unexpected combination"),
            };

            Round { me, opponent }
        })
        .collect()
}

fn run_round(round: &Round) -> u64 {
    match round.me {
        Shape::Paper => {
            let base = 2;
            match round.opponent {
                Shape::Rock => 6 + base,
                Shape::Paper => 3 + base,
                Shape::Scissor => base,
            }
        }
        Shape::Rock => {
            let base = 1;
            match round.opponent {
                Shape::Rock => 3 + base,
                Shape::Paper => base,
                Shape::Scissor => 6 + base,
            }
        }
        Shape::Scissor => {
            let base = 3;
            match round.opponent {
                Shape::Rock => base,
                Shape::Paper => 6 + base,
                Shape::Scissor => 3 + base,
            }
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let points: u64 = parse_input(input).into_iter().map(|r| run_round(&r)).sum();
    println!("{points}")
}

#[cfg(test)]
mod test {
    use crate::{parse_input, run_round, Round, Shape};

    #[test]
    fn test_parse_input() {
        let input = "A Y
        B X
        C Z";

        let parsed = parse_input(input);
        assert_eq!(parsed[0].me, Shape::Rock);
        assert_eq!(parsed[0].opponent, Shape::Rock);

        assert_eq!(parsed[1].me, Shape::Rock);
        assert_eq!(parsed[1].opponent, Shape::Paper);

        assert_eq!(parsed[2].me, Shape::Rock);
        assert_eq!(parsed[2].opponent, Shape::Scissor);
    }

    #[test]
    fn test_run_round_rock_paper() {
        let round = Round {
            opponent: Shape::Rock,
            me: Shape::Paper,
        };

        let res = run_round(&round);
        assert_eq!(8, res)
    }

    #[test]
    fn test_run_round_paper_rock() {
        let round = Round {
            opponent: Shape::Paper,
            me: Shape::Rock,
        };

        let res = run_round(&round);
        assert_eq!(1, res);
    }

    #[test]
    fn test_run_round_scissor_scissor() {
        let round = Round {
            opponent: Shape::Scissor,
            me: Shape::Scissor,
        };

        let res = run_round(&round);
        assert_eq!(6, res)
    }
}
