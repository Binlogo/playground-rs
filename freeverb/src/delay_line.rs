pub struct DelayLine {
    buffer: Vec<f64>,
    index: usize,
}

impl DelayLine {
    pub fn new(length: usize) -> Self {
        Self {
            buffer: vec![0.0; length],
            index: 0,
        }
    }

    pub fn read(&self) -> f64 {
        self.buffer[self.index]
    }

    pub fn write_and_advance(&mut self, value: f64) {
        self.buffer[self.index] = value;

        if self.index + 1 >= self.buffer.len() {
            self.index = 0
        } else {
            self.index += 1
        }
    }
}

#[test]
fn length_10() {
    let mut line = DelayLine::new(10);
    for i in 0..10 {
        assert_eq!(line.read(), 0);
        line.write_and_advance(i);
    }
    for i in 0..10 {
        assert_eq!(line.read(), i);
    }
}
