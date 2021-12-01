use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_increase<'a>(depths: &[usize]) -> usize {
    let pairs = depths.windows(2);
    pairs.filter(|pair| pair[1] > pair[0]).count()
}

fn read_ints(filename: &str) -> Vec<usize> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|x| x.unwrap().parse().unwrap()).collect()
}

fn puzzle1(input: &[usize]) {
    println!("{}", count_increase(input));
}

fn puzzle2(input: &[usize]) {
    let windows : Vec<usize> = input.windows(3).map(|window| window.iter().sum()).collect();
    println!("{}", count_increase(&windows));
}

fn main() {
    let input = read_ints("day1/input.txt");
    puzzle1(&input);
    puzzle2(&input);
}
