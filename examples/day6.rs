use aoc2020::aoc::load_data;
use nom::{
    bits::complete::*, branch::*, bytes::complete::*, character::complete::*, combinator::*,
    multi::*, sequence::*, IResult,
};
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::BufRead;

type Group = Vec<Person>;

type Person = Vec<char>;

fn groups<I>(v: I) -> std::io::Result<Vec<Group>>
where
    I: Iterator<Item = std::io::Result<String>>,
{
    let mut groups = Vec::with_capacity(32);
    let mut current_group = Vec::new();
    for line in v {
        let l = line?;
        if l.is_empty() {
            groups.push(current_group);
            current_group = Vec::new();
        } else {
            current_group.push(l.chars().collect());
        }
    }
    groups.push(current_group);
    Ok(groups)
}

fn distinct<I, A>(v: I) -> Vec<A>
where
    I: Iterator<Item = A>,
    A: Hash + Eq,
{
    let mut set: HashSet<A> = HashSet::new();
    for a in v {
        set.insert(a);
    }
    set.drain().into_iter().collect()
}

fn everyone<I>(groups: I) -> Vec<usize>
where
    I: Iterator<Item = Group>,
{
   
    let mut counts = Vec::new();

    for group in groups {
        let mut count = 0;
        let mut map: HashMap<char, usize> = HashMap::new();
        for person in group.iter() {
            for ans in person.iter() {
                let mut value = map.entry(ans.clone()).or_default();
                *value += 1;
            }
            count += 1;
        }
        println!("map {:?}, count {}", map, count);
        let count = map
            .drain()
            .into_iter()
            .filter_map(|(k, c)| if c == count { Some(k) } else { None })
            .count();
        counts.push(count);
    }
    counts
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let data = load_data("examples/data/day6.txt")?;
    let example = load_data("examples/data/day6-example.txt")?;
    let mut ex_groups = groups(data.lines())?;
    let counts_distinct: Vec<usize> = ex_groups
        .iter()
        .map(|g| distinct(g.iter().flatten()).len())
        .collect();
    let counts_everyone: Vec<usize> = everyone(ex_groups.drain(0..ex_groups.len()));
    println!("{:?}", ex_groups);
    println!("{:?}", counts_distinct);
    println!("{:?}", counts_everyone.iter().sum::<usize>());

    let set: HashSet<&str> = vec!["a", "a"].into_iter().collect();
    println!("{:?}", set);
    Ok(())
}
