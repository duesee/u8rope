use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub struct Rope {
    root: RcNode,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RopeByte<'a> {
    index: usize,
    rope: &'a Rope,
}

#[derive(Debug, PartialEq, Clone)]
enum Node {
    Leaf(Vec<u8>),
    Branch(RcNode, RcNode),
}

#[derive(Debug, PartialEq, Clone)]
struct RcNode(Rc<Node>);

impl RcNode {
    fn get(&self, index: usize) -> Option<&u8> {
        match *self.0 {
            Node::Leaf(ref value) => value.get(index),
            Node::Branch(ref l, ref r) => {
                if index < self.weight() {
                    l.get(index)
                } else {
                    r.get(index - self.weight())
                }
            },
        }
    }

    fn concat(&self, other: &Self) -> Self {
        RcNode(Rc::new(Node::Branch(self.clone(), other.clone())))
    }

    fn split(&self, index: usize) -> (Self, Self) {
        match *self.0 {
            Node::Leaf(ref value) => {
                let (l, r) = value.split_at(index);
                (RcNode(Rc::new(Node::Leaf(l.to_owned()))),
                 RcNode(Rc::new(Node::Leaf(r.to_owned()))))
            },
            Node::Branch(ref l, ref r) => {
                unimplemented!();
            }
        }
    }

    fn len(&self) -> usize {
        match *self.0 {
            Node::Leaf(ref vec) => vec.len(),
            Node::Branch(ref l, ref r) => l.len() + r.len(),
        }
    }

    fn weight(&self) -> usize {
        match *self.0 {
            Node::Leaf(ref vec) => vec.len(),
            Node::Branch(ref l, _) => l.len(),
        }
    }

    fn graphviz(&self, counter: usize, names: &mut Vec<u8>) {
        let name = names.pop().unwrap_or('?' as u8) as char;

        match *self.0 {
            Node::Leaf(ref value) => {
                let name = format!("\"{} {:?}\"", name, value);
                println!("{};", name);
                println!("\t{} [shape=rectangle]", name);
            },
            Node::Branch(ref l, ref r) => {
                let name = format!("\"{}, ({}w) {}l\"", name, self.weight(), self.len());

                if counter > 0 {
                    println!("{};", name);
                }

                print!("\t{} -> ", name);
                l.graphviz(counter + 1, names);
                print!("\t{} -> ", name);
                r.graphviz(counter + 1, names);
            }
        }
    }
}

impl Rope {
    pub fn new() -> Self {
        Rope { root: RcNode(Rc::new(Node::Leaf(vec![]))) }
    }

    pub fn get(&self, index: usize) -> Option<&u8> {
        self.root.get(index)
    }

    pub fn concat(&self, other: &Rope) -> Self {
        Rope { root: self.root.concat(&other.root) }
    }

    pub fn split(&self, index: usize) -> (Self, Self) {
        let (l, r) = self.root.split(index);
        (Rope { root: l }, Rope {root: r} )
    }

    pub fn iter(&self) -> RopeByte {
        IntoIterator::into_iter(self)
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    pub fn graphviz(&self) {
        println!("digraph BST {{");
        let mut names = (65..91).rev().collect::<Vec<_>>();
        self.root.graphviz(0, &mut names);
        println!("}}");
    }
}

impl<'a> IntoIterator for &'a Rope {
    type Item = &'a u8;
    type IntoIter = RopeByte<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RopeByte { index: 0, rope: self }
    }
}

impl<'a> Iterator for RopeByte<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.rope.get(self.index);
        self.index += 1;
        ret
    }
}

impl<T: Into<Vec<u8>>> From<T> for Rope {
    fn from(bytes: T) -> Self {
        Rope { root: RcNode(Rc::new(Node::Leaf(bytes.into()))) }
    }
}

#[cfg(test)]
mod test {
    extern crate quickcheck;

    use super::*;
    use self::quickcheck::quickcheck;

    fn create_wiki_rope() -> (Rope, Vec<u8>) {
//        let node_n = Node {
//            head: NodeHead { weight: 6 },
//            body: NodeBody::Leaf("_Simon".into()),
//        };
//
//        let node_m = Node {
//            head: NodeHead { weight: 1 },
//            body: NodeBody::Leaf("s".into()),
//        };
//
//        let node_k = Node {
//            head: NodeHead { weight: 4 },
//            body: NodeBody::Leaf("me_i".into()),
//        };
//
//        let node_j = Node {
//            head: NodeHead { weight: 2 },
//            body: NodeBody::Leaf("na".into()),
//        };
//
//        let node_h = Node {
//            head: NodeHead { weight: 1 },
//            body: NodeBody::Branch(Rc::new(node_m), Rc::new(node_n)),
//        };
//
//        let node_g = Node {
//            head: NodeHead { weight: 2 },
//            body: NodeBody::Branch(Rc::new(node_j), Rc::new(node_k)),
//        };
//
//        let node_f = Node {
//            head: NodeHead { weight: 3 },
//            body: NodeBody::Leaf("my_".into()),
//        };
//
//        let node_e = Node {
//            head: NodeHead { weight: 6 },
//            body: NodeBody::Leaf("Hello_".into()),
//        };
//
//        let node_d = Node {
//            head: NodeHead { weight: 6 },
//            body: NodeBody::Branch(Rc::new(node_g), Rc::new(node_h)),
//        };
//
//        let node_c = Node {
//            head: NodeHead { weight: 6 },
//            body: NodeBody::Branch(Rc::new(node_e), Rc::new(node_f)),
//        };
//
//        let node_b = Node {
//            head: NodeHead { weight: 9 },
//            body: NodeBody::Branch(Rc::new(node_c), Rc::new(node_d)),
//        };
//
//        let node_a = Node {
//            head: NodeHead { weight: 22 },
//            body: NodeBody::Branch(Rc::new(node_b), Rc::new(Node {
//                head: NodeHead { weight: 0 },
//                body: NodeBody::Nil,
//            })),
//        };
//
//        let rope = Rope {
//            root: Rc::new(node_a),
//            length: 22,
//        };
//
//        (rope, "Hello_my_name_is_Simon".as_bytes().into())
        unimplemented!();
    }

    #[test]
    fn basic() {
        let ropes = vec![
            Rope::new(),
            Rope::from("created using `from` and &str"),
            Rope::from(vec![0, 1, 2, 3, 4]),
            "created using `into`".into(),
            //create_wiki_rope().0,
        ];

        for rope in ropes {
            for i in 0..30 {
                print!("{}", rope.get(i).map(|x| *x as char).unwrap_or('?'));
            }
            println!();
        }
    }

    #[test]
    fn from() {
//        let r1 = Rope::from(vec![]);
//        let r2 = Rope::from(vec![0]);
//        let r3 = Rope::from(vec![0, 1]);
//
//        assert_eq!(r1.len(), 0);
//        assert_eq!(r2.len(), 1);
//        assert_eq!(r3.len(), 2);
//
//        assert_eq!(r1.root.head.weight, 0);
//        assert_eq!(r1.root.body, NodeBody::Nil);
//
//        assert_eq!(r2.root.head.weight, 1);
//        assert_eq!(r2.root.body, NodeBody::Leaf(vec![0]));
//
//        assert_eq!(r3.root.head.weight, 2);
//        assert_eq!(r3.root.body, NodeBody::Leaf(vec![0, 1]));
    }

    #[test]
    fn clone() {
        impl Drop for Node {
            fn drop(&mut self) {
                println!("DROP COUNTER");
            }
        }

        let r1 = Rope::from(vec![0; 1024*1024]);

        let mut container = Vec::new();
        for i in 1..1024 * 2 {
            container.push(r1.clone());
        }
    }

    #[test]
    fn get() {
        fn same_as_vec(test: Vec<u8>, index: usize) -> bool {
            let rope = Rope::from(test.clone());
            rope.get(index) == test.get(index) && rope.len() == test.len()
        }

//        fn test_wiki_rope(index: usize) -> bool {
//            let (rope, test) = create_wiki_rope();
//            rope.get(index) == test.get(index) && rope.len() == test.len()
//        }

        quickcheck(same_as_vec as fn(Vec<u8>, usize) -> bool);
//        quickcheck(test_wiki_rope as fn(usize) -> bool);
    }

    #[test]
    fn concat() {
        fn same_as_vec(test: Vec<Vec<u8>>) -> bool {
            let r = {
                let mut tmp = Rope::new();
                for v in &test {
                    tmp = tmp.concat(&Rope::from(v.clone()));
                }
                tmp
            };

            let v = {
                let mut tmp = Vec::new();
                for v in &test {
                    tmp.append(&mut v.clone());
                }
                tmp
            };

            println!("R: {:?}\nV: {:?}", r.iter().take(20).collect::<Vec<_>>(), v.iter().take(20).collect::<Vec<_>>());

            r.iter().count() == v.iter().count() && r.iter().zip(v.iter()).all(|(&x, &y)| x == y)
        }

        quickcheck(same_as_vec as fn(Vec<Vec<u8>>) -> bool);
    }

    #[test]
    fn split() {
        let rope = Rope::from(vec![0, 1, 2, 3, 4, 5, 6]);
        let (l, r) = rope.split(4);

        l.graphviz();
        r.graphviz();
    }

    #[test]
    fn report() {
        let rope = Rope::from("Hello, World!");

        let x = IntoIterator::into_iter(&rope);

        // let v: () = x; // RopeByte

        for c in x {
            print!("{}", *c as char);
        }
        println!();

        for c in &rope {
            print!("{}", *c as char);
        }
        println!();

        let bytes = IntoIterator::into_iter(&rope).collect::<Vec<_>>();
        println!("{:?}", bytes);

        let x = IntoIterator::into_iter(&rope);
        let y = IntoIterator::into_iter(&rope);
        let z = IntoIterator::into_iter(&rope);

        println!("{:?},{:?},{:?}", x.count(), y.min(), z.size_hint());

        let x = Rope::from("Hello, ");
        let y = Rope::from("World!");
        for c in x.iter().chain(y.iter()) {
            print!("[{}]", *c as char);
        }
        println!();
    }

    #[test]
    fn graphviz() {
        let ropeA = {
            let tmp = Rope::from(vec![0, 1]);
            let tmp = tmp.concat(&Rope::from(vec![2, ]));
            let tmp = tmp.concat(&Rope::from(vec![3, 4]));
            let tmp = tmp.concat(&Rope::from(vec![5]));
            tmp
        };

        let ropeB = {
            let tmp = Rope::from(vec![6, 7]);
            let tmp = tmp.concat(&Rope::from(vec![8, 9]));
            let tmp = tmp.concat(&Rope::from(vec![10, 11]));
            let tmp = tmp.concat(&Rope::from(vec![12]));
            tmp
        };

        let ropeC = ropeA.concat(&ropeB);

        ropeA.graphviz();
        ropeB.graphviz();
        ropeC.graphviz();
    }
}

/*


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

///
/// Rope data structure.
///
///
/// Rope byte-by-byte iterator.
/// TODO: Very, very, bad version... (for testing purposes only)
///

//
//    #[test]
//    fn insert() {
//        unimplemented!();
//    }
//
//    #[test]
//    fn delete() {
//        unimplemented!();
//    }

/// Create an empty `Rope`.

/// Return the byte at position `index`.
///
/// # Examples
///
/// ```
/// # use u8rope::Rope;
/// let rope = Rope::from("Hello, World!");
///
/// assert_eq!(rope.get( 4), Some(&('o' as u8)));
/// assert_eq!(rope.get(13), None);
/// ```

/// Concatenate `self` with `other` and return the new `Rope`.
///
/// # Examples
///
/// ```
/// # use u8rope::Rope;
/// let r1 = Rope::from("Hello, ");
/// let r2 = Rope::from("World!");
///
/// assert_eq!(r1.concat(&r2), "Hello, World!")
/// ```

/// Iterator (TODO)
///
/// # Examples
///
/// ```
/// # use u8rope::Rope;
/// let rope = Rope::from(vec![0, 1, 2, 3, 4]);
/// for byte in &rope {
///     print!("{:02x }", byte);
/// }
/// println!();
/// ```

///
/// Returns the length of the `Rope`, i.e. the bytes it contains. (TODO)
///

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
//pub fn split(rope: Rope) -> (Rope, Rope) {
//    unimplemented!();
//}

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
//pub fn insert(rope: Rope, s: &[u8], i: usize) -> Rope {
//    unimplemented!();
//}

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
//pub fn delete(rope: Rope, i: usize, j: usize) -> Rope {
//    unimplemented!();
//}

///
/// Output the string c_i, ..., c_i + j − 1.
///
/// # Examples
///
/// ```
/// assert_eq!(report("Hello, World!", 0, 5), "Hello");
/// ```
///
//pub fn report(rope: &Rope, i: usize, j: usize) -> &[u8] {
//    unimplemented!();
//}


//    fn get_mut(&mut self, index: usize) -> Option<&mut u8> {
//        let &mut Node { ref head, ref body } = self;
//
//        match *body {
//            NodeBody::Nil => None,
//            NodeBody::Leaf(ref value) => value.get_mut(index),
//            NodeBody::Branch(ref left, ref right) => {
//                if index < head.weight {
//                    left.get_mut(index)
//                } else {
//                    right.get_mut(index - head.weight)
//                }
//            },
//        }
//    }
*/