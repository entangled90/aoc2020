use aoc2020::aoc::{load_data, Res};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;
use std::hash::Hash;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Seat {
    Floor,
    Free,
    Occupied,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Seats {
    seats: Vec<Vec<Seat>>,
    width: usize,
    height: usize,
}

impl Seats {
    fn occupied(&self) -> usize{
        self.seats.iter().fold(0, |acc, row| {
            row.iter().fold(acc, |acc, s| {
                acc + (if *s == Seat::Occupied { 1 } else { 0 })
            })
        })
    }
}

impl Display for Seats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.seats.iter() {
            for seat in row.iter() {
                write!(f, "{}", seat)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Display for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Seat::Floor => '.',
            Seat::Free => 'L',
            Seat::Occupied => '#',
        };
        write!(f, "{}", repr)
    }
}

fn occupied_neighbours(seats: &Seats, i: i64, j: i64) -> u32 {
    let mut occupied = 0;
    for delta_i in -1..2 {
        for delta_j in -1..2 {
            if delta_i != 0 || delta_j != 0 {
                let i_prime = i + delta_i;
                let j_prime = j + delta_j;
                if let Some(s) = seats
                    .seats
                    .get(i_prime as usize)
                    .and_then(|r| r.get(j_prime as usize))
                {
                    if *s == Seat::Occupied {
                        occupied += 1;
                    }
                }
            }
        }
    }
    occupied
}

fn occupied_neighbours2(seats: &Seats, i: i64, j: i64) -> u32 {
    let mut occupied = 0;
    let max_multi: i64= std::cmp::max(seats.width, seats.height)as i64;
    for delta_i in -1..2 {
        for delta_j in -1..2 {
            for multi in 1..max_multi + 1 {
                if delta_i != 0 || delta_j != 0 {
                    let i_prime = i + multi* delta_i;
                    let j_prime = j + multi* delta_j;
                    if let Some(s) = seats
                        .seats
                        .get(i_prime as usize)
                        .and_then(|r| r.get(j_prime as usize))
                    {
                        if *s == Seat::Occupied {
                            occupied += 1;
                        }
                        if *s != Seat::Floor{
                            break;
                        }
                    }
                }
            }
        }
    }
    occupied
}

fn step<F>(previous_seats: &Seats, limit: u32, f: F) -> (u32, Seats)
where
    F: Fn(&Seats, i64, i64) -> u32,
{
    let mut changes = 0;
    let mut next_seats = previous_seats.clone();
    for i in 0..previous_seats.seats.len() {
        let row = &previous_seats.seats[i];
        for j in 0..(row.len()) {
            let occupied = f(previous_seats, i as i64, j as i64);
            let seat = unsafe { next_seats.seats.get_unchecked_mut(i).get_unchecked_mut(j) };
            if *seat != Seat::Floor {
                changes += match occupied {
                    0 if *seat == Seat::Free => {
                        *seat = Seat::Occupied;
                        1
                    }
                    n if n >= limit && *seat == Seat::Occupied => {
                        *seat = Seat::Free;
                        1
                    }
                    _ => 0,
                }
            }
            // println!("occupied {}, changes:{}, before: {}, after: {}", occupied, changes, seat_before_change, seat);
        }
    }
    (changes, next_seats)
}

fn main() -> Res<()> {
    let data: Vec<Vec<Seat>> = load_data("examples/data/day11.txt")?
        .lines()
        .map(|l| {
            let line = l.expect("Could not get line");
            line.chars()
                .map(|c| {
                    if c == 'L' {
                        Seat::Free
                    } else if c == '.' {
                        Seat::Floor
                    } else {
                        panic!("Invalid seat!")
                    }
                })
                .collect()
        })
        .collect();

    let seats_original = Seats {
        seats: data.clone(),
        width: data.len(),
        height: data[0].len(),
    };

    let mut seats = seats_original.clone();

    println!("Seat: \n{}", seats);
    loop {
        let (changes, next_seats) = step(&seats, 4, occupied_neighbours);
        seats = next_seats;
        println!("changes: {} , Seat: \n{}", changes, seats);
        if changes == 0 {
            break;
        }
    }

    println!("occupied: {}",  seats.occupied());


    println!("Step two");

    let mut seats = seats_original.clone();
    loop {
        let (changes, next_seats) = step(&seats, 5, occupied_neighbours2);
        seats = next_seats;
        // println!("changes: {} , Seat: \n{}", changes, seats);
        if changes == 0 {
            break;
        }
    }

    println!("occupied: {}",  seats.occupied());

    
    Ok(())
}
