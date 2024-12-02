use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn print_lines(input: &str) {
    if let Ok(lines) = read_lines(input) {
        for line in lines.flatten() {
            println!("{}", line);
        }
    }
}

pub fn line_to_vec(line: &str) -> Vec<i32> {
    return line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
}