use fizzbuzz::renderer;

#[test]
pub fn test_should_return_one_as_string_for_one() {
    assert_eq!("1", renderer::render(1))
}

#[test]
pub fn test_should_return_two_as_string_for_two() {
    assert_eq!("2", renderer::render(2))
}

#[test]
pub fn test_should_return_fizz_for_three() {
    assert_eq!("Fizz", renderer::render(3))
}