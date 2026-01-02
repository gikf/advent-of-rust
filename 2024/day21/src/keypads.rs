#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum Keypad {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
}

impl From<char> for Keypad {
    fn from(value: char) -> Self {
        match value {
            '0' => Keypad::Key0,
            '1' => Keypad::Key1,
            '2' => Keypad::Key2,
            '3' => Keypad::Key3,
            '4' => Keypad::Key4,
            '5' => Keypad::Key5,
            '6' => Keypad::Key6,
            '7' => Keypad::Key7,
            '8' => Keypad::Key8,
            '9' => Keypad::Key9,
            'A' => Keypad::A,
            _ => unimplemented!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum DirectionalKeypad {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl DirectionalKeypad {
    pub const VALUES: [Self; 5] = [Self::Up, Self::Down, Self::Left, Self::Right, Self::A];
}
