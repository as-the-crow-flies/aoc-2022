use std::fs;

fn main() {
    let path = "input/01.txt";

    let mut totals = fs::read_to_string(path)
        .expect(format!("Should have been able to read input file: '{}'", path).as_str())
        .lines()
        .fold(vec![0], |mut totals, input| {
            if let (Some(total), Ok(calories)) = (totals.last_mut(), input.parse::<u32>()) {
                *total += calories;
            } else {
                totals.push(0);
            }
            
            return totals;
        });

    totals.sort();
    totals.reverse();

    let max = totals.first().unwrap_or(&0);
    println!("A1: The most calories being carried by any Elf is {:?}", max);

    let top3 = totals.iter().take(3).fold(0, |x, y| x + y);
    println!("A2: The total calories being carried by the top three Elves is {:?}", top3);
}
