use std::collections::HashMap;

use itertools::Itertools;

fn main()
{
    let mut files = HashMap::<String, usize>::new();
    let mut path = Vec::<&str>::new();

    for line in include_str!("../input/07.txt").lines()
    {
        match line.split(" ").collect_vec().as_slice() {
            &["$", "cd", "/"] => { path = vec!["/"]; }
            &["$", "cd", ".."] => { path = path[..path.len()-1].to_vec(); }
            &["$", "cd", dir] => { path.push(&dir); }
            &["dir", _] | &["$", "ls"] => { }
            &[size, file] => { 
                files.insert(path.join("/") + "/" + file, size.parse::<usize>().unwrap());
            }
            _ => ()
        }
    }

    let mut directories = HashMap::<String, usize>::new();

    for (file, size) in files.iter()
    {
        let paths = file.split("/").collect_vec();
        (0..paths.len())
            .map(|i| paths.iter().take(i).join("/"))
            .for_each(|dir| {
                directories
                    .entry(dir)
                    .and_modify(|s| *s+= *size)
                    .or_insert(*size);
            })
    }
    
    let a1: usize = directories
        .values()
        .filter(|&s| *s <= 100000)
        .sum();
    
    println!("A1: {:?}", a1);

    let available = 70000000 - directories.get("/").unwrap();
    let to_free = 30000000 - available;

    let a2 = directories
        .values()
        .filter(|&s| *s > to_free)
        .min()
        .unwrap();

    println!("A2: {:?}", a2);
}