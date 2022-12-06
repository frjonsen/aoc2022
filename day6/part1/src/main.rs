fn find_marker(input: &str) -> usize {
    const MESSAGE_LENGTH: usize = 4;
    let (intro, rest) = input.split_at(MESSAGE_LENGTH - 1);
    let mut start = ['0'; MESSAGE_LENGTH];
    intro
        .chars()
        .enumerate()
        .for_each(|(i, c)| start[i + 1] = c);
    rest.chars()
        .scan(start, |state, c| {
            state.rotate_left(1);
            state[state.len() - 1] = c;
            Some(*state)
        })
        .enumerate()
        .find_map(|(i, c)| {
            for (index, ch) in c.iter().enumerate() {
                let rest = &c[(index + 1)..];
                if rest.contains(ch) {
                    return None;
                }
            }
            Some(i + MESSAGE_LENGTH)
        })
        .expect("Didn't find start of marker")
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", find_marker(input));
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::find_marker;

    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_find_marker(input: &str, index: usize) {
        assert_eq!(index, find_marker(input));
    }
}
