use std::collections::HashSet;
use itertools::Itertools;

fn set(s: &str) -> HashSet<char>
{
    s.chars().collect::<HashSet<_>>()
}

fn priority(item: char) -> u32
{
    if item.is_lowercase() { item as u32 - 'a' as u32 + 1 }
    else { item as u32 - 'A' as u32 + 27 }
}

fn main()
{
    let input = include_str!("../input/03.txt");

    let a1: u32 = input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| *((&set(left) & &set(right)).iter().next().unwrap()))
        .map(priority)
        .sum();

    let a2: u32 = input
        .lines()
        .map(set)
        .tuples()
        .map(|(x0, x1, x2)| *((&(&x0 & &x1) & &x2).iter().next().unwrap()))
        .map(priority)
        .sum();
    
    println!("A1: {}", a1);
    println!("A2: {}", a2);
}