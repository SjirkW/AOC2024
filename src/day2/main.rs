use std::str::Lines;

const INPUT: &str = "src/day2/input.txt";
// const INPUT: &str = "src/day2/test.txt";

fn is_increasing(nums: &[i32]) -> bool {
    let mut increase_count = 0;
    for i in 0..3 {
        let diff = nums[i + 1] - nums[i];
        if diff > 0 {
            increase_count += 1;
        }
    }

    increase_count > 1
}

fn is_safe(nums: &[i32]) -> usize {
    let is_increasing = is_increasing(&nums);
    for (i, window) in nums.windows(2).enumerate() {
        let diff = window[1] - window[0];
        let increase = diff > 0;

        if (is_increasing && !increase)
            || (!is_increasing && increase)
            || diff.abs() < 1
            || diff.abs() > 3
        {
            return i;
        }
    }

    usize::MAX
}

fn count_safe(lines: Lines<'_>, check_faulty: bool) {
    let mut safe = 0;
    for line in lines {
        let nums: Vec<i32> = line_to_vec(line);

        let faulty_index = is_safe(&nums.as_slice());
        if faulty_index == usize::MAX {
            safe += 1;
        } else if (check_faulty) {
            let mut nums_copy = nums.clone();
            nums_copy.remove(faulty_index);

            let mut nums_copy2 = nums.clone();
            nums_copy2.remove(faulty_index + 1);

            if is_safe(&nums_copy) == usize::MAX || is_safe(&nums_copy2) == usize::MAX {
                safe += 1;
            }
        }
    }

    println!("{safe}");
}

pub fn run() {
    let lines = std::fs::read_to_string(INPUT).unwrap();
    let iter = lines
        .lines()
        .into_iter();
    count_safe(iter, false);
    count_safe(lines.lines(), true);
}
