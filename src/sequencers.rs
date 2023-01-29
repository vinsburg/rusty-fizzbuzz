use super::renderer;

pub struct Sequencer {
    render: fn(count: u32) -> String,
    display: fn(content: &str),
}

fn default_display(content: &str) {
    print!("{}\n", content);
}

impl Sequencer {
    pub fn new() -> Self {
        Self {
            render: renderer::render,
            display: default_display,
        }
    }

    pub fn sequence(self: &Self, count: u32) {
        for number in 1..count+1 {
            let content: String = (self.render)(number);
            (self.display)(&content[..]);
        }
    }
}