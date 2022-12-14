fn main() {
    let input = include_str!("../../input.txt");
    let max = input
        .split("\n\n")
        .map(|e| {
            e.split_whitespace()
                .map(|i| i.parse::<i64>().expect("Failed to parse item"))
                .sum::<i64>()
        })
        .max()
        .unwrap();
    println!("{max}")
}
