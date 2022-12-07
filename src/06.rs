use std::collections::HashSet;
use itertools::Itertools;

fn find_start(n: usize) -> usize
{
    include_str!("../input/06.txt")
    .as_bytes()
    .windows(n)
    .find_position(|&window| window.iter().collect::<HashSet<&u8>>().len() == n)
    .unwrap()
    .0 + n
}

fn main()
{
    println!("A1: {}", find_start(4));
    println!("A2: {}", find_start(14));
}