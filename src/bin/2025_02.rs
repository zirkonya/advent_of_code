use advent_of_code::read_input;
use rayon::iter::{IntoParallelIterator, ParallelIterator as _};
use std::error::Error;

fn is_invalid_id(id: usize) -> bool {
    let id_str: &str = &id.to_string();
    let pivot = id_str.len();
    pivot & 1 == 0 && {
        let (l, r) = id_str.split_at(pivot >> 1);
        l == r
    }
}

fn is_invalid_id_2(id: usize) -> bool {
    let id_str: &[u8] = &id.to_string().as_bytes().to_owned();
    let len = id_str.len();
    for chunk in (1..len).rev() {
        if len.is_multiple_of(chunk) {
            let parts = id_str.chunks(chunk).collect::<Vec<_>>();
            let len = parts.len();
            if parts[1..]
                .into_iter()
                .zip(parts[0..(len - 1)].to_owned())
                .all(|(&lhs, rhs)| lhs == rhs)
            {
                return true;
            }
        }
    }
    false
}

// TODO : optimize this
fn puzzle(invalid: fn(usize) -> bool) -> Result<usize, Box<dyn Error>> {
    let input = read_input(2025, 2, 1)?
        .trim()
        .split(",")
        .map(|ids| {
            let Some((lhs, rhs)) = ids.split_once("-") else {
                panic!("unexpected wrong format");
            };
            let (lhs, rhs) = (lhs.parse::<usize>().unwrap(), rhs.parse::<usize>().unwrap());
            lhs..=rhs
        })
        .collect::<Vec<_>>();
    let sum: usize = input
        .into_par_iter()
        .map(|range| {
            range
                .into_par_iter()
                .filter(|&id| invalid(id))
                .sum::<usize>()
        })
        .sum();
    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== 🎄 DAY 2 ===");
    println!("\tPUZZLE 1");
    let wrong_id_sum = puzzle(is_invalid_id)?;
    println!("WRONG ID SUM: {wrong_id_sum}");
    println!("\tPUZZLE 2");
    let wrong_id_sum = puzzle(is_invalid_id_2)?;
    println!("WRONG ID SUM: {wrong_id_sum}");
    Ok(())
}
