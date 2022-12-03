use std::collections::{HashSet};

fn set(s: &str) -> HashSet<char>
{
    HashSet::<_>::from_iter(s.chars())
}

fn priority(item: char) -> u32
{
    if item.is_lowercase() { item as u32 - 'a' as u32 + 1 }
    else { item as u32 - 'A' as u32 + 27 }
}

fn main()
{
    let a1: u32 = include_str!("../input/03.txt")
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| *(set(left).intersection(&set(right)).next().unwrap()))
        .map(priority)
        .sum();

    println!("A1: {}", a1);
}