fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| l.chars().map(|b| b.to_digit(2).unwrap() as usize).collect())
        .collect()
}

fn column_majority(mat: &[Vec<usize>], col: usize) -> usize {
    (2 * mat.iter().map(|r| r[col]).sum::<usize>() >= mat.len()) as usize
}

fn gamma_vec(bits: &[Vec<usize>]) -> Vec<usize> {
    (0..bits[0].len())
        .map(|col| column_majority(bits, col))
        .collect()
}

fn bits_to_int(bits: &[usize]) -> usize {
    bits.iter().fold(0, |acc, b| (acc << 1) + b)
}

fn gas_rating(_bits: &[Vec<usize>], negate: bool) -> Vec<usize> {
    let mut bits = _bits.to_owned();
    let mut col = 0;
    while bits.len() > 1 {
        let maj = column_majority(&bits, col);
        bits.retain(|r| negate ^ (r[col] == maj));
        col += 1
    }
    bits[0].clone()
}

fn main() {
    let input = include_str!("input.txt");
    let input_bits = parse_input(input);
    let gamma_vec = gamma_vec(&input_bits);
    let epsilon_vec: Vec<usize> = gamma_vec.iter().map(|b| (b + 1) & 1).collect();
    let power = bits_to_int(&gamma_vec) * bits_to_int(&epsilon_vec);
    let oxygen = gas_rating(&input_bits, true);
    let co2 = gas_rating(&input_bits, false);
    let status = bits_to_int(&oxygen) * bits_to_int(&co2);
    println!("{}, {}", power, status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamma_rate() {
        let input = include_str!("test.txt");
        assert!(bits_to_int(&gamma_vec(&parse_input(input))) == 22)
    }

    #[test]
    fn test_oxygen_rating() {
        let input = include_str!("test.txt");
        assert!(bits_to_int(&gas_rating(&parse_input(input), false)) == 23)
    }
}
