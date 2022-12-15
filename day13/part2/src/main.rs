use element::Element;
use parser::parse_input;

mod element;
mod parser;

fn run(mut pairs: Vec<Vec<Element>>) -> usize {
    let p1 = vec![Element::Array(vec![Element::Digit(2)])];
    let p2 = vec![Element::Array(vec![Element::Digit(6)])];

    pairs.push(vec![Element::Array(vec![Element::Digit(2)])]);
    pairs.push(vec![Element::Array(vec![Element::Digit(6)])]);
    pairs.sort();

    let mut p1_pos: Option<usize> = None;
    let mut p2_pos: Option<usize> = None;
    for (i, packet) in pairs.into_iter().enumerate() {
        if packet == p1 {
            p1_pos = Some(i + 1)
        } else if packet == p2 {
            p2_pos = Some(i + 1)
        }

        if p1_pos.is_some() && p2_pos.is_some() {
            break;
        }
    }

    let (Some(p1_pos), Some(p2_pos)) = (p1_pos, p2_pos) else {
        panic!("Didn't find indices for packets");
    };

    p1_pos * p2_pos
}

fn main() {
    let input = include_str!("../../input.txt").trim();
    let input = parse_input(input);
    let res = run(input);

    println!("{res}")
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use test_case::test_case;

    use crate::{
        element::Element,
        parser::{parse_array, parse_input},
        run,
    };

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

        assert_eq!(140, res);
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
