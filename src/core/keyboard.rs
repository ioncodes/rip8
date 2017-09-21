pub struct Keyboard {
    pub keyboard: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keyboard: [false; 16]
        }
    }

    pub fn set(&mut self, key: String) {
        let index = usize::from_str_radix(&key, 16).unwrap();
        self.keyboard[index] = true;
    }

    pub fn unset(&mut self, key: String) {
        let index = usize::from_str_radix(&key, 16).unwrap();
        self.keyboard[index] = false;
    }
}