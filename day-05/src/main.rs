fn main() {
    let input = include_str!("../input/input.txt");

    println!("Part 1: {}", brute_force_part1(&input));
    println!("Part 2: {}", brute_force_part2(&input));
}

fn brute_force_part1(input: &str) -> String {
    let mut result = Vec::<char>::new();
    let mut index = 0 as u32;

    while result.len() < 8 {
        let hash = hash_hex(input, index);

        if hash.starts_with("00000") {
            result.push(hash.chars().nth(5).unwrap());
        }

        if result.len() >= 8 {
            break;
        }

        index += 1;
    }

    result.into_iter().collect::<String>()
}

fn brute_force_part2(input: &str) -> String {
    let mut result = [None; 8];
    let mut index = 0 as u32;

    while result.iter().any(|v| v.is_none()) {
        let hash = hash_hex(input, index);

        if hash.starts_with("00000") {
            if let Some((pos, c)) = extract_pos_and_char(&hash) {
                if pos < result.len() && result[pos].is_none() {
                    result[pos] = Some(c);
                }
            }
        }

        index += 1;
    }

    result
        .into_iter()
        .map(|v| v.expect("should have value"))
        .collect::<String>()
}

fn hash_hex(input: &str, index: u32) -> String {
    let data = format!("{}{}", input, index);
    let digest = md5::compute(&data);
    format!("{:x}", digest)
}

fn extract_pos_and_char(hash: &str) -> Option<(usize, char)> {
    let pos = hash.chars().nth(5)?.to_digit(10)? as usize;
    let c = hash.chars().nth(6)?;
    Some((pos, c))
}
