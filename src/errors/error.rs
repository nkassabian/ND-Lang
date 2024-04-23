pub struct Error {
    position: usize,
    line: usize,
    message: String,
    tip: String,
}

impl Error {
    pub fn new(position: usize, line: usize, message: String, tip: String) -> Self {
        Self {
            position,
            line,
            message,
            tip,
        }
    }

    pub fn report(&self) {
        println!("[{}::{}] {}", self.line, self.position, self.message);
    }
}
