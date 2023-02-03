use super::renderer;

pub struct Sequencer<F: Fn(&str)> {
    render: fn(count: u32) -> String,
    display: F,
}

impl<F> Sequencer<F> 
where F: Fn(&str) {
    pub fn sequence(self: &Self, count: u32) {
        for number in 1..count+1 {
            let content: String = (self.render)(number);
            (self.display)(&content[..]);
        }
    }
}

pub fn get_default_sequencer() -> Sequencer<impl Fn(&str)> {
    Sequencer {
        render: renderer::render,
        display: |content: &str| print!("{}\n", content),
    }
}