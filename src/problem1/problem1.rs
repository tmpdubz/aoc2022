use std::{
    fs,
    io::{prelude::*, BufReader},
    path::Path,
    // path::PathBuf,
};

struct Elf {
    ration_sum: i32
}

impl Elf {
    fn add(&mut self, ration_value: i32) {
        self.ration_sum += ration_value;
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    return buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
}

pub fn print_sample() {
    let file = "/Users/kosmik/repos/aoc2022/src/problem1/sample_data";
    print_solution(file, 1);
}

pub fn print_part1() {
    let file = "/Users/kosmik/repos/aoc2022/src/problem1/puzzle_input";
    print_solution(file, 1);
}

pub fn print_part2() {
    let file = "/Users/kosmik/repos/aoc2022/src/problem1/puzzle_input";
    print_solution(file, 3);
}

fn solution(lines: Vec<String>, sum_top_n: usize) -> i32 {
    // init first elf
    let mut curr_elf = Elf {
        ration_sum: 0
    };
    // init elf vector
    let mut elves = Vec::new();
    for line in lines {
        // make a new elf
        if line == "" {
            elves.push(curr_elf);
            curr_elf = Elf {
                ration_sum: 0
            };
        } else {
            curr_elf.add(line.parse::<i32>().unwrap());
        }
    }
    let mut ration_sums: Vec<i32> = elves.iter().map(|e| e.ration_sum).collect();
    ration_sums.sort_by(|a, b| b.cmp(a)); // sort reverse
    return ration_sums.iter().take(sum_top_n).sum();
}

fn print_solution(file: &str, sum_top_n: usize) {
    let lines: Vec<String> = lines_from_file(file);
    let sln = solution(lines, sum_top_n);
    println!("{:?}", sln);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(1, both_parts());
    }
}