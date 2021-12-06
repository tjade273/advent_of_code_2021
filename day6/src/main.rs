fn count_from_fish(fish: impl IntoIterator<Item = usize>) -> [usize; 9] {
    let mut fish_count: [usize; 9] = [0; 9];
    for f in fish {
        fish_count[f] += 1;
    }
    fish_count
}

fn advance(fish_count: &mut [usize; 9]) {
    fish_count.rotate_left(1);
    fish_count[6] += fish_count[8]
}

fn main() {
    let input = include_str!("input.txt")
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap());

    let mut fish_count = count_from_fish(input);
    for _ in 0..80 {
        advance(&mut fish_count)
    }
    println!("{}", fish_count.iter().sum::<usize>());
    for _ in 0..(256 - 80) {
        advance(&mut fish_count)
    }
    println!("{}", fish_count.iter().sum::<usize>())
}
