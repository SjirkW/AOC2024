// use std::str::Lines;
use regex::Regex;

const INPUT: &str = "src/day3/input.txt";
// const INPUT: &str = "src/day3/test.txt";

fn a() {
    let lines = std::fs::read_to_string(INPUT).unwrap();
    let iter = lines
        .lines()
        .into_iter();
   
    let mut total: i32 = 0;
    for line in iter {
        // use regex to match "mul(x,y)" multiple times
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let caps = re.captures_iter(line);

        for (i, cap) in caps.enumerate() {
            // Get both values
            let x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            total += x * y;
        }
    }

    println!("Total: {}", total);
}

fn b() {
    let lines = std::fs::read_to_string(INPUT).unwrap();
    let iter = lines
        .lines()
        .into_iter();
   
    let mut total: i32 = 0;
    let mut enabled = true;
    for line in iter {
        let mut left_side = "".to_owned();
        let mut right_side = line;

        let len = right_side.len();
        for i in 0..len {
            let char = right_side.chars().nth(0);
            right_side = &right_side[1..];
            left_side.push(char.unwrap());

            if (left_side.ends_with("do()")) {
                enabled = true;
            } else if (left_side.ends_with("don't()")) {
                enabled = false;
            }

            if left_side.ends_with("mul") {                
                let re = Regex::new(r"\((\d+),(\d+)\)").unwrap();
                let cap = re.captures(&right_side).unwrap();

                let index = cap.get(1).unwrap().start();
                if (index == 1 && enabled) {
                    let x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();

                    total += x * y;
                }
            }
        }
    }

    print!("Total: {}", total);
}

pub fn run() {
    // a();
    b();
}
