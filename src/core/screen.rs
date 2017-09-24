pub struct Screen {
    pub screen: [[u8; 32]; 64]
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            screen: [[0; 32]; 64]
        }
    }
}