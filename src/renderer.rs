pub fn render(number: u32) -> String {
    return match number {
        3 => "Fizz".to_string(),
        other => other.to_string(),
    }
}