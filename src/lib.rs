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
#[derive(Debug)]
pub struct Rope {
    root: Node
}

///
/// Rope byte-by-byte iterator.
/// TODO: Very, very, bad version... (for testing purposes only)
///
#[derive(Debug)]
pub struct RopeByte<'a> {
    index: usize,
    rope: &'a Rope,
}

#[derive(Debug)]
struct Node {
    head: NodeHead,
    body: NodeBody,
}

#[derive(Debug)]
struct NodeHead {
    weight: usize,
}

#[derive(Debug)]
enum NodeBody {
    Nil,
    Leaf(Vec<u8>),
    Branch(Box<Node>, Box<Node>),
}

impl Rope {
    /// Create an empty `Rope`.
    pub fn new() -> Self {
        Rope {
            root: Node {
                head: NodeHead { weight: 0 },
                body: NodeBody::Nil,
            }
        }
    }

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
    pub fn get(&self, index: usize) -> Option<&u8> {
        self.root.get(index)
    }

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
    pub fn concat(&self, other: &Rope) -> Self {
        Rope { root: self.root.concat(&other.root) }
    }

    ///
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
    pub fn iter(&self) -> RopeByte {
        IntoIterator::into_iter(self)
    }
}

impl Node {
    fn get(&self, index: usize) -> Option<&u8> {
        let &Node { ref head, ref body } = self;

        match *body {
            NodeBody::Nil => None,
            NodeBody::Leaf(ref value) => value.get(index),
            NodeBody::Branch(ref left, ref right) => {
                if index < head.weight {
                    left.get(index)
                } else {
                    right.get(index - head.weight)
                }
            },
        }
    }

    fn concat(&self, other: &Node) -> Node {
//        let left = self;
//        let right = other;
//
//        let &Node { head: ref head1, body: ref body1 } = left;
//        let &Node { head: ref head2, body: ref body2 } = right;
//
//        Node {
//            head: NodeHead { weight: head1.weight },
//            body: NodeBody::Branch(Box::new(*left), Box::new(*right)),
//        }
        unimplemented!();
    }
}

impl<'a> IntoIterator for &'a Rope {
    type Item = &'a u8;
    type IntoIter = RopeByte<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RopeByte {
            index: 0,
            rope: self,
        }
    }
}

impl<'a> Iterator for RopeByte<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.index < self.rope.root.head.weight {
            self.rope.get(self.index)
        } else {
            None
        };
        self.index += 1;
        ret
    }
}

impl<T: Into<Vec<u8>>> From<T> for Rope {
    fn from(bytes: T) -> Self {
        let data = bytes.into();
        Rope {
            root: Node {
                head: NodeHead {
                    weight: data.len(),
                },
                body: NodeBody::Leaf(data),
            },
        }
    }
}

#[cfg(test)]
mod test {
    extern crate quickcheck;

    use super::*;
    use self::quickcheck::quickcheck;

    fn create_wiki_rope() -> (Rope, Vec<u8>) {
        let node_n = Node {
            head: NodeHead { weight: 6 },
            body: NodeBody::Leaf("_Simon".into()),
        };

        let node_m = Node {
            head: NodeHead { weight: 1 },
            body: NodeBody::Leaf("s".into()),
        };

        let node_k = Node {
            head: NodeHead { weight: 4 },
            body: NodeBody::Leaf("me_i".into()),
        };

        let node_j = Node {
            head: NodeHead { weight: 2 },
            body: NodeBody::Leaf("na".into()),
        };

        let node_h = Node {
            head: NodeHead { weight: 1 },
            body: NodeBody::Branch(Box::new(node_m), Box::new(node_n)),
        };

        let node_g = Node {
            head: NodeHead { weight: 2 },
            body: NodeBody::Branch(Box::new(node_j), Box::new(node_k)),
        };

        let node_f = Node {
            head: NodeHead { weight: 3 },
            body: NodeBody::Leaf("my_".into()),
        };

        let node_e = Node {
            head: NodeHead { weight: 6 },
            body: NodeBody::Leaf("Hello_".into()),
        };

        let node_d = Node {
            head: NodeHead { weight: 6 },
            body: NodeBody::Branch(Box::new(node_g), Box::new(node_h)),
        };

        let node_c = Node {
            head: NodeHead { weight: 6 },
            body: NodeBody::Branch(Box::new(node_e), Box::new(node_f)),
        };

        let node_b = Node {
            head: NodeHead { weight: 9 },
            body: NodeBody::Branch(Box::new(node_c), Box::new(node_d)),
        };

        let node_a = Node {
            head: NodeHead { weight: 22 },
            body: NodeBody::Branch(Box::new(node_b), Box::new(Node {
                head: NodeHead { weight: 0 },
                body: NodeBody::Nil,
            })),
        };

        let rope = Rope {
            root: node_a
        };

        (rope, "Hello_my_name_is_Simon".as_bytes().into())
    }

    #[test]
    fn basic() {
        let ropes = vec![
            Rope::new(),
            Rope::from("created using `from` and &str"),
            Rope::from(vec![0, 1, 2, 3, 4]),
            "created using `into`".into(),
            create_wiki_rope().0,
        ];

        for rope in ropes {
            for i in 0..30 {
                print!("{}", rope.get(i).map(|x| *x as char).unwrap_or('?'));
            }
            println!();
        }
    }

    #[test]
    fn get() {
        fn same_behaviour_as_vec(test: Vec<u8>, index: usize) -> bool {
            let rope = Rope::from(test.clone());
            rope.get(index) == test.get(index)
        }

        fn test_wiki_rope(index: usize) -> bool {
            let (rope, test) = create_wiki_rope();
            rope.get(index) == test.get(index)
        }

        quickcheck(same_behaviour_as_vec as fn(Vec<u8>, usize) -> bool);
        quickcheck(test_wiki_rope as fn(usize) -> bool);
    }

    #[test]
    fn concat() {
        //        let (mut s1, mut s2): (Vec<u8>, Vec<u8>) = ("Hello, ".into(), "World!".into());
        //        let mut truth = s1.clone();
        //        truth.append(&mut s2.clone());
        //
        //        let (r1, r2) = (Rope::from(s1.clone()), Rope::from(s2.clone()));
        //        let value = r1.concat(r2);
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
}

/*
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