use aoc2020::aoc::load_data;
use nom::{
    bits::complete::*, branch::*, bytes::complete::*, character::complete::*, combinator::*,
    multi::*, sequence::*, IResult,
};
use std::collections::HashMap;
use std::io::BufRead;
use regex::Regex;


fn separator(i: &str) -> IResult<&str, &str> {
    alt((map(newline, |_| ""), space1))(i)
}

// input is a single line with no /n
fn parser(input: &str) -> IResult<&str, HashMap<&str, &str>> {
    map(
        separated_list1(
            separator,
            tuple((
                alphanumeric1,
                char(':'),
                take_till(|c| c == '\n' || c == ' '),
            )),
        ),
        |mut vec| {
            let mut result = HashMap::new();
            for (k, _, v) in vec.drain(0..vec.len()) {
                result.insert(k, v);
            }
            result
        },
    )(input)
}

fn contains_all(m: &HashMap<&str, &str>, keys: &Vec<&str>) -> bool {
    keys.iter().fold(true, |acc, el| acc && m.contains_key(el))
}

fn decode(vec: &Vec<String>) -> Vec<PassFields> {
    vec.iter()
        .map(|line| 
            parser(line.as_str())
                .map(|(_, map)| map).unwrap_or(HashMap::new())).collect()
}

fn check_passports(vec: &Vec<PassFields>, keys: &Vec<&str>) -> usize {
    vec.iter().filter(|v| contains_all(v, keys)).count()
}

type PassFields<'a> = HashMap<&'a str, &'a str>;

fn check_field<F, P, A>(field: &str, fields: &PassFields, decode: F, verify: P) -> Option<bool>
where
    F: FnOnce(&str) -> Option<A>,
    P: FnOnce(A) -> bool,
{
    let p = decode(fields.get(field)?)?;
    Some(verify(p))
}

fn validate_fields(m: &HashMap<&str, &str>) -> bool {
    let color_regex =  Regex::new(r"^#[0-9a-f]{6}$").unwrap(); 
    let valid_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let pass_regex = Regex::new(r"^\d{9}$").unwrap();
    let conditions = vec![check_field(
        "byr",
        m,
        |s| s.parse::<u32>().ok(),
        |yr| yr >= 1920 && yr <= 2002,
    ),
    check_field(
        "iyr",
        m,
        |s| s.parse::<u32>().ok(),
        |yr| yr >= 2010 && yr <= 2020,
    ),
    check_field(
        "eyr",
        m,
        |s| s.parse::<u32>().ok(),
        |yr| yr >= 2020 && yr <= 2030,
    ),
    check_field(
        "hgt",
        m,
        |s| {
            let (l, u) = s.split_at(s.len() -2);
            Some((l.to_string(), u.to_string()))
        },
        |(len, unit)|{
            match (len.parse::<u32>().ok(), unit.as_str()) {
                (Some(l), "cm") => l >= 150 && l <= 193,
                (Some(l), "in") => l >= 59 && l <= 76,
                _ => false
            }
        },
    ),
    check_field(
        "hcl",
        m,
        |s| Some(s.to_string()),
        |s| color_regex.is_match(&s),
    ),
    check_field(
        "ecl",
        m,
        |s| Some(s.to_string()),
        |s| valid_colors.contains(&s.as_str()),
    ),
    check_field(
        "pid",
        m,
        |s| Some(s.to_string()),
        |s| pass_regex.is_match(&s),
    ),
    ];
    conditions.iter().all(|b| b.unwrap_or(false))
}

fn read_input(iter: Vec<String>) -> Vec<String> {
    let mut lines = Vec::with_capacity(64);
    let mut previous_line: Option<String> = None;
    for line in iter.iter() {
        if line.is_empty() {
            if let Some(prev) = previous_line.take() {
                lines.push(prev);
            }
        } else {
            match previous_line {
                Some(ref mut prev) => prev.push_str(" "),
                None => (),
            };
            let prev = previous_line.get_or_insert(String::new());
            prev.push_str(line);
        }
    }
    if let Some(line) = previous_line.take() {
        lines.push(line);
    }
    lines
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let reader = load_data("examples/data/day4.txt")?;
    // cid is not needed
    let keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut file_lines = Vec::with_capacity(64);
    for line in reader.lines() {
        file_lines.push(line?);
    }
    // file_lines.truncate(6);
    let input = read_input(file_lines);
    println!("{:?}", input);
    let decoded = decode(&input);
    let exercise_count = check_passports(&decoded, &keys);
    println!("exercise count: {}", exercise_count);
    println!("validated {:?}", decoded.iter().filter(|m| contains_all(m, &keys) && validate_fields(m)).count());
    Ok(())
}
