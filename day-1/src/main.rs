const MIN_DIGIT: u32 = 0;
const MAX_DIGIT: u32 = 99;
const INITIAL_DIGIT: u32 = 50;
const TARGET_DIGIT: u32 = 0;

#[derive(Default)]
pub struct Dial {
    current_digit: u32,
    min_digit: u32,
    max_digit: u32,
    zero_counter: u32,
}

impl Dial {
    pub fn new() -> Self {
        Dial {
            current_digit: INITIAL_DIGIT,
            min_digit: MIN_DIGIT,
            max_digit: MAX_DIGIT,
            zero_counter: 0,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
}

pub fn click(direction: Direction, dial: &mut Dial, num_steps: u32) {
    let mut remaining_steps = num_steps;

    println!(
        "Dial starting at: {}\nMoving steps: {}\nDirection: {:?}",
        dial.current_digit, num_steps, direction
    );

    for _i in 0..num_steps {
        if direction == Direction::Left {
            if dial.current_digit == dial.min_digit {
                dial.current_digit = dial.max_digit;
            } else {
                dial.current_digit -= 1;
            }
        } else if dial.current_digit == dial.max_digit {
            dial.current_digit = dial.min_digit
        } else {
            dial.current_digit += 1;
        }

        remaining_steps -= 1;

        println!(
            "Click... {}... {} steps left",
            dial.current_digit, remaining_steps
        );
    }
    if dial.current_digit == TARGET_DIGIT {
        println!(
            "Hit target digit! Incrementing counter to {}",
            dial.zero_counter + 1
        );
        dial.zero_counter += 1;
    }
}

fn main() {
    let input =
        std::fs::read_to_string("./input/input.txt").expect("Incorrect path to input file!");

    let mut dial = Dial::new();

    for line in input.lines() {
        println!("{}", line);

        let (dir, steps) = line.split_at(1);
        let direction = match dir {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Direction char {} was neither L nor R", dir),
        };

        let num_steps = steps
            .parse::<u32>()
            .expect("Couldn't parse steps into number!");

        click(direction, &mut dial, num_steps);
    }

    println!("Number of times hit zero: {}", dial.zero_counter);
}
