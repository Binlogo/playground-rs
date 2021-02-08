static feedback: f64 = 0.5;
pub struct AllPass {
    delay_line: DelayLine,
}

impl AllPass {
    pub fn new(delay_length: usize) -> Self {
        Self {
            delay_line: DelayLine::new(delay_length),
        }
    }

    pub fn tick(&mut self, input: f64) -> f64 {
        let delayed = self.delay_line.read();
        let output = -input + delayed;
        self.delay_line
            .write_and_advance(input + delayed * feedback);
        output
    }
}
