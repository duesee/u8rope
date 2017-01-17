# u8rope (unusable)

This crate may provide a persistent rope based on bytes (u8).

The aim is to explore data structure design in Rust by using `std::rc::Rc`, `std::borrow::Cow`, etc.
Design decisions are aimed towards undo/redo functionality, lazy-loading and ease-of-use.

## Example

```Rust
extern crate u8rope;

use u8rope::Rope;

fn main() {
	let rope = Rope::from("Hello,... World!"); // Hello,... World!
	println!("{}", rope);

	let rope = rope.delete(6, 3); // Hello, World!
	println!("{}", rope);

	let rope = rope.delete(7, 5); // Hello, !
	println!("{}", rope);

	let rope = rope.insert(7, &Rope::from("Universe")); // Hello, Universe!
	println!("{}", rope);

	let (left, right) = rope.split(6); // "Hello,"/" Universe!"
	println!("\"{}\"/\"{}\"", left, right);
}
```
