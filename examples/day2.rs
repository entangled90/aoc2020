use aoc2020::aoc::load_data;
use nom::{character::complete::*, sequence::*, IResult};
use std::io::BufRead;

#[derive(Debug)]
struct PasswordPolicy {
    from: u32,
    to: u32,
    character: char,
}

fn parse_policy(str: &str) -> IResult<&str, (PasswordPolicy, String)> {
    tuple((
        digit1,
        char('-'),
        digit1,
        space1,
        anychar,
        char(':'),
        space1,
        alphanumeric1,
    ))(str)
    .map(|(rem, (from, _, to, _, character, _, _, password))| {
        (
            rem,
            (
                PasswordPolicy {
                    from: from.parse().unwrap(),
                    to: to.parse().unwrap(),
                    character,
                },
                password.to_string(),
            ),
        )
    })
}

fn pass_is_valid(str: &str) -> bool {
    let parse_result = parse_policy(str);
    let (_, (policy, password)) = parse_result.unwrap();
    let mut count = 0;
    for c in password.chars() {
        if c == policy.character {
            count += 1;
        }
    }
    count >= policy.from && count <= policy.to
}

fn pass_is_valid_2(str: &str) -> bool {
    let parse_result = parse_policy(str);
    let (_, (policy, password)) = parse_result.unwrap();
    let first = password.as_bytes().get((policy.from - 1) as usize) == Some(&(policy.character as u8));
    let second=  password.as_bytes().get((policy.to - 1) as usize) == Some(&(policy.character as u8));
    let res = first ^ second;
    // println!("policy {:?}, pass: {:?}, first {}, second {}, xor {}", policy, password,  first, second, res);
    res
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let reader = load_data("examples/data/day2-passwords.txt")?;
    let mut valid_passwords = 0;
    let mut valid_passwords_2 = 0;
    for line_err in reader.lines().into_iter() {
        let line = line_err?;
        if pass_is_valid(&line) {
            valid_passwords +=1;
        }
        if pass_is_valid_2(&line) {
            valid_passwords_2 +=1;
        }
    }

    println!("valid: {:?}", valid_passwords);
    println!("valid 2: {:?}", valid_passwords_2);

    Ok(())
}
