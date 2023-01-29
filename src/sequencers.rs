use super::renderer;

pub struct Sequencer {
    render: fn(count: u32) -> String,
    display: fn(content: &str),
}

fn default_display(content: &str) {
    print!("{}", content);
}

impl Sequencer {
    pub fn new() -> Self {
        Self {
            render: renderer::render,
            display: default_display,
        }
    }

    pub fn sequence(self: &Self, count: u32) {
        let content: String = (self.render)(count);
        (self.display)(&content[..])
    }
}