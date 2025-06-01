pub trait Keypad {
    fn get_button(&self) -> char;
    fn process_char(&mut self, c: &char);

    fn process_line(&mut self, line: &str) -> char {
        for c in line.chars() {
            self.process_char(&c);
        }

        self.get_button()
    }
}

pub struct SimpleKeypad {
    pub position: (usize, usize),
}

pub struct ComplexKeypad {
    pub position: (usize, usize),
    button_map: Vec<Vec<Option<char>>>,
}

impl SimpleKeypad {
    pub fn new() -> SimpleKeypad {
        SimpleKeypad { position: (1, 1) }
    }

    fn try_inc(value: usize) -> usize {
        if value >= 2 {
            return 2;
        }

        value + 1
    }
}

impl Keypad for SimpleKeypad {
    fn process_char(&mut self, c: &char) {
        self.position = match c {
            'U' => (self.position.0, try_dec(self.position.1)),
            'D' => (self.position.0, SimpleKeypad::try_inc(self.position.1)),
            'L' => (try_dec(self.position.0), self.position.1),
            'R' => (SimpleKeypad::try_inc(self.position.0), self.position.1),
            _ => panic!("invalid direction"),
        };
    }

    fn get_button(&self) -> char {
        let value = 1 + self.position.0 + self.position.1 * 3;
        char::from_digit(value as u32, 10).unwrap()
    }
}

impl ComplexKeypad {
    pub fn new() -> ComplexKeypad {
        let position = (0, 2);

        //     1
        //   2 3 4
        // 5 6 7 8 9
        //   A B C
        //     D
        let button_map = vec![
            vec![None, None, Some('1'), None, None],
            vec![None, Some('2'), Some('3'), Some('4'), None],
            vec![Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
            vec![None, Some('A'), Some('B'), Some('C'), None],
            vec![None, None, Some('D'), None, None],
        ];

        ComplexKeypad { position, button_map }
    }

    fn get_button(&self, pos: &(usize, usize)) -> Option<char> {
        self.button_map.get(pos.1)?.get(pos.0)?.as_ref().copied()
    }
}

impl Keypad for ComplexKeypad {
    fn process_char(&mut self, c: &char) {
        let pos = match c {
            'U' => (self.position.0, try_dec(self.position.1)),
            'D' => (self.position.0, self.position.1 + 1),
            'L' => (try_dec(self.position.0), self.position.1),
            'R' => (self.position.0 + 1, self.position.1),
            _ => panic!("invalid direction"),
        };

        if let Some(_) = self.get_button(&pos) {
            self.position = pos;
        }
    }

    fn get_button(&self) -> char {
        self.button_map[self.position.1][self.position.0]
            .expect("invalid position in keypad. this should not happen")
    }
}

fn try_dec(value: usize) -> usize {
    if value <= 0 {
        return 0;
    }

    value - 1
}
