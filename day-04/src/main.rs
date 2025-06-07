use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/input.txt");
    let rooms: Vec<_> = input
        .split("\n")
        .map(|line| line.trim())
        .map(|line| RoomDefinition::from(line))
        .collect();

    let part1 = rooms
        .iter()
        .filter(|r| is_valid(&r))
        .map(|r| r.sector_id)
        .sum::<u32>();

    println!("Part 1: {}", part1);
}

#[derive(Debug)]
struct RoomDefinition {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl RoomDefinition {
    fn from(input: &str) -> RoomDefinition {
        // group 1: name
        // group 2: sector id
        // group 3: checksum (incl. braces)
        // group 4: checksum (characters only)
        let regex = Regex::new(r"([a-z-]*)(\d+)(\[([a-z]*)\])").unwrap();

        let captures = regex.captures(input).expect("invalid room definition");

        RoomDefinition {
            name: captures[1].to_string(),
            sector_id: captures[2].parse::<u32>().unwrap(),
            checksum: captures[4].to_string(),
        }
    }
}

fn is_valid(room: &RoomDefinition) -> bool {
    get_top_n_chars(&room.name, 5) == room.checksum
}

fn count_letters(name: &str) -> HashMap<char, u32> {
    let mut map: HashMap<char, u32> = HashMap::new();

    for c in name.chars() {
        if c == '-' {
            continue;
        }

        map.entry(c).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    map
}

fn get_top_n_chars(name: &str, n: usize) -> String {
    let map = count_letters(name);

    let mut sorted: Vec<_> = map.into_iter().collect();
    sorted.sort_by(|a, b| {
        let order = b.1.cmp(&a.1);
        if order.is_ne() {
            order
        } else {
            a.0.cmp(&b.0)
        }
    });

    sorted.iter().map(|(c, _)| c).take(n).collect()
}
