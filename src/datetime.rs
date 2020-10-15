#[derive(Debug)]
pub struct Time {
    pub hour: usize,
    pub minute: usize,
    pub second: usize,
    pub millisecond: usize,
}

impl Time {
    pub fn new(hour: usize, minute: usize, second: usize, millisecond: usize) -> Self {
        Self {
            hour,
            minute,
            second,
            millisecond,
        }
    }

    pub fn getTime(&self) -> usize {
        (self.hour * 60 * 60 + self.minute * 60 + self.second) * 1000 + self.millisecond
    }
}
