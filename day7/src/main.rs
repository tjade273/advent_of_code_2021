fn norm_part2(a: i32, b: i32) -> i32 {
    let n = (a - b).abs();
    n * (n + 1) / 2
}

fn total_fuel_part2(crabs: &Vec<i32>, pos: i32) -> i32 {
    crabs.iter().map(|&crab| norm_part2(crab, pos)).sum()
}

fn main() {
    let input = include_str!("input.txt");
    let mut crabs: Vec<i32> = input.strip_suffix('\n').unwrap().split(',').map(|n| n.parse().unwrap()).collect();
    let n = crabs.len();
    let (_, &mut pos, _) = crabs.select_nth_unstable(n / 2);

    let fuel_part1: i32 = crabs.iter().map(|&crab| (crab - pos).abs()).sum();

    let &min_crab = crabs.iter().min().unwrap();
    let &max_crab = crabs.iter().max().unwrap();
    let fuel_part2 = (min_crab..max_crab).map(|pos| total_fuel_part2(&crabs, pos)).min().unwrap();
    println!("{}, {}", fuel_part1, fuel_part2)
}
