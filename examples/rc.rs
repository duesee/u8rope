//!
//! Learning to use std::rc::Rc correctly...
//!
//! See https://doc.rust-lang.org/std/rc/
///

use std::rc::Rc;

fn test1() {
    struct Sheep;

    impl Drop for Sheep {
        fn drop(&mut self) {
            println!("Dropping now!");
        }
    }

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

    println!("Container goes out of scope!");
}

fn test2() {
    struct Owner {
        bikes: Vec<Rc<Bike>>,
        name: String,
    }

    struct Bike;

    impl Owner {
        fn new(name: &str) -> Self {
            Owner {
                bikes: vec![],
                name: name.into(),
            }
        }

        fn add_bike(&mut self, bike: Rc<Bike>) {
            self.bikes.push(bike);
        }
    }

    impl Drop for Owner {
        fn drop(&mut self) {
            println!("Dropping owner {}!", self.name);
        }
    }

    impl Bike {
        fn look_at_it(&self) {
            println!("...");
        }
    }

    impl Drop for Bike {
        fn drop(&mut self) {
            println!("Dropping bike!");
        }
    }

    let bike = Rc::new(Bike);

    let mut owner1 = Owner::new("Damian");
    let mut owner2 = Owner::new("Daniel");

    owner1.add_bike(bike.clone());
    owner2.add_bike(bike.clone());

    owner1.bikes[0].look_at_it();
    owner2.bikes[0].look_at_it();
}

fn main() {
    let tests: Vec<fn()> = vec![test1, test2];

    for (no, test) in tests.iter().enumerate() {
        println!("Test #{}", no + 1);
        test();
        println!();
    }
}