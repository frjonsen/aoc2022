fn main() {
    let input = include_str!("../../input.txt");
    let mut elves = input
        .split("\n\n")
        .map(|e| {
            e.split_whitespace()
                .map(|i| i.parse::<i64>().expect("Failed to parse item"))
                .sum::<i64>()
        })
        .collect::<Vec<_>>();

    elves.sort_by(|a, b| b.cmp(a));
    let total = elves.into_iter().take(3).sum::<i64>();
    println!("{total}")
}
