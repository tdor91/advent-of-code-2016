use std::collections::HashSet;
use day_01::*;

fn main() {
    let input = include_str!("../input/input.txt");

    let actions: Vec<_> = input.split(", ").map(|x| Action::new(x)).collect();

    println!("Part 1: {}", part1(&actions));
    println!("Part 2: {}", part2(&actions));
}

fn part1(actions: &Vec<Action>) -> u32 {
    let mut position = Position::new();

    for action in actions {
        position = position.walk(&action);
    }

    position.point.distance_to_origin()
}

fn part2(actions: &Vec<Action>) -> u32 {
    let mut position = Position::new();

    let mut visited: HashSet<Point> = HashSet::from([(Point { x: 0, y: 0 })]);

    for action in actions {
        let (new_position, path) = position.trace_walk(&action);

        position = new_position;

        for point in path {
            if visited.contains(&point) {
                return point.distance_to_origin();
            }

            visited.insert(point);
        }
    }

    panic!("no result found");
}

