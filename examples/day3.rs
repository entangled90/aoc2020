use aoc2020::aoc::load_data;
use nom::{character::complete::*, sequence::*, IResult};
use std::io::BufRead;

type Matrix<T> = Vec<Vec<T>>;

fn entry_at<T>(m: &Matrix<T>, i: usize, j: usize) -> Option<&T> {
    m.get(i).and_then(|inner| inner.get(j % inner.len()))
}

fn traverse_matrix(matrix: &Matrix<char>, stride_i: usize, stride_j: usize) -> u64 {
    let mut trees = 0;
    let mut i = 0;
    let mut j = 0;
    while let Some(entry) = entry_at(&matrix, i, j) {
        if *entry == '#' {
            trees += 1;
        }
        i += stride_i;
        j += stride_j;
    }
    trees
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let reader = load_data("examples/data/day3.txt")?;
    let mut matrix: Matrix<char> = Vec::with_capacity(32);
    for line in reader.lines() {
        matrix.push(line?.chars().collect());
    }
    let strides = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut total_trees = 1;
    for (stride_n, (j, i)) in strides.iter().enumerate() {
        let trees =             traverse_matrix(&matrix, *i, *j);
        total_trees *= trees;
        println!(
            "trees for stride {}: {}",
            stride_n, trees
        );
    }
    println!("total trees {}", total_trees);

    Ok(())
}
