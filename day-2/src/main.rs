use fancy_regex::Regex;
use itertools::Itertools;

const RANGE_SEPARATOR: char = ',';
const ID_SEPARATOR: char = '-';

pub fn parse_input(input: String) -> Vec<(usize, usize)> {
    input
        .split(RANGE_SEPARATOR)
        .map(|r| {
            r.split(ID_SEPARATOR)
                .map(|id| id.parse::<usize>().unwrap())
                .take(2)
                .collect_tuple::<(usize, usize)>()
                .unwrap()
        })
        .collect()
}

pub fn is_valid(id: usize) -> bool {
    let id_str = id.to_string();

    for i in 1..id_str.len() {
        let pattern = id_str
            .split_at_checked(i)
            .expect("Invalid index for split")
            .0;

        let split = id_str.split(pattern).collect::<Vec<&str>>();

        let pattern_count = split.len();

        if split.iter().all(|s| s.is_empty()) && pattern_count >= 2 {
            println!(
                "ID {} INVALID! Pattern {} repeated {} times",
                id,
                pattern,
                pattern_count - 1
            );
            return false;
        }
    }
    println!("ID {} VALID", id);
    true
}

pub fn is_valid_regex(id: usize) -> bool {
    let re = Regex::new(r"^(\d+)\1+$").unwrap();
    re.is_match(&id.to_string()).unwrap()
}

fn main() {
    let input = std::fs::read_to_string("./input/example.txt").expect("Couldn't find input file!");

    let ranges: Vec<(usize, usize)> = parse_input(input);

    let mut part_1_invalid_ids = Vec::new();
    let mut part_2_invalid_ids = Vec::new();

    for range in ranges {
        println!("Range: {}-{}", range.0, range.1);
        for id in range.0..=range.1 {
            // Part 1 Solution
            let id_string = id.to_string();
            let (first, last) = id_string.split_at(id_string.len() / 2);

            if first.starts_with('0') || last.starts_with('0') {
                continue;
            }

            if first == last {
                part_1_invalid_ids.push(id);
            }

            // Part 2 Solution
            if !is_valid_regex(id) {
                part_2_invalid_ids.push(id);
            }
        }
    }

    println!(
        "Part 1 Solution: {}",
        part_1_invalid_ids.iter().sum::<usize>()
    );
    println!(
        "Part 2 Solution: {}",
        part_2_invalid_ids.iter().sum::<usize>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_valid_id_returns_true() {
        let valid_id = 101;
        assert!(is_valid(valid_id));
    }

    #[test]
    fn is_valid_invalid_id_returns_false() {
        let invalid_id = 1188511885;
        assert!(!is_valid(invalid_id));
    }

    #[test]
    fn is_valid_other_invalid_id_returns_false() {
        let invalid_id = 111;
        assert!(!is_valid(invalid_id));
    }

    #[test]
    fn test_part2_example_input() {
        let input =
            std::fs::read_to_string("./input/example.txt").expect("Couldn't find input file!");

        let ranges: Vec<(usize, usize)> = parse_input(input);
        let mut sum: usize = 0;

        for range in ranges {
            for id in range.0..=range.1 {
                if !is_valid(id) {
                    sum += id;
                }
            }
        }

        assert_eq!(sum, 4174379265);
    }
}
