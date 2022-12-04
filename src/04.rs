fn main()
{
    let pairs: Vec<((u32, u32), (u32, u32))> = include_str!("../input/04.txt")
        .lines()
        .filter_map(|line| {
            let (x, y) = line.split_once(",")?;
            let (x0, x1) = x.split_once("-")?;
            let (y0, y1) = y.split_once("-")?;
            return Some(((x0.parse().ok()?, x1.parse().ok()?), (y0.parse().ok()?, y1.parse().ok()?)));
        })
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