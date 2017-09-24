pub struct Keyboard {
    pub keyboard: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keyboard: [false; 16]
        }
    }

    pub fn set(&mut self, key: u8) {
        self.keyboard[key as usize] = true;
    }

    pub fn unset(&mut self, key: u8) {
        self.keyboard[key as usize] = false;
    }

    pub fn pressed(&self, key: u8) -> bool {
        self.keyboard[key as usize]
    }
}