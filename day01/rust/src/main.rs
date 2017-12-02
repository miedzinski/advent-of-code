use std::io::{self, Read};

fn part1(nums: &[u32]) -> u32 {
    let sum: u32 = nums.iter()
        .zip(nums.iter().skip(1))
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .sum();
    // handle cycle
    if nums.first() == nums.last() {
        sum + nums[0]
    } else {
        sum
    }
}

fn part2(nums: &[u32]) -> u32 {
    nums.iter()
        .zip(nums.iter().skip(nums.len() / 2))
        .filter_map(|(a, b)| if a == b { Some(a + b) } else { None })
        .sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nums: Vec<_> = input.chars().map(|x| x.to_digit(10).unwrap()).collect();
    println!("part1: {}", part1(&nums));
    println!("part2: {}", part2(&nums));
}
