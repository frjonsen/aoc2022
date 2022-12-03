use std::collections::HashSet;

fn get_value(item: char) -> u32 {
    let converted = item.to_ascii_uppercase() as u32 - 64;

    if item.is_ascii_uppercase() {
        converted + 26
    } else {
        converted
    }
}

fn find_badge(rucksacks: &mut [HashSet<char>]) -> char {
    let (intersection, others) = rucksacks.split_at_mut(1);
    let intersection = &mut intersection[0];
    for other in others {
        intersection.retain(|e| other.contains(e))
    }

    *intersection.iter().next().unwrap()
}

fn main() {
    let input = include_str!("../../input.txt");
    let sum: u32 = input
        .trim()
        .lines()
        .map(str::trim)
        .map(|l| l.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>()
        .chunks_exact_mut(3)
        .map(find_badge)
        .map(get_value)
        .sum();

    println!("{sum}")
}

#[cfg(test)]
mod tests {
    use crate::get_value;
    use test_case::test_case;

    #[test_case('p', 16)]
    #[test_case('L', 38)]
    fn test_convert(item: char, value: u32) {
        assert_eq!(value, get_value(item))
    }
}
