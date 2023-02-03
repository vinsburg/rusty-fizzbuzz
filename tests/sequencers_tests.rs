use fizzbuzz::sequencers::Sequencer;
struct Screen {
    displayed: Vec<String>,
}

impl Screen {
    fn new() -> Self {
        Self {displayed: vec![]}
    }

    pub fn display(self: &mut Self, content: &str) {
        self.displayed.push(content.to_string())
    }
}

fn render(count: u32) -> String {
    format!("{}{}", "x", count.to_string())
}

#[test]
pub fn test_screen_renders_one_as_x1() {
    assert_eq!("x1", render(1))
}

#[test]
pub fn test_screen_displays_x1() {
    let mut screen: Screen = Screen::new();
    screen.display("x1");
    assert_eq!("x1", screen.displayed[0])
}

#[test]
pub fn test_should_display_two_numbers() {
    let mut screen: Screen = Screen::new();
    let display = |content: &str| screen.display(content);
    let mut sequencer = Sequencer{render, display};
    sequencer.sequence(2);
    assert_eq!(2, screen.displayed.len())
}

#[test]
pub fn test_should_render_one_as_x1() {
    let mut screen: Screen = Screen::new();
    let display = |content: &str| screen.display(content);
    let mut sequencer = Sequencer{render, display};
    sequencer.sequence(1);
    assert_eq!("x1", screen.displayed[0])
}