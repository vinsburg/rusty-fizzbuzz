pub mod renderer;
pub mod sequencers;

pub fn default_sequencer() -> sequencers::Sequencer<impl FnMut(&str)> {
    sequencers::Sequencer {
        render: renderer::render,
        display: |content: &str| print!("{}\n", content),
    }
}