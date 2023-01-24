use fizzbuzz::renderer;

#[test]
pub fn test_should_return_one_as_string_for_one() {
    assert!("1" == renderer::render(1))
}