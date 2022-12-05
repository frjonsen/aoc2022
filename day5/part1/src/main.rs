use itertools::Itertools;
use std::collections::VecDeque;

fn parse_start(columns: &str) -> Vec<VecDeque<char>> {
    let mut lines = columns.lines().rev();
    let last_line = lines.next().expect("Failed to take last line");
    let nr_of_columns = (last_line.len() + 1) / 4;

    let mut stacks: Vec<VecDeque<char>> = (0..nr_of_columns).map(|_| VecDeque::new()).collect();
    for line in lines {
        for (index, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[index].push_front(c);
            }
        }
    }

    stacks
}

fn main() {
    let input = include_str!("../../input.txt");
    let (start, moves) = input.split_once("\n\n").expect("Failed to find split");
    let mut stacks = parse_start(start);

    for line in moves.lines() {
        let moves = line
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|r| r.parse::<usize>().unwrap())
            .collect_tuple::<(usize, usize, usize)>()
            .unwrap();

        for _ in 0..moves.0 {
            let c = stacks[moves.1 - 1]
                .pop_front()
                .expect(&format!("Stack {} was empty", moves.1));
            stacks[moves.2 - 1].push_front(c);
        }
    }

    for mut stack in stacks {
        print!("{}", stack.pop_front().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_start;

    #[test]
    fn test_parse() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";
        let mut res = parse_start(input);
        assert_eq!(res[1].pop_front().unwrap(), 'D');
        assert_eq!(res[1].pop_front().unwrap(), 'C');
        assert_eq!(res[1].pop_front().unwrap(), 'M');
        assert_eq!(res[0].pop_front().unwrap(), 'N');
        assert_eq!(res[0].pop_front().unwrap(), 'Z');
        assert_eq!(res[2].pop_front().unwrap(), 'P');
    }
}
