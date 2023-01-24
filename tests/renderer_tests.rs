use fizzbuzz::renderer;

#[test]
pub fn test_one_is_rendered_as_string_one() {
    assert_eq!("1", renderer::render(1))
}

#[test]
pub fn test_two_is_rendered_as_string_two() {
    assert_eq!("2", renderer::render(2))
}

#[test]
pub fn test_three_is_rendered_as_fizz() {
    assert_eq!("Fizz", renderer::render(3))
}

#[test]
pub fn test_six_is_rendered_as_fizz() {
    assert_eq!("Fizz", renderer::render(6))
}