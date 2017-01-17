extern crate u8rope;

use u8rope::Rope;

fn main() {
	let rope = Rope::from("Hello,... World!");
	println!("{}", rope);

	let rope = rope.delete(6, 3);
	println!("{}", rope);

	let rope = rope.delete(7, 5);
	println!("{}", rope);

	let rope = rope.insert(7, &Rope::from("Universe"));
	println!("{}", rope);

	let (left, right) = rope.split(6);
	println!("\"{}\"/\"{}\"", left, right);
}
