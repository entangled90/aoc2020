use aoc2020::aoc::{load_data, Res};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::io::BufRead;

fn combinations(n: usize) -> usize {
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 8 - 1,
        _ => panic!("invalid len")
    }
}


fn main() -> Res<()> {
    let mut data : Vec<u32>= load_data("examples/data/day10.txt")?
        .lines()
        .filter_map(|l| l.ok().and_then(|s| s.parse().ok())).collect();
    
    data.sort();

    // add my adapter

    let last = data.pop().expect("vector cannot be empty");
    data.push(last);
    data.push(last + 3);

    let result: Vec<u32> = data.iter().scan(0, |prev, &jolt| {
        let p = prev.clone();
        *prev = jolt;
        Some(jolt - p)
    }).collect();

    let (ones, threes) = result.iter().fold((0,0), |(ones, threes), next| {
        if *next == 1 {
            (ones + 1, threes)
        } else if *next == 3 {
            (ones, threes +1)
        } else {
            panic!(format!("Invalid number {}", next));
        }
    });
    println!("{}, {}", ones, threes);
    println!("Result: {:?}", result);
    println!("grouped: solution {}", ones * threes);

    // second star

    let splitted: Vec<_> = result.split(|x| *x == 3).into_iter().collect();
    println!("splitted {:?}", splitted);

    let comb : usize = splitted.iter().map(|v| combinations(v.len())).product();
    println!("number {}", comb);
    Ok(())
}
