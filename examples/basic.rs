extern crate u8rope;

use u8rope::Rope;

fn main() {
	let rope = Rope::from("Hello,... World!");
	println!("{}", rope); // "Hello,... World!"

	let rope = rope.delete(6, 3);
	println!("{}", rope); // "Hello, World!"

	let rope = rope.delete(7, 5);
	println!("{}", rope); // "Hello, !"

	let rope = rope.insert(7, &Rope::from("Universe"));
	println!("{}", rope); // "Hello, Universe!"

	let (left, right) = rope.split(6);
	println!("{}", left); // "Hello,"
	println!("{}", right); // " Universe!"
	
	let rope = rope.append(&Rope::from("!!"));
	println!("{}", rope); // "Hello, Universe!!!"
}
