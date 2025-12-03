use advent_of_code::read_input;
use rayon::iter::ParallelBridge;
use std::error::Error;

fn puzzle_1() -> Result<i64, Box<dyn Error>> {
    Ok(read_input(2025, 3, 1)?
        .lines()
        .map(|line| get_max_joltage(line.as_bytes(), 2))
        .sum::<i64>())
}

fn puzzle_2() -> Result<i64, Box<dyn Error>> {
    Ok(read_input(2025, 3, 2)?
        .lines()
        .map(|line| get_max_joltage(line.as_bytes(), 12))
        .sum::<i64>())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== 🎄 DAY 1 ===");
    println!("\tPUZZLE 1");
    let max_joltage = puzzle_1()?;
    println!("MAX JOLTAGE: {max_joltage}");
    println!("\tPUZZLE 2");
    let max_joltage = puzzle_2()?;
    println!("MAX JOLTAGE: {max_joltage}");
    Ok(())
}

fn get_max_joltage(sequence: &[u8], n: usize) -> i64 {
    let mut joltage = 0;
    let len = sequence.len();
    let mut start = 0;
    for k in (0..n).rev() {
        let mut max = (0, 0);
        for i in start..(len - k) {
            if sequence[i] > max.0 {
                max = (sequence[i], i);
            }
        }
        joltage = joltage * 10 + (max.0 - b'0') as i64;
        start = max.1 + 1;
    }
    joltage
}

#[cfg(test)]
mod tests {
    use crate::get_max_joltage;

    #[test]
    fn test_puzzle_1() {
        let sequence = b"987654321111111";
        assert_eq!(get_max_joltage(sequence, 2), 98);
        let sequence = b"811111111111119";
        assert_eq!(get_max_joltage(sequence, 2), 89);
        let sequence = b"234234234234278";
        assert_eq!(get_max_joltage(sequence, 2), 78);
        let sequence = b"818181911112111";
        assert_eq!(get_max_joltage(sequence, 2), 92);
    }

    #[test]
    fn test_puzzle_2() {
        let sequence = b"987654321111111";
        assert_eq!(get_max_joltage(sequence, 12), 987654321111);
        let sequence = b"811111111111119";
        assert_eq!(get_max_joltage(sequence, 12), 811111111119);
        let sequence = b"234234234234278";
        assert_eq!(get_max_joltage(sequence, 12), 434234234278);
        let sequence = b"818181911112111";
        assert_eq!(get_max_joltage(sequence, 12), 888911112111);
    }
}
