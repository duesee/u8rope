//! # Rope data structure
//!
//! ...
//!
//! ## Properties
//!
//! ...
//!
//! ## Usage
//!
//! ...

extern crate quickcheck;

///
/// Rope data structure.
///
#[derive(Debug)]
pub struct Rope {
    root: Node
}

///
/// Node
///
#[derive(Debug)]
pub enum Node {
    Leaf(Option<Vec<u8>>),
    Branch(Box<Node>, Box<Node>),
}

impl<T: Into<Vec<u8>>> From<T> for Rope {
    fn from(bytes: T) -> Self {
        Rope {
            root: Node::Leaf(Some(bytes.into())),
        }
    }
}

/*impl<'a> From<&'a str> for Rope {
    fn from(value: &str) -> Self {
        value.as_bytes().into()
    }
}*/

/*impl<'a> From<Rope> for Vec<u8> {
    fn from(rope: Rope) -> Self {
        unimplemented!();
    }
}*/

/*impl<'a> From<Rope> for String {
    fn from(rope: Rope) -> Self {
        unimplemented!();
    }
}*/

/// TODO: implement index trait (runtime panic if out of bounds)

///
/// Return the byte at position `index`.
///
/// # Examples
///
/// ```
/// # use u8rope::{Rope, get};
/// let rope = Rope::from("Hello, World!");
///
/// assert_eq!(get(&rope, 4), Some('o' as u8));
/// assert_eq!(get(&rope, 13), None);
/// ```
///
pub fn get(node: &Node, index: usize) -> Option<u8> {
    match node {
        &Node::Leaf(ref value) => {
            value.as_ref()
                .and_then(|s| s.get(index))
                .map(|x| *x)
        },
        &Node::Branch(ref left, ref right) => {
            /*if index < node.weight {
                get(left, index)
            } else {
                get(right, index - node.weight)
            }*/
            unimplemented!();
        },
    }
}

/*
function index(RopeNode node, integer i)
    if node.weight < i then
        return index(node.right, i - node.weight)
    else
        if exists(node.left) then
            return index(node.left, i)
        else
            return node.string[i]
        end
    end
end
*/

///
/// Concatenate two ropes, `rope1` and `rope2`, into a single rope.
///
/// # Examples
///
/// ```
/// assert_eq!(concat("Hello,", ", World!"), "Hello, World!");
/// ```
///
pub fn concat(rope1: Rope, rope2: Rope) -> Rope {
    unimplemented!();
}

///
/// Split the string `s` into two new strings `s_1` and `s_2`,
/// with (s_1, s_2) = (c_1, ..., c_i, c_i + 1, ..., c_m) where
/// c_m is the length of the string s.
///
/// # Examples
///
/// ```
/// assert_eq!(split("Hello, World!" 4), ("Hello", ", World!"));
/// ```
///
pub fn split(rope: Rope) -> (Rope, Rope) {
    unimplemented!();
}

///
/// Insert the string `s'` beginning at position `i` in the string `s`,
/// to form a new string c_1, ..., c_i, s', c_i + 1, ..., c_m where
/// c_m is the length of the string s'.
///
/// # Examples
///
/// ```
/// assert_eq!(insert("Hello!", ", World", 4), "Hello, World!");
/// ```
///
pub fn insert(rope: Rope, s: &[u8], i: usize) -> Rope {
    unimplemented!();
}

///
/// Delete the substring c_i, ..., c_i + j - 1, from s to form
/// a new string c_1, ..., c_i − 1, c_i + j, ..., c_m.
///
/// # Examples
///
/// ```
/// assert_eq!(delete("Hello, World!", 5, 7), "Hello!");
/// ```
///
pub fn delete(rope: Rope, i: usize, j: usize) -> Rope {
    unimplemented!();
}

///
/// Output the string c_i, ..., c_i + j − 1.
///
/// # Examples
///
/// ```
/// assert_eq!(report("Hello, World!", 0, 5), "Hello");
/// ```
///
pub fn report(rope: &Rope, i: usize, j: usize) -> &[u8] {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::{Rope, Node};

    fn create_wiki_rope() -> (Rope, Vec<u8>) {
        let text = "Hello_my_name_is_Simon".as_bytes().into();
        let rope = Rope {
            root: Node::Branch(
                Box::new(Node::Branch(
                    Box::new(Node::Branch(
                        Box::new(Node::Leaf(Some("Hello_".into()))),
                        Box::new(Node::Leaf(Some("my_".into())))
                    )),
                    Box::new(Node::Branch(
                        Box::new(Node::Branch(
                            Box::new(Node::Leaf(Some("na".into()))),
                            Box::new(Node::Leaf(Some("me_i".into())))
                        )),
                        Box::new(Node::Branch(
                            Box::new(Node::Leaf(Some("s".into()))),
                            Box::new(Node::Leaf(Some("_Simon".into())))
                        ))
                    ))
                )),
                Box::new(Node::Leaf(Some(vec![])))
            ),
        };

        (rope, text)
    }

    #[test]
    fn basic() {
        let ropes = vec![
            Rope::from(vec![1, 2, 3, 4]),
            Rope::from("Hello, World!"),
        ];

        for rope in ropes {
            println!("{:?}", rope);
        }
    }

    #[test]
    fn get() {
        fn same_behaviour_as_vec(test: Vec<u8>, index: usize) -> bool {
            let rope = Rope::from(test.as_slice());
            super::get(&rope.root, index) == test.get(index).map(|x| *x)
        }

        fn test_rope_from_wiki(index: usize) -> bool {
            let (rope, test) = create_wiki_rope();
            super::get(&rope.root, index) == test.get(index).map(|x| *x)
        }

        quickcheck(same_behaviour_as_vec as fn(Vec<u8>, usize) -> bool);
        quickcheck(test_rope_from_wiki as fn(usize) -> bool);
    }

    #[test]
    fn concat() {
        unimplemented!();
    }

    #[test]
    fn split() {
        unimplemented!();
    }

    #[test]
    fn insert() {
        unimplemented!();
    }

    #[test]
    fn delete() {
        unimplemented!();
    }

    #[test]
    fn report() {
        unimplemented!();
    }
}
