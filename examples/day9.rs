use aoc2020::aoc::{load_data, Res};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::io::BufRead;
use std::collections::VecDeque;


fn find_second_star(numbers: &[i64], invalid: i64)-> Vec<i64>{
    let mut v = Vec::new();
    
    'outer: for i  in 0..numbers.len(){
        for j in i..numbers.len(){
            v.push(numbers[j]);
            let sum = v.iter().fold(0, |acc, el| acc + el); 
            if  sum == invalid {
                // found the solution, exit without cleanup
                println!("Solution is found!");
                break 'outer;
            } else if sum > invalid{
                break;
            }
        }
        v.clear();
    }

    v
}

fn is_valid(preamble: &VecDeque<i64>, n: i64) -> bool{
    let mut valid = false;
    'outer: for (i,p) in preamble.iter().enumerate(){
        for (j,q) in preamble.iter().enumerate(){
            if i != j {
                valid = n == q + p;
                if valid {
                    break 'outer;
                }
            }
        }
    }
    valid
}

fn invalid_numbers(v: &[i64], preamble_length: usize) -> Vec<i64>{
    let mut invalid  = Vec::new();
    let mut preamble = VecDeque::with_capacity(preamble_length as usize);
    for (i, n) in v.iter().enumerate(){
        if i >= preamble_length {
            if !is_valid(&preamble, *n){
                invalid.push(*n);
            }
            preamble.pop_front();
        }
        preamble.push_back(*n);
        println!("preamble[i={}] is {:?}", i, preamble);
    }
    invalid
}

fn main() -> Res<()> {
    let data = load_data("examples/data/day9.txt")?;
    let lines : Vec<_> = data.lines().filter_map(|l| l.ok().and_then(|s| s.parse::<i64>().ok())).collect();
    let invalid = invalid_numbers(&lines, 25).pop().unwrap();
    println!("invalid: {:?}", invalid);
    let mut second = find_second_star(&lines, invalid);
    second.sort();
    println!("Second: {:?}", second);
    println!("Res: {}", second[0] + second[second.len() -1]);
    Ok(())
}