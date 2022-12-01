fn main() {
    let mut totals: Vec<u32> = include_str!("../input/01.txt")
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap_or_default()).sum())
        .collect();

    totals.sort();
    totals.reverse();

    println!("A1: {}", totals.first().unwrap_or(&0));
    println!("A2: {}", totals.iter().take(3).sum::<u32>());
}
