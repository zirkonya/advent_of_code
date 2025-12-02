use std::error::Error;

use advent_of_code::read_input;
use logos::Logos;

#[derive(Logos, Debug)]
#[logos(skip(r"\n"))]
enum Direction {
    #[regex(r"L(\d*)", |lex| lex.slice()[1..].parse::<i32>().expect("Error while parsing Left count"))]
    Left(i32),
    #[regex(r"R(\d*)",  |lex| lex.slice()[1..].parse::<i32>().expect("Error while parsing Right count"))]
    Right(i32),
}

const MAX: i32 = 100;

fn puzzle_1() -> Result<i32, Box<dyn Error>> {
    let content = read_input(2025, 1, 1)?;
    let direction_lexer = Direction::lexer(&content);
    let mut current_value = 50;
    let mut count = 0;
    for dir in direction_lexer {
        let Ok(dir) = dir else {
            continue;
        };
        current_value = match dir {
            Direction::Right(count) => (current_value + count) % MAX,
            Direction::Left(count) => (current_value - count + MAX) % MAX,
        };
        if current_value == 0 {
            count += 1;
        }
    }
    Ok(count)
}

fn puzzle_2() -> Result<i32, Box<dyn Error>> {
    let content = read_input(2025, 1, 2)?; // Same as first puzzle
    let direction_lexer = Direction::lexer(&content);
    let mut count = 0;
    let mut current_value = 50;
    for dir in direction_lexer {
        let Ok(dir) = dir else {
            continue;
        };
        // chaque fois qu'on passe par 0 on ajoute 1
        current_value = match dir {
            Direction::Right(count) => current_value + count,
            Direction::Left(count) => current_value - count,
        };
        if !(0..=99).contains(&current_value) {
            let times = (current_value / MAX).abs();
            if current_value.is_negative() {
                current_value = (current_value + MAX * times) % MAX;
            } else {
                current_value %= MAX;
            }
            count += times;
        }
        if current_value == 0 {
            count += 1;
        }
    }
    Ok(count)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== 🎄 DAY 1 ===");
    println!("\tPUZZLE 1");
    let count = puzzle_1()?;
    println!("⛄ PASSWORD: {count}");
    println!("\tPUZZLE 2");
    let count = puzzle_2()?;
    println!("⛄ GOOD PASSWORD: {count}");
    Ok(())
}
