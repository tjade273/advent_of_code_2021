fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| l.chars().map(|b| b.to_digit(2).unwrap() as usize).collect())
        .collect()
}

fn column_majority(mat: &Vec<Vec<usize>>, col: usize) -> usize {
    (2 * mat.iter().map(|r| r[col]).sum::<usize>() >= mat.len()) as usize
}

fn transpose<T: Copy>(mat: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!mat.is_empty() && !mat[0].is_empty());
    assert!(mat.iter().all(|r| r.len() == mat[0].len()));

    let nrows = mat.len();
    let ncols = mat[0].len();
    (0..ncols).map(|c| (0..nrows).map(|r| mat[r][c]).collect()).collect()
}



fn gamma_vec(bits: &Vec<Vec<usize>>) -> Vec<usize> {
    (0..bits[0].len()).map(|col| column_majority(bits, col)).collect()
}

fn bits_to_int(bits: &Vec<usize>) -> usize {
    bits.iter().fold(0, |acc, b| (acc << 1) + b)
}
/*
fn oxygen_rating(_bits: &Vec<Vec<usize>>) -> Vec<usize> {
    let bits: Vec<&Vec<usize>>= _bits.iter().collect();
    let ncols = bits[0].len(); 
    for i in 0..ncols {
        if bits.len() == 1 {
            break
        }
        let gamma = gamma_vec(&bits)[i];
        bits = bits.iter().filter(|r| r[i] == gamma).collect();
    }
    return bits[0];
}
*/
fn main() {
    let input = include_str!("input.txt");
    let input_bits = parse_input(input);
    let gamma_vec = gamma_vec(&input_bits);
    let epsilon_vec = gamma_vec.iter().map(|b| (b + 1) & 1).collect();
    let power = bits_to_int(&gamma_vec) * bits_to_int(&epsilon_vec);
    println!("{}", power)
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
    fn test_transpose() {
        let v = vec![vec![0,0,0], vec![1,1,1]];
        assert!(transpose(&v) == vec![vec![0,1], vec![0,1], vec![0,1]])
    }
}
