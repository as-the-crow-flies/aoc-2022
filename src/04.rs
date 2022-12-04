use itertools::Itertools;

fn main()
{
    let pairs: Vec<_> = include_str!("../input/04.txt")
        .lines()
        .map(|line| line
            .split(",")
            .map(|elf| elf
                .split("-")
                .map(|x| x.parse::<u32>().unwrap())
                .next_tuple()
                .unwrap())
            .next_tuple()
            .unwrap())
        .collect();

    let a1: u32 = pairs
        .iter()
        .map(|((x0, x1), (y0, y1))| ((x0 <= y0 && x1 >= y1) || (y0 <= x0 && y1 >= x1)) as u32)
        .sum();

    let a2: u32 = pairs
        .iter()
        .map(|((x0, x1), (y0, y1))| ((x0 <= y1 && y0 <= x1) || (y0 <= x1 && x0 <= y1)) as u32)
        .sum();

    println!("A1: {:?}", a1);
    println!("A2: {:?}", a2);
}