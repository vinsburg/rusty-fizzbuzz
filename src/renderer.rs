pub fn render(number: u32) -> String {
    let mut rendered: String = "".to_string();
    if (number % 3) != 0 {
        rendered = "Fizz".to_string();
    }
    if rendered == "" {
        rendered = number.to_string();
    }
    rendered
}