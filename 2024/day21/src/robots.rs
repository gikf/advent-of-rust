use crate::keypads::{DirectionalKeypad, Keypad};

#[derive(Debug, PartialEq)]
pub struct DirectionalRobot {
    pub current_button: DirectionalKeypad,
    pub record: Vec<DirectionalKeypad>,
}

impl DirectionalRobot {
    pub fn new() -> Self {
        DirectionalRobot {
            current_button: DirectionalKeypad::A,
            record: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.record = vec![];
    }

    pub fn press_buttons(&mut self, buttons: &[DirectionalKeypad]) {
        buttons.iter().for_each(|button| self.press(button))
    }

    pub fn press(&mut self, target: &DirectionalKeypad) {
        match self.current_button {
            DirectionalKeypad::Up => match target {
                DirectionalKeypad::Up => {}
                DirectionalKeypad::Down => {
                    self.record.push(DirectionalKeypad::Down);
                }
                DirectionalKeypad::Left => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Left);
                }
                DirectionalKeypad::Right => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Right);
                }
                DirectionalKeypad::A => {
                    self.record.push(DirectionalKeypad::Right);
                }
            },
            DirectionalKeypad::Down => match target {
                DirectionalKeypad::Up => {
                    self.record.push(DirectionalKeypad::Up);
                }
                DirectionalKeypad::Down => {}
                DirectionalKeypad::Left => {
                    self.record.push(DirectionalKeypad::Left);
                }
                DirectionalKeypad::Right => {
                    self.record.push(DirectionalKeypad::Right);
                }
                DirectionalKeypad::A => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Right);
                }
            },
            DirectionalKeypad::Left => match target {
                DirectionalKeypad::Up => {
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Up);
                }
                DirectionalKeypad::Down => {
                    self.record.push(DirectionalKeypad::Right);
                }
                DirectionalKeypad::Left => {}
                DirectionalKeypad::Right => {
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Right);
                }
                DirectionalKeypad::A => {
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Up);
                }
            },
            DirectionalKeypad::Right => match target {
                DirectionalKeypad::Up => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Up);
                }
                DirectionalKeypad::Down => {
                    self.record.push(DirectionalKeypad::Left);
                }
                DirectionalKeypad::Left => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Left);
                }
                DirectionalKeypad::Right => {}
                DirectionalKeypad::A => {
                    self.record.push(DirectionalKeypad::Up);
                }
            },
            DirectionalKeypad::A => match target {
                DirectionalKeypad::Up => {
                    self.record.push(DirectionalKeypad::Left);
                }
                DirectionalKeypad::Down => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Down);
                }
                DirectionalKeypad::Left => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Left);
                }
                DirectionalKeypad::Right => {
                    self.record.push(DirectionalKeypad::Down);
                }
                DirectionalKeypad::A => {}
            },
        }
        self.record.push(DirectionalKeypad::A);
        self.current_button = *target;
    }
}

#[derive(Debug, PartialEq)]
pub struct KeypadRobot {
    pub current_button: Keypad,
    pub record: Vec<DirectionalKeypad>,
}

impl KeypadRobot {
    pub fn new() -> Self {
        KeypadRobot {
            current_button: Keypad::A,
            record: vec![],
        }
    }

    pub fn press_buttons(&mut self, buttons: &[Keypad]) {
        buttons.iter().for_each(|button| self.press(button));
    }

    pub fn press(&mut self, target: &Keypad) {
        match self.current_button {
            Keypad::Key0 => match target {
                Keypad::Key0 => {}
                Keypad::Key1 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key2 => {
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key3 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key4 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key5 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key6 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key7 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key8 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key9 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::A => {
                    self.record.push(DirectionalKeypad::Right);
                }
            },
            Keypad::Key1 => match target {
                Keypad::Key0 => {
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key1 => {}
                Keypad::Key2 => {
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key3 => {
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key4 => {
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key5 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key6 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key7 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key8 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key9 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::A => {
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Down);
                }
            },
            Keypad::Key2 => match target {
                Keypad::Key0 => {
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key1 => {
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key2 => {}
                Keypad::Key3 => {
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key4 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key5 => {
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key6 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key7 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key8 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key9 => {
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::A => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Right);
                }
            },
            Keypad::Key3 => match target {
                Keypad::Key0 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key1 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key2 => {
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key3 => {}
                Keypad::Key4 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key5 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key6 => {
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key7 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key8 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key9 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::A => {
                    self.record.push(DirectionalKeypad::Down);
                }
            },
            Keypad::Key4 => match target {
                Keypad::Key0 => {
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key1 => {
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key2 => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key3 => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key4 => {}
                Keypad::Key5 => {
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key6 => {
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key7 => {
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key8 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key9 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::A => {
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                }
            },
            Keypad::Key5 => match target {
                Keypad::Key0 => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key1 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key2 => {
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key3 => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key4 => {
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key5 => {}
                Keypad::Key6 => {
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key7 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key8 => {
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key9 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::A => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Right);
                }
            },
            Keypad::Key6 => match target {
                Keypad::Key0 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key1 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key2 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key3 => {
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key4 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key5 => {
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key6 => {}
                Keypad::Key7 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key8 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key9 => {
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::A => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                }
            },
            Keypad::Key7 => match target {
                Keypad::Key0 => {
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key1 => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key2 => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key3 => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key4 => {
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key5 => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key6 => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key7 => {}
                Keypad::Key8 => {
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key9 => {
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::A => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Right);
                    self.record.push(DirectionalKeypad::Right);
                }
            },
            Keypad::Key8 => match target {
                Keypad::Key0 => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key1 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key2 => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key3 => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key4 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key5 => {
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key6 => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::Key7 => {
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key8 => {}
                Keypad::Key9 => {
                    self.record.push(DirectionalKeypad::Right);
                }
                Keypad::A => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Right);
                }
            },
            Keypad::Key9 => match target {
                Keypad::Key0 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key1 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key2 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key3 => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key4 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key5 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key6 => {
                    self.record.push(DirectionalKeypad::Down);
                }
                Keypad::Key7 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key8 => {
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key9 => {}
                Keypad::A => {
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                    self.record.push(DirectionalKeypad::Down);
                }
            },
            Keypad::A => match target {
                Keypad::Key0 => {
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key1 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key2 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key3 => {
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key4 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key5 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key6 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key7 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Left);
                }
                Keypad::Key8 => {
                    self.record.push(DirectionalKeypad::Left);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::Key9 => {
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                    self.record.push(DirectionalKeypad::Up);
                }
                Keypad::A => {}
            },
        }
        self.record.push(DirectionalKeypad::A);
        self.current_button = *target;
    }
}
