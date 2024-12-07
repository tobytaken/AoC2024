use std::fs::File;
use std::io::BufRead;

fn main() {
    let f = File::open("input.txt").expect("Failed to open input.txt");
    let content: Vec<String> = std::io::BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    let parsed_nums = parse_nums(&content);
    one(&parsed_nums);
    two(&parsed_nums);
}

fn one(parsed_nums: &Vec<Vec<i32>>) {
    let mut sum: i32 = 0;

    for line in parsed_nums {
        let mut last = -1;
        let mut pos = false;

        for (index, number) in line.iter().enumerate() {
            if index == 0 {
                last = *number;
                continue;
            }

            if index == 1 {
                pos = number > &last;
            }

            let diff = number - last;
            last = *number;

            if pos && diff.is_negative()
                || !pos && diff.is_positive()
                || diff.abs() > 3
                || diff.abs() < 1
            {
                break;
            }

            if index == line.len() - 1 {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}

fn two(parsed_nums: &Vec<Vec<i32>>) {
    let mut sum: i32 = 0;

    for line in parsed_nums {
        if is_safe(line) {
            sum += 1;
            continue;
        }

        let mut safe_with_removal = false;
        for i in 0..line.len() {
            let mut modified_line = line.clone();
            modified_line.remove(i);

            if is_safe(&modified_line) {
                safe_with_removal = true;
                break;
            }
        }

        if safe_with_removal {
            sum += 1;
        }
    }

    println!("{}", sum);
}

fn parse_nums(content: &Vec<String>) -> Vec<Vec<i32>> {
    let mut vector: Vec<Vec<i32>> = vec![];
    for line in content {
        let parsed_line: Vec<i32> = line
            .split_whitespace()
            .map(|c| c.parse().unwrap())
            .collect();
        vector.push(parsed_line);
    }
    vector
}

fn is_safe(line: &Vec<i32>) -> bool {
    let mut last = -1;
    let mut pos = false;

    for (index, number) in line.iter().enumerate() {
        if index == 0 {
            last = *number;
            continue;
        }

        if index == 1 {
            pos = number > &last;
        }

        let diff = number - last;

        if pos && diff.is_negative()
            || !pos && diff.is_positive()
            || diff.abs() > 3
            || diff.abs() < 1
        {
            return false;
        }

        last = *number;
    }

    return true;
}
