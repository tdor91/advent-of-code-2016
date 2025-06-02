fn main() {
    let input = include_str!("../input/input.txt");
    let lines: Vec<_> = input.split("\n").map(|line| line.trim()).collect();

    let triangles: Vec<_> = lines.iter().map(|line| Triangle::from(line)).collect();

    println!(
        "Part 1: {}",
        triangles.iter().filter(|t| t.is_valid()).count()
    );

    let triangles: Vec<_> = lines
        .chunks(3)
        .map(|chunk| Triangle::from_group(chunk))
        .flatten()
        .collect();

    println!(
        "Part 2: {}",
        triangles.iter().filter(|t| t.is_valid()).count()
    );
}

#[derive(Debug)]
struct Triangle(u32, u32, u32);

impl Triangle {
    fn from(definition: &str) -> Triangle {
        let parts = Triangle::parse_line(definition);
        Triangle(parts[0], parts[1], parts[2])
    }

    fn from_group(definitions: &[&str]) -> Vec<Triangle> {
        assert_eq!(definitions.len(), 3, "invalid group definition");

        let parts: Vec<[u32; 3]> = definitions
            .iter()
            .map(|line| Triangle::parse_line(line))
            .collect();

        (0..=2)
            .map(|i| Triangle(parts[0][i], parts[1][i], parts[2][i]))
            .collect()
    }

    fn is_valid(&self) -> bool {
        self.0 + self.1 > self.2 && self.1 + self.2 > self.0 && self.0 + self.2 > self.1
    }

    fn parse_line(line: &str) -> [u32; 3] {
        let parts: Vec<_> = line
            .split(" ")
            .filter(|p| !p.is_empty())
            .map(|p| p.parse::<u32>().expect("expected a number"))
            .collect();

        assert_eq!(parts.len(), 3, "invalid definition");

        [parts[0], parts[1], parts[2]]
    }
}
