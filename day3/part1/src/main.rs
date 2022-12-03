fn find_duplicate(rucksack: &str) -> char {
    let size = rucksack.len();
    let left = rucksack.get(..=(size / 2)).unwrap();
    let right = rucksack.get(size / 2..).unwrap();

    for c in left.chars() {
        if let Some(q) = right.chars().find(|f| *f == c) {
            return q;
        }
    }

    panic!("No duplicate found");
}

fn get_value(item: char) -> u32 {
    let converted = item.to_ascii_uppercase() as u32 - 64;

    if item.is_ascii_uppercase() {
        converted + 26
    } else {
        converted
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let sum: u32 = input
        .trim()
        .lines()
        .map(str::trim)
        .map(find_duplicate)
        .map(get_value)
        .sum();

    println!("{sum}")
}

#[cfg(test)]
mod tests {
    use crate::{find_duplicate, get_value};
    use test_case::test_case;

    #[test_case("vJrwpWtwJgWrhcsFMMfFFhFp", 'p')]
    #[test_case("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L')]
    #[test_case("PmmdzqPrVvPwwTWBwg", 'P')]
    #[test_case("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 'v')]
    #[test_case("ttgJtRGJQctTZtZT", 't')]
    #[test_case("CrZsJsPPZsGzwwsLwLmpwMDw", 's')]
    fn test_samples(sample: &str, expected: char) {
        assert_eq!(expected, find_duplicate(sample));
    }

    #[test_case('p', 16)]
    #[test_case('L', 38)]
    fn test_convert(item: char, value: u32) {
        assert_eq!(value, get_value(item))
    }
}
