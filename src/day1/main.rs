use std::{collections::HashMap, str::Lines};

use crate::utils::line_to_vec;

// const INPUT: &str = "src/day1/input.txt";
const INPUT: &str = "src/day1/test.txt";

fn a(lines: Lines<'_>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines {
        let nums: Vec<i32> = line_to_vec(line);
        left.push(nums[0]);
        right.push(nums[1]);
    }

    left.sort();
    right.sort();

    let mut total = 0;
    for i in 0..left.len() {
        total += (left[i] - right[i]).abs();
    }

    println!("a: {total}", );
}

fn b(lines: Lines<'_>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in lines {
        let nums: Vec<i32> = line_to_vec(line);
        left.push(nums[0]);
        right.push(nums[1]);
    }

    let mut counts = HashMap::new();

    for &num in &right {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mut total: i32 = 0;
    for &left_num in &left {
        if let Some(&amount_in_right) = counts.get(&left_num) {
            total += amount_in_right * left_num;
        }
    }
    
    println!("b: {total}", );
}

pub fn run() {
    let lines = std::fs::read_to_string(INPUT).unwrap();
    let iter = lines
        .lines()
        .into_iter();
    a(iter.clone());
    b(iter.clone());
}
