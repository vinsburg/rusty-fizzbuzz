use fizzbuzz::renderer;

#[test]
pub fn test_should_return_one_as_string_for_one() {
    assert!("1" == renderer::render(1))
}

#[test]
pub fn test_should_return_two_as_string_for_two() {
    assert!("2" == renderer::render(2))
}