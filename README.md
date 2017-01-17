# u8rope (unusable)

This crate may provide a persistent rope based on bytes (u8).

The aim is to explore data structure design in Rust by using `std::rc::Rc`, `std::borrow::Cow`, etc.
Design decisions are aimed towards undo/redo functionality, lazy-loading and ease-of-use.

## Example

```Rust
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
```
