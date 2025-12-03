const MAX_BATTERIES: u32 = 2;

pub fn parse_input(path: &str) -> Vec<Vec<char>> {
    let input = std::fs::read_to_string(path).unwrap();

    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

pub fn find_maximum_joltage(bank: &[char]) -> u32 {
    let mut joltages = Vec::new();

    let mut first_index = 0;
    let mut second_index = first_index + 1;

    loop {
        if first_index == second_index {
            first_index += 1;
            second_index = first_index + 1;
            continue;
        }

        if let Some(first) = bank.get(first_index) {
            if let Some(second) = bank.get(second_index) {
                joltages.push(format!("{}{}", first, second).parse::<u32>().unwrap());
                second_index += 1;
            } else {
                first_index += 1;
                second_index = first_index + 1;
            }
        } else {
            break;
        }
    }

    *joltages.iter().max().unwrap()
}

fn main() {
    let battery_banks = parse_input("./input/example.txt");

    let max_joltages: Vec<u32> = battery_banks
        .iter()
        .map(|bank| find_maximum_joltage(bank))
        .collect();

    for joltage in &max_joltages {
        println!("{}", joltage);
    }
    println!("Part 1 Solution: {}", max_joltages.iter().sum::<u32>());
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
