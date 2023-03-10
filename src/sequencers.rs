pub struct Sequencer<F: FnMut(&str)> {
    pub render: fn(count: usize) -> String,
    pub display: F,
}

impl<F> Sequencer<F> 
where F: FnMut(&str) {
    pub fn sequence(self: &mut Self, count: usize) {
        for number in 1..count+1 {
            let content: String = (self.render)(number);
            (self.display)(&content[..]);
        }
    }
}