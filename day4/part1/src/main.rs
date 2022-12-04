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

fn calculate_full_overlaps(input: &str) -> usize {
    input
        .lines()
        .map(str::trim)
        .map(|p| p.split(',').map(parse_assignment).collect::<Vec<_>>())
        .filter(|p| {
            let left = p[0];
            let right = p[1];
            let largest = left.max(right);
            largest == left | right
        })
        .count()
}

fn main() {
    let input = include_str!("../../input.txt");
    let res = calculate_full_overlaps(input);

    println!("{res}")
}

#[cfg(test)]
mod test {
    use crate::{calculate_full_overlaps, parse_assignment};

    #[test]
    fn test_parse_interval() {
        let interval = "1-3";
        let res = parse_assignment(interval);
        println!("{res:b}");
        assert_eq!(7, res);
    }

    #[test]
    fn test_overlap() {
        let input = "2-8,3-7";
        assert_eq!(1, calculate_full_overlaps(input));
    }

    #[test]
    fn test_not_full_overlap() {
        let input = "2-8,1-7";
        assert_eq!(0, calculate_full_overlaps(input));
    }
}
