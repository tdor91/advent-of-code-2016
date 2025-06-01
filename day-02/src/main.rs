use day_02::{ComplexKeypad, Keypad, SimpleKeypad};

fn main() {
    let input = include_str!("../input/input.txt");
    let lines: Vec<_> = input.split("\n").map(|x| x.trim()).collect();

    println!("Part 1: {}", solve(&lines, SimpleKeypad::new()));
    println!("Part 2: {}", solve(&lines, ComplexKeypad::new()));
}

fn solve<T>(lines: &Vec<&str>, mut keypad: T) -> String
where
    T: Keypad,
{
    let mut buttons: Vec<char> = vec![];

    for line in lines {
        buttons.push(keypad.process_line(line));
    }

    buttons
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>()
}
