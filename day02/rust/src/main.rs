use std::io::{self, Read};
use std::str::FromStr;

fn part1<Grid: AsRef<[Row]>, Row: AsRef<[usize]>>(rows: Grid) -> usize {
    rows.as_ref()
        .iter()
        .map(|row| {
            let row = row.as_ref();
            row.iter().max().unwrap() - row.iter().min().unwrap()
        })
        .sum()
}

fn sorted_pairs<T: Copy + Ord>(nums: &[T]) -> Vec<(T, T)> {
    let mut v = vec![];
    for (idx, x) in nums.iter().enumerate() {
        for y in &nums[idx + 1..] {
            if x > y {
                v.push((*x, *y))
            } else {
                v.push((*y, *x))
            }
        }
    }
    v
}

fn part2<Grid: AsRef<[Row]>, Row: AsRef<[usize]>>(rows: Grid) -> usize {
    rows.as_ref()
        .iter()
        .map(|row| {
            sorted_pairs(row.as_ref())
                .iter()
                .find(|&&(a, b)| a % b == 0)
                .map(|&(a, b)| a / b)
                .unwrap()
        })
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let rows: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| usize::from_str(x).unwrap())
                .collect()
        })
        .collect();
    println!("part1: {}", part1(&rows));
    println!("part2: {}", part2(&rows));
}
