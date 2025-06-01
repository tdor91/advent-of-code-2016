use day_02::{ComplexKeypad, Keypad, SimpleKeypad};

fn main() {
    let input = include_str!("../input/input.txt");

    let lines: Vec<_> = input.split("\n").map(|x| x.trim()).collect();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn part1(lines: &Vec<&str>) -> String {
    let mut buttons: Vec<char> = vec![];
    let mut keypad = SimpleKeypad::new();

    for line in lines {
        buttons.push(keypad.process_line(line));
    }

    buttons
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>()
}

fn part2(lines: &Vec<&str>) -> String {
    let mut buttons: Vec<char> = vec![];
    let mut keypad = ComplexKeypad::new();

    for line in lines {
        buttons.push(keypad.process_line(line));
    }

    buttons
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>()
}
