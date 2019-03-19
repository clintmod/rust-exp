
pub fn concat(a:&str, b:&str) -> String {
	return format!("{}{}", a, b);
}

#[test]
fn test_concat() {
	assert_eq!(concat("asdf", "asdf2"), "asdfasdf2");
}