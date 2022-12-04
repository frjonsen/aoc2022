fn parse_assignment(range: &str) -> u128 {
    let mut parts = range.split('-');
    let start = parts.next().unwrap().parse::<u32>().unwrap();
    let end = parts.next().unwrap().parse::<u32>().unwrap();

    let mut assignment: u128 = 0;
    for i in (start - 1)..end {
        assignment |= 2u128.pow(i);
    }

    assignment
}

fn calculate_overlaps(input: &str) -> usize {
    input
        .lines()
        .map(str::trim)
        .map(|p| p.split_once(',').unwrap())
        .filter(|p| {
            let left = parse_assignment(p.0);
            let right = parse_assignment(p.1);

            left & right != 0
        })
        .count()
}

fn main() {
    let input = include_str!("../../input.txt");
    let res = calculate_overlaps(input);

    println!("{res}")
}

#[cfg(test)]
mod test {
    use crate::{calculate_overlaps, parse_assignment};

    #[test]
    fn test_parse_interval() {
        let interval = "1-3";
        let res = parse_assignment(interval);
        println!("{res:b}");
        assert_eq!(7, res);
    }

    #[test]
    fn test_full_overlap() {
        let input = "2-8,3-7";
        assert_eq!(1, calculate_overlaps(input));
    }

    #[test]
    fn test_partial_overlap() {
        let input = "1-3,2-4";
        assert_eq!(1, calculate_overlaps(input))
    }

    #[test]
    fn test_no_overlap() {
        let input = "1-3,4-8";
        assert_eq!(0, calculate_overlaps(input));
    }
}
