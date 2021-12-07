fn main() {
    let input = include_str!("input.txt");
    let mut crabs: Vec<i32> = input.strip_suffix('\n').unwrap().split(',').map(|n| n.parse().unwrap()).collect();
    let n = crabs.len();
    let (_, &mut pos, _) = crabs.select_nth_unstable(n / 2);
    let fuel: i32 = crabs.iter().map(|&crab| (crab - pos).abs()).sum();
    println!("{}", fuel)
}
