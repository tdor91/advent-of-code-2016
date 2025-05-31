#[derive(Debug)]
pub struct Action {
    turn: Turn,
    distance: i32,
}

impl Action {
    pub fn new(s: &str) -> Action {
        let (a, b) = s.split_at(1);

        let turn = a
            .chars()
            .nth(0)
            .map(|c| match c {
                'R' => Turn::Right,
                'L' => Turn::Left,
                _ => panic!("invalid 'turn'. expected values are 'R' and 'L'"),
            })
            .unwrap();

        let distance = b.trim().parse::<i32>().expect("distance is not a number");

        Action { turn, distance }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    fn from_value(val: usize) -> Self {
        match val {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("invalid value; expected a value from 0 to 3"),
        }
    }

    pub fn turn(self, turn: &Turn) -> Direction {
        let next_val = match turn {
            Turn::Right => (self as usize + 1) % Self::ALL.len(),
            Turn::Left => (self as usize + Self::ALL.len() - 1) % Self::ALL.len(),
        };

        Direction::from_value(next_val)
    }

    pub fn as_step(&self) -> Point {
        match self {
            Direction::Up => Point::new(0, 1),
            Direction::Down => Point::new(0, -1),
            Direction::Right => Point::new(1, 0),
            Direction::Left => Point::new(-1, 0),
        }
    }
}

#[derive(Debug)]
pub enum Turn {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub direction: Direction,
    pub point: Point,
}

impl Position {
    pub fn new() -> Position {
        Position {
            direction: Direction::Up,
            point: Point { x: 0, y: 0 },
        }
    }

    pub fn walk(&self, action: &Action) -> Position {
        let direction = self.direction.turn(&action.turn);

        let gap = direction.as_step().mult(action.distance);
        let point = self.point.add(&gap);

        Position { direction, point }
    }

    pub fn trace_walk(&self, action: &Action) -> (Position, Vec<Point>) {
        let mut temp_point = self.point.clone();
        let mut points: Vec<Point> = vec![];
        
        let direction = self.direction.turn(&action.turn);
        let step = direction.as_step();

        for _ in 1..=action.distance {
            temp_point = temp_point.add(&step);
            points.push(temp_point);
        }

        let point = self.point.add(&step.mult(action.distance));

        (Position { direction, point }, points)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn add(&self, d: &Point) -> Point {
        Point {
            x: self.x + d.x,
            y: self.y + d.y,
        }
    }

    pub fn mult(&self, f: i32) -> Point {
        Point {
            x: self.x * f,
            y: self.y * f,
        }
    }

    pub fn distance_to_origin(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}
