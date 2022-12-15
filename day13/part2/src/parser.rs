use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::delimited,
    Finish, IResult,
};

use crate::element::Element;

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

pub fn parse_array(input: &str) -> IResult<&str, Vec<Element>> {
    let empty_array = map(tag("[]"), |_| Element::Array(vec![]));
    let mut f = alt((empty_array, parse_non_empty_array));
    f(input).map(|c| match c.1 {
        Element::Array(f) => (c.0, f),
        _ => panic!("Wrong element type"),
    })
}
pub fn parse_input(input: &str) -> Vec<Vec<Element>> {
    let mut pairs = Vec::new();
    for packet in input.lines().filter(|l| !l.trim().is_empty()) {
        let p = all_consuming(parse_array)(packet).finish().unwrap().1;
        pairs.push(p)
    }

    pairs
}

#[cfg(test)]
mod tests {
    use crate::{element::Element, parser::parse_array};

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
}
