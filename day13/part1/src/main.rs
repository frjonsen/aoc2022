use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::delimited,
    Finish, IResult,
};

#[derive(PartialEq, Eq, Debug)]
enum Element {
    Digit(i32),
    Array(Vec<Element>),
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Element::Digit(l), Element::Digit(r)) => l.cmp(r),
            (Element::Digit(l), Element::Array(r)) => vec![Element::Digit(*l)].cmp(r),
            (Element::Array(l), Element::Digit(r)) => l.cmp(&vec![Element::Digit(*r)]),
            (Element::Array(l), Element::Array(r)) => l.cmp(r),
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_digit(input: &str) -> IResult<&str, Element> {
    map(digit1, |c: &str| Element::Digit(c.parse().unwrap()))(input)
}

fn parse_non_empty_array(input: &str) -> IResult<&str, Element> {
    let empty_array = map(tag("[]"), |_| Element::Array(vec![]));
    let array_contents = alt((empty_array, parse_non_empty_array, parse_digit));
    let non_empty_array = delimited(
        tag("["),
        separated_list0(alt((tag(", "), tag(","))), array_contents),
        tag("]"),
    );
    map(non_empty_array, |c| Element::Array(c))(input)
}

fn parse_array(input: &str) -> IResult<&str, Vec<Element>> {
    let empty_array = map(tag("[]"), |_| Element::Array(vec![]));
    let mut f = alt((empty_array, parse_non_empty_array));
    f(input).map(|c| match c.1 {
        Element::Array(f) => (c.0, f),
        _ => panic!("Wrong element type"),
    })
}
fn parse_input(input: &str) -> Vec<(Vec<Element>, Vec<Element>)> {
    let mut pairs = Vec::new();
    for pair in input.split("\n\n") {
        let (left, right) = pair.split_once("\n").unwrap();
        let left = all_consuming(parse_array)(left).finish().unwrap().1;
        let right = all_consuming(parse_array)(right).finish().unwrap().1;
        pairs.push((left, right));
    }

    pairs
}

fn run(pairs: Vec<(Vec<Element>, Vec<Element>)>) -> usize {
    println!("Checking {} pairs", pairs.len());
    pairs
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if p.0.cmp(&p.1) != Ordering::Greater {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input.txt").trim();
    let input = parse_input(input);
    let res = run(input);

    println!("{res} are in correct order")
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{parse_array, parse_input, run, Element};
    use test_case::test_case;

    #[test]
    fn test_parse_single_digit() {
        let res = parse_array("[5]").unwrap();
        assert_eq!(res.1, vec![Element::Digit(5)]);
    }

    #[test]
    fn test_parse_multiple_digits() {
        let res = parse_array("[2, 3, 5]").unwrap();
        assert_eq!(
            res.1,
            vec![Element::Digit(2), Element::Digit(3), Element::Digit(5)]
        )
    }

    #[test]
    fn test_parse_multi_digit_number() {
        let res = parse_array("[20]").unwrap();
        assert_eq!(res.1, vec![Element::Digit(20)])
    }

    #[test]
    fn test_parse_empty_array() {
        let res = parse_array("[]").unwrap();
        assert_eq!(res.1, vec![])
    }

    #[test]
    fn test_nested_array() {
        let res = parse_array("[5, [2]]").unwrap();
        assert_eq!(
            res.1,
            vec![Element::Digit(5), Element::Array(vec![Element::Digit(2)])]
        )
    }

    #[test]
    fn test_nested_empty_array() {
        let res = parse_array("[[]]").unwrap();
        assert_eq!(res.1, vec![Element::Array(vec![])]);
    }

    #[test]
    fn test_pair_1() {
        let left: Vec<Element> = [1, 1, 3, 1, 1].into_iter().map(Element::Digit).collect();
        let right: Vec<Element> = [1, 1, 5, 1, 1].into_iter().map(Element::Digit).collect();

        assert_eq!(left.cmp(&right), Ordering::Less);
    }

    #[test]
    fn test_pair_2() {
        let left = "[[1],[2,3,4]]";
        let left = parse_array(left).unwrap().1;
        let right = "[[1],4]";
        let right = parse_array(right).unwrap().1;

        assert_eq!(left.cmp(&right), Ordering::Less);
    }

    #[test]
    fn test_pair_3() {
        let left = "[9]";
        let right = "[8,7,6]";
        let left = parse_array(left).unwrap().1;
        let right = parse_array(right).unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn test_pair_4() {
        let left = "[[4,4],4,4]";
        let right = "[[4,4],4,4,4]";
        let left = parse_array(left).unwrap().1;
        let right = parse_array(right).unwrap().1;
        assert_eq!(left.cmp(&right), Ordering::Less);
    }

    #[test]
    fn test_pair_5() {
        let left: Vec<_> = [7].repeat(4).into_iter().map(Element::Digit).collect();
        let right = [7].repeat(3).into_iter().map(Element::Digit).collect();
        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn test_pair_6() {
        let left = vec![];
        let right = vec![Element::Digit(3)];

        assert_eq!(left.cmp(&right), Ordering::Less);
    }

    #[test]
    fn test_pair_7() {
        let left = "[[[]]]";
        let left = parse_array(left).unwrap().1;
        let right = "[[]]";
        let right = parse_array(right).unwrap().1;

        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn test_pair_8() {
        let left = "[1, [2, [3, [4, [5, 6, 7]]]], 8, 9]";
        let left = parse_array(left).unwrap().1;
        let right = "[1,[2,[3,[4,[5,6,0]]]], 8, 9]";
        let right = parse_array(right).unwrap().1;

        assert_eq!(left.cmp(&right), Ordering::Greater);
    }

    #[test]
    fn test_sample() {
        let input = include_str!("../sample.txt");
        let input = parse_input(input);
        let res = run(input);

        assert_eq!(13, res);
    }

    #[test_case("[[4],3]", "[[5],2]", true)]
    #[test_case("[[1],[2,3,4]]", "[[1],4]", true)]
    #[test_case("[[[]]]", "[[]]", false)]
    #[test_case("[[1],1]", "[1,1,1]", true)]
    #[test_case("[3]", "[[]]", false)]
    #[test_case("[[[3]]]", "[[3]]", true)]
    #[test_case("[1,1,1]", "[1,1,1]", true)]
    #[test_case("[[8,[[7]]]]", "[[[[8]]]]", false)]
    #[test_case("[1,2,3,[1,2,3],4,1]", "[1,2,3,[1,2,3],4,0]", false)]
    #[test_case("[8,[[7]]]", "[[[8],2]]", true)]
    #[test_case("[[1,2],4]", "[[[3]],5,5]", true)]
    #[test_case("[[1,2],4]", "[[1],5,5]", false)]
    fn test_found_cases(left: &str, right: &str, expected: bool) {
        let left = parse_array(left).unwrap().1;
        let right = parse_array(right).unwrap().1;
        assert!((left.cmp(&right) != Ordering::Greater) == expected);
    }
}
