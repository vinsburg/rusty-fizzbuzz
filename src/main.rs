use fizzbuzz::sequencers;

fn main() {
    let mut sequencer = sequencers::get_default_sequencer();
    sequencer.sequence(100);
}
