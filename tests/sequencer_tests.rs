// use fizzbuzz::sequencer;
struct Screen {
    displayed: Vec<String>,
}

impl Screen {
    fn new() -> Self {
        Self {displayed: vec![]}
    }

    fn display(self: &mut Self, content: &str) {
        self.displayed.push(content.to_string())
    }

    fn render(self: &Self, count: i32) -> String {
        format!("{}{}", "x", count.to_string())
    }
}

#[test]
pub fn test_screen_renders_and_displays_one_as_x1() {
    let mut screen: Screen = Screen::new();
    let content: String = screen.render(1);
    screen.display(&content);
    assert_eq!("x1", screen.displayed[0])
}

// next implement a sequencer and instantiate it with the display and render methods