use itertools::Itertools;
use regex::Regex;

fn main()
{
    let (crates_input, procedure_input) = include_str!("../input/05.txt").split_once("\n\n").unwrap();

    let mut crates = vec![vec![]; 9];

    crates_input
        .lines()
        .rev()
        .skip(1)
        .for_each(|line| line
            .chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i,x)| if x.is_alphabetic() { crates[i].push(x) }));

    let re = Regex::new(r"\d+").unwrap();
    let procedure: Vec<_> = procedure_input
        .lines()
        .filter_map(|line| re.find_iter(line).filter_map(|x| x.as_str().parse::<usize>().ok()).next_tuple())
        .map(|(n, from, to)| (n, from-1, to-1))
        .collect();

    println!("A1: {}", a1(&mut crates.clone(), &procedure));
    println!("A2: {}", a2(&mut crates.clone(), &procedure));
}

fn a1(crates: &mut Vec<Vec<char>>, procedure: &Vec<(usize, usize, usize)>) -> String
{
    procedure
        .iter()
        .for_each(|&(n, from, to)| {
            let stack: Vec<_> = (0..n).filter_map(|_| crates[from].pop()).collect();
            crates[to].extend(stack);
        });

    crates.iter().map(|x| x.last().unwrap()).collect()
}

fn a2(crates: &mut Vec<Vec<char>>, procedure: &Vec<(usize, usize, usize)>) -> String
{
    procedure
        .iter()
        .for_each(|&(n, from, to)| {
            let stack: Vec<_> = (0..n).filter_map(|_| crates[from].pop()).collect();
            crates[to].extend(stack.iter().rev());
        });

    crates.iter().map(|x| x.last().unwrap()).collect()
}