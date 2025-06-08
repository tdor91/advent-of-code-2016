use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/input.txt");

    let lines: Vec<_> = input.split('\n').map(|line| line.trim()).collect();
    let size = lines[0].len();

    let mut columns: Vec<_> = (0..size).map(|_| ColumnCounter::new()).collect();

    for line in lines {
        for (i, c) in line.char_indices() {
            columns[i].add(c);
        }
    }

    let result = columns
        .iter()
        .map(|column| column.max_char_part1())
        .collect::<String>();

    println!("Part 1: {}", result);

    let result = columns
        .iter()
        .map(|column| column.min_char())
        .collect::<String>();

    println!("Part 2: {}", result);
}

#[derive(Debug)]
struct ColumnCounter {
    characters: HashMap<char, u32>,
}

impl ColumnCounter {
    fn new() -> ColumnCounter {
        ColumnCounter {
            characters: HashMap::new(),
        }
    }

    fn add(&mut self, c: char) {
        *self.characters.entry(c).or_insert(0) += 1;
    }

    fn max_char_part1(&self) -> char {
        self.characters
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _)| k.clone())
            .unwrap()
    }

    fn max_char(&self) -> char {
        self.extreme_char(|ord| ord == std::cmp::Ordering::Greater)
    }

    fn min_char(&self) -> char {
        self.extreme_char(|ord| ord == std::cmp::Ordering::Less)
    }

    fn extreme_char<'a>(&self, comp_func: fn(std::cmp::Ordering) -> bool) -> char {
        self.characters
            .iter()
            .reduce(|a, b| if comp_func(a.1.cmp(&b.1)) { a } else { b })
            .map(|(k, _)| k.clone())
            .unwrap()
    }
}
