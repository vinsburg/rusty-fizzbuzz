use fizzbuzz::sequencers;

fn main() {
    let sequencer = sequencers::get_default_sequencer();
    sequencer.sequence(100);
}
