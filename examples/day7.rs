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

fn remove_bags(s: &str) -> Option<(u32, String)> {
    let re = Regex::new(r"^\s*(\d)*\s*([\w\s]*)\s*(bag[s\.]*)").unwrap();
    re.captures(s).and_then(|captures| {
        let num = captures.get(1).and_then(|s| s.as_str().parse::<u32>().ok());
        let name = captures.get(2).map(|s| s.as_str());
        num.and_then(|n| name.map(|_name| (n, _name.trim().to_owned())))
    })
}

fn search_dfs(
    // graph
    m: &HashMap<String, Vec<(u32, String)>>,
    from: &str,
    searched: &str,
) -> u64 {
    match m.get(from) {
        Some(vs) => {
            let mut total = 0;
            // println!("Searching into {:?}, from {}", vs, from);
            for (_, s) in vs.iter() {
                // already found a match
                if total == 0 {
                    if s == searched {
                        total += 1;
                    } else {
                        total += search_dfs(m, s, searched);
                    }
                }
            }
            total
        }
        None => 0,
    }
}

fn count_bag_dfs(
    // graph
    m: &HashMap<String, Vec<(u32, String)>>,
    from: &str,
) -> u64 {
    match m.get(from) {
        Some(vs) => {
            let mut total = 0;
            // println!("Searching into {:?}, from {}", vs, from);
            for (c, s) in vs.iter() {
                // already found a match
                total += (*c as u64);
                total += (*c as u64)*count_bag_dfs(m, s);
            }
            total
        }
        None => 1,
    }
}

fn search_for(m: &HashMap<String, Vec<(u32, String)>>, searched: &str) -> u64 {
    let mut total = 0;
    for (container, contained) in m {
        // println!("starting dfs at {}", container);
        total += search_dfs(m, container, searched);
    }
    total
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let data = load_data("examples/data/day7.txt")?;
    let mut rules = HashMap::new();
    for line in data.lines() {
        let l: String = line?;
        let split: Vec<_> = l.split("contain").into_iter().collect();
        // println!("split {:?}", split);
        let container = split[0];
        let contained: Vec<(u32, String)> = split[1]
            .split(",")
            .map(|s| remove_bags(s))
            .flatten()
            .collect::<Vec<_>>();

        rules.insert(container[0..container.len() - 6].to_owned(), contained);
    }
    println!("{:?}", rules);

    // println!("{:?}", search_for(&rules, "shiny gold"));

    println!("second start{}", count_bag_dfs(&rules, "shiny gold"));
    Ok(())
}
