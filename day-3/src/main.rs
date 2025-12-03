const MAX_BATTERIES: u32 = 12;

pub fn parse_input(path: &str) -> Vec<Vec<char>> {
    let input = std::fs::read_to_string(path).unwrap();

    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

pub fn find_maximum_joltage(bank: &[char]) -> usize {
    let mut best_digits: Vec<usize> = Vec::new();

    let digits: Vec<usize> = bank
        .iter()
        .map(|char| char.to_digit(10).unwrap() as usize)
        .collect();

    for digit in digits.iter().enumerate() {
        if best_digits.len() == MAX_BATTERIES as usize {
            break;
        }
        if digits
            .iter()
            .skip(digit.0 + 1)
            .filter(|d| **d > *digit.1)
            .count()
            < (MAX_BATTERIES as usize - best_digits.len())
        {
            best_digits.push(*digit.1);
        }
    }

    assert_eq!(best_digits.len(), MAX_BATTERIES as usize);

    best_digits.iter().fold(0, |acc, digit| acc * 10 + digit)
}

fn main() {
    let battery_banks = parse_input("./input/example.txt");

    let max_joltages: Vec<usize> = battery_banks
        .iter()
        .map(|bank| find_maximum_joltage(bank))
        .collect();

    for joltage in &max_joltages {
        println!("{}", joltage);
    }
    println!("Part 1 Solution: {}", max_joltages.iter().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_3_returns_78() {
        let input = "234234234234278".chars().collect::<Vec<char>>();

        assert_eq!(find_maximum_joltage(&input), 78);
    }
}
