use super::renderer;

pub struct Sequencer<F: FnMut(&str)> {
    pub render: fn(count: u32) -> String,
    pub display: F,
}

impl<F> Sequencer<F> 
where F: FnMut(&str) {
    pub fn sequence(self: &mut Self, count: u32) {
        for number in 1..count+1 {
            let content: String = (self.render)(number);
            (self.display)(&content[..]);
        }
    }
}

pub fn get_default_sequencer() -> Sequencer<impl FnMut(&str)> {
    Sequencer {
        render: renderer::render,
        display: |content: &str| print!("{}\n", content),
    }
}