use aoc2020::aoc::load_data;
use std::io::BufRead;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let reader = load_data("examples/data/day1-expenses.txt")?;

    let mut expenses: Vec<u64> = Vec::with_capacity(1024);

    for (_, s) in reader.lines().enumerate() {
        expenses.push(s?.parse()?);
    }
    let mut result = Vec::new();
    for i in 0..expenses.len() {
        for j in i + 1..expenses.len() {
            let v1 = expenses.get(i).unwrap();
            let v2 = expenses.get(j).unwrap();
            if (v1 + v2) == 2020 {
                result.push(vec![v1, v2]);
            }
            for k in j + 1..expenses.len() {
                let v3 = expenses.get(k).unwrap();
                if v1 + v2 + v3 == 2020 {
                    result.push(vec![v1, v2, v3])
                }
            }
        }
    }

    assert_eq!(result.len(), 2);
    for res in result {
        println!("Result is {:?}", res.iter().fold(1, |acc, &v| acc * v));
    }
    Ok(())
}
