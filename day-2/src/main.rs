use itertools::Itertools;

const RANGE_SEPARATOR: char = ',';
const ID_SEPARATOR: char = '-';

fn main() {
    let input = std::fs::read_to_string("./input/input.txt").expect("Couldn't find input file!");

    let ranges: Vec<(usize, usize)> = input
        .split(RANGE_SEPARATOR)
        .map(|r| {
            r.split(ID_SEPARATOR)
                .map(|id| id.parse::<usize>().unwrap())
                .take(2)
                .collect_tuple::<(usize, usize)>()
                .unwrap()
        })
        .collect();

    let mut invalid_ids = Vec::new();

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
            if is_invalid(id) {
                part_2_invalid_ids.push(id);
            }
        }
    }

    println!("Solution: {}", invalid_ids.iter().sum::<usize>());
}
