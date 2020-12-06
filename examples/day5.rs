use aoc2020::aoc::load_data;
use nom::{
    bits::complete::*, branch::*, bytes::complete::*, character::complete::*, combinator::*,
    multi::*, sequence::*, IResult,
};
use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

fn binary_partition(input: &str, lower: char, upper: char) -> u64 {
    // 128 -> 0..127
    let mut min_value = 0;
    let mut max_value = 2u64.pow(input.len() as u32) - 1;
    for c in input.chars() {
        let half = (min_value + max_value)/ 2;
        match c {
            c if c == lower => max_value = half,
            c if c == upper => min_value = half + 1,
            c => panic!(format!("invalid char {}!", c))
        }
    }
    assert_eq!(min_value, max_value);
    min_value
}

fn decode(input: &str) -> (u64, u64, u64){
    let (row_input, col_input) = input.split_at(input.len() - 3);
    let row = binary_partition(row_input, 'F', 'B');
    let col = binary_partition(col_input, 'L', 'R');
    (row, col, row * 8 + col)
}


fn search_my_seat(seats: &[u64]) -> u64{
    let mut seat = 0;
    for (i, s) in seats.iter().enumerate(){
        if seats[i+1] != s + 1 {
            seat = s + 1;
            break;
        }
    }
    seat
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let reader = load_data("examples/data/day5.txt")?;
    // let examples = vec![("FBFBBFFRLR", (44, 5, 357)),("BFFFBBFRRR", (70, 7, 567)) , ("FFFBBBFRRR", (14, 7, 119)), ("BBFFBBFRLL", (102, 4, 820))];
    
    // for (src, res) in examples.iter(){
    //     let decoded = decode(src);
    //     assert_eq!(&decoded, res);
    //     println!("{:?}", decoded);
    // }
    let mut max = 0;
    let mut seats = Vec::with_capacity(1024);
    for line in reader.lines(){
        let (_, _, id) = decode(line?.as_str());
        if id > max {
            max = id;
        }
        seats.push(id);
    }
    println!("max: {}", max);

    seats.sort();
    println!("{:?}", seats);
    println!("my seat{}", search_my_seat(&seats.as_slice()));

    Ok(())
}
