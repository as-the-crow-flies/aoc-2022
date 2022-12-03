#[derive(Copy, Clone)]
enum Shape
{
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

#[derive(Copy, Clone)]
enum Outcome
{
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Shape
{
    fn new(c: &str) -> Shape
    {
        match c {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!()
        }
    }

    fn beats(&self, other: &Shape) -> bool
    {
        match (self, other) {
            (Shape::Rock, Shape::Scissors) | (Shape::Scissors, Shape::Paper) | (Shape::Paper, Shape::Rock) => true,
            _ => false
        }
    }

    fn outcome(&self, other: &Shape) -> Outcome
    {
        if self.beats(other) { Outcome::Win }
        else if other.beats(self) { Outcome::Lose }
        else { Outcome::Draw }
    }

    fn score(&self, other: &Shape) -> u32
    {
        *self as u32 + self.outcome(other) as u32
    }
}

impl Outcome
{
    fn new(c: &str) -> Outcome
    {
        match c {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!()
        }
    }

    fn shape(&self, other: &Shape) -> Shape
    {
        match (self, other) {
            (Outcome::Lose, Shape::Paper) | (Outcome::Draw, Shape::Rock) | (Outcome::Win, Shape::Scissors)  => Shape::Rock,
            (Outcome::Lose, Shape::Scissors) | (Outcome::Draw, Shape::Paper) | (Outcome::Win, Shape::Rock)  => Shape::Paper,
            _ => Shape::Scissors
        }
    }

    fn score(&self, other: &Shape) -> u32
    {
        self.shape(other).score(other)
    }
}

fn main()
{
    let input: Vec<Vec<_>> = include_str!("../input/02.txt")
        .lines()
        .map(|line| line.split(" ").collect())
        .collect();

    let a1: u32 = input
        .iter()
        .map(|entry| Shape::new(entry[1]).score(&Shape::new(entry[0])))
        .sum();

    let a2: u32 = input
        .iter()
        .map(|entry| Outcome::new(entry[1]).score(&Shape::new(entry[0])))
        .sum();

    println!("A1: {}", a1);
    println!("A2: {}", a2);
}