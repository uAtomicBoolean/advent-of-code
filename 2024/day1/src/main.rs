use std::{collections::HashMap, fs};

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let mut ids = line.split_whitespace();
            let left: i32 = ids
                .next()
                .and_then(|i| i.parse().ok())
                .expect("Error with left part");
            let right: i32 = ids
                .next()
                .and_then(|i| i.parse().ok())
                .expect("Error with right part");

            (left, right)
        })
        .unzip()
}

fn part_one(left: Vec<i32>, right: Vec<i32>) -> u32 {
    let mut left = left.clone();
    let mut right = right.clone();

    left.sort_unstable();
    right.sort_unstable();

    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

fn part_two(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut right_hm: HashMap<&i32, i32> = HashMap::new();
    for r in right.iter() {
        *right_hm.entry(r).or_default() += 1;
    }

    left.iter().map(|l| l * right_hm.get(l).unwrap_or(&0)).sum()
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error while reading file.");
    let (left, right) = parse_input(&input);
    println!("Part one : {}", part_one(left.clone(), right.clone()));
    println!("Part two : {}", part_two(left, right));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "3   4
		4   3
		2   5
		1   3
		3   9
		3   3";

    #[test]
    fn test_parse_input() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(parse_input(INPUT), (left, right));
    }

    #[test]
    fn test_part_one() {
        let (left, right) = parse_input(INPUT);
        assert_eq!(part_one(left, right), 11);
    }

    #[test]
    fn test_part_two() {
        let (left, right) = parse_input(INPUT);
        assert_eq!(part_two(left, right), 31);
    }
}
