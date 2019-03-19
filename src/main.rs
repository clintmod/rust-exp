mod strings;

fn main() {
	let str = strings::concat("Hello", " World");
	println!("{:?}", str);
}
