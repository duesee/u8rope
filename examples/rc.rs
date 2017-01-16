//!
//! Learning to use std::rc::Rc correctly...
//!
//! See https://doc.rust-lang.org/std/rc/
///

use std::borrow::Borrow;
use std::rc::Rc;

struct Sheep;

impl Drop for Sheep {
    fn drop(&mut self) {
        println!("Dropping now!");
    }
}

fn main() {
    let mut container = {
        let rc = Rc::new(Sheep);

        vec![
            rc.clone(),
            rc.clone(),
            rc.clone(),
        ]
    };

    println!("Wait for it...");
    container.pop();

    println!("... wait ...");
    container.pop();

    println!("... wait ...");
    container.pop();

    println!("container goes out of scope");
}