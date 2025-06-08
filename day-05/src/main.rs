fn main() {
    let input = include_str!("../input/input.txt");

    println!("Part 1: {}", brute_force_part1(&input));
    println!("Part 2: {}", brute_force_part2(&input));
}

fn brute_force_part1(input: &str) -> String {
    let mut result = Vec::new();

    for index in 0.. {
        let hash = hash_hex(input, index);

        if hash.starts_with("00000") {
            result.push(hash.chars().nth(5).unwrap());

            if result.len() >= 7 {
                // all the characters found
                break;
            }
        }
    }

    result.into_iter().collect()
}

fn brute_force_part2(input: &str) -> String {
    let mut result = [None; 8];

    // Use an infinite loop instead of a while with condition:
    // We can reduce the number of times we have to check the condition by checking
    // inside the loop only when we found a new character. This should improve performance.
    for index in 0.. {
        let hash = hash_hex(input, index);

        if hash.starts_with("00000") {
            if let Some((pos, c)) = extract_pos_and_char(&hash) {
                if pos < result.len() && result[pos].is_none() {
                    result[pos] = Some(c);

                    if !result.iter().any(|v| v.is_none()) {
                        // all the characters found
                        break;
                    }
                }
            }
        }
    }

    result
        .into_iter()
        .map(|v| v.expect("should have value"))
        .collect()
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
