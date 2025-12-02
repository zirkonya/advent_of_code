use std::fs;

pub fn read_input(year: i32, day: i32, puzzle: i32) -> std::io::Result<String> {
    fs::read_to_string(format!("input/input_{year:04}_{day:02}_{puzzle:02}.txt"))
}
