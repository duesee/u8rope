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

use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub struct Rope {
    root: RcNode,
}

#[derive(Debug)]
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

impl Rope {
    pub fn new() -> Self {
        Rope { root: RcNode(Rc::new(Node::Leaf(vec![]))) }
    }

    pub fn get(&self, index: usize) -> Option<&u8> {
        self.root.get(index)
    }

    pub fn append(&self, other: &Self) -> Self {
        Rope { root: self.root.append(&other.root) }
    }

    pub fn split(&self, index: usize) -> (Self, Self) {
        let (l, r) = self.root.split(index);
        (Rope { root: l }, Rope {root: r} )
    }

    pub fn insert(&self, index: usize, other: &Self) -> Self {
        Rope { root: self.root.insert(index, &other.root) }
    }

    pub fn delete(&self, index: usize, count: usize) -> Self {
        Rope { root: self.root.delete(index, count) }
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

    fn append(&self, other: &Self) -> Self {
        RcNode(Rc::new(Node::Branch(self.clone(), other.clone())))
    }

    fn split(&self, index: usize) -> (Self, Self) {
        match *self.0 {
            Node::Leaf(ref value) => {
                let (l, r) = value.split_at(index);
                (RcNode(Rc::new(Node::Leaf(l.into()))),
                 RcNode(Rc::new(Node::Leaf(r.into()))))
            },
            Node::Branch(ref l, ref r) => {
                if index == self.weight() {
                    (l.clone(), r.clone())
                } else if index < self.weight() {
                    let (l_new, r_new) = l.split(index);
                    (l_new, r_new.append(&r))
                } else {
                    let (l_new, r_new) = r.split(index - self.weight());
                    (l.append(&l_new), r_new)
                }
            }
        }
    }

    fn insert(&self, index: usize, other: &Self) -> Self {
        let (ref l, ref r) = self.split(index);
        l.append(other).append(r)
    }

    fn delete(&self, index: usize, count: usize) -> Self {
        let (ref l, ref i) = self.split(index);
        let (_, ref r) = i.split(count);
        l.append(r)
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
        let name = format!("{}: {}", names.pop().unwrap(), &(*self.0) as *const Node as u64);

        match *self.0 {
            Node::Leaf(ref value) => {
                let name = format!("\"{} {:?}\"", name, value.iter().map(|x| *x as char).collect::<Vec<_>>());
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

impl std::fmt::Display for Rope {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for i in self.iter() {
            write!(fmt, "{}", *i as char)?;
        }

        Ok(())
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
        // See https://en.wikipedia.org/w/index.php?title=Rope_(data_structure)&oldid=746036659
        let rope_c = &Rope::from("Hello_").append(&Rope::from("my_"));
        let rope_g = &Rope::from("na").append(&Rope::from("me_i"));
        let rope_h = &Rope::from("s").append(&Rope::from("_Simon"));
        let rope_d = rope_g.append(&rope_h);
        let rope_b = rope_c.append(&rope_d);
        let rope_a = rope_b.append(&Rope::from(""));
        let rope = rope_a.clone();

        (rope, "Hello_my_name_is_Simon".into())
    }

    impl Drop for RcNode {
        fn drop(&mut self) {
            println!("Dropping node, {}/{}", self.weight(), self.len());
        }
    }

    #[test]
    fn usage() {
        let rope = Rope::new();
        println!("{}", rope);
        rope.graphviz();
        let rope = rope.append(&Rope::from("Hello,... World!"));
        println!("{}", rope);
        rope.graphviz();
        let rope = rope.delete(6, 3);
        println!("{}", rope);
        rope.graphviz();
        let rope = rope.delete(7, 5);
        println!("{}", rope);
        rope.graphviz();
        let rope = rope.insert(7, &Rope::from("Universe"));
        println!("{}", rope);
        rope.graphviz();
    }

    #[test]
    fn from() {
        let ropes = vec![
            Rope::new(),
            Rope::from("created using `from` and &str"),
            Rope::from(vec![0, 1, 2, 3, 4]),
            "created using `into`".into(),
            create_wiki_rope().0,
        ];

        for rope in ropes {
            println!("{}", rope);
        }
    }

    #[test]
    fn clone() {
        let r1 = Rope::from(vec![0; 1024*1024]);

        let mut container = Vec::new();
        for _ in 1..1024 * 2 {
            container.push(r1.clone());
        }
    }

    #[test]
    fn wiki() {
        let (rope, _) = create_wiki_rope();
        rope.graphviz();
    }

    #[test]
    fn graphviz() {
        let rope_a = {
            let tmp = Rope::from(vec![0, 1]);
            let tmp = tmp.append(&Rope::from(vec![2, ]));
            let tmp = tmp.append(&Rope::from(vec![3, 4]));
            let tmp = tmp.append(&Rope::from(vec![5]));
            tmp
        };

        let rope_b = {
            let tmp = Rope::from(vec![6, 7]);
            let tmp = tmp.append(&Rope::from(vec![8, 9]));
            let tmp = tmp.append(&Rope::from(vec![10, 11]));
            let tmp = tmp.append(&Rope::from(vec![12]));
            tmp
        };

        let rope_c = rope_a.append(&rope_b);

        rope_a.graphviz();
        rope_b.graphviz();
        rope_c.graphviz();
    }

    mod operations {
        use super::*;

        #[test]
        fn get() {
            fn same_as_vec(test: Vec<u8>, index: usize) -> bool {
                let rope = Rope::from(test.clone());
                rope.get(index) == test.get(index) && rope.len() == test.len()
            }

            fn test_wiki_rope(index: usize) -> bool {
                let (rope, test) = create_wiki_rope();
                rope.get(index) == test.get(index) && rope.len() == test.len()
            }

            quickcheck(same_as_vec as fn(Vec<u8>, usize) -> bool);
            quickcheck(test_wiki_rope as fn(usize) -> bool);
        }

        #[test]
        fn append() {
            fn same_as_vec(test: Vec<Vec<u8>>) -> bool {
                let r = {
                    let mut tmp = Rope::new();
                    for v in &test {
                        tmp = tmp.append(&Rope::from(v.clone()));
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
        fn split1() {
            let d = Rope::from(vec![0, 1, 2]);
            let e = Rope::from(vec![3, 4, 5]);
            let f = Rope::from(vec![6, 7]);
            let rope = d.append(&e).append(&f);

            rope.graphviz();

            let (l, r) = rope.split(3);

            l.graphviz();
            r.graphviz();
        }

        #[test]
        fn split2() {
            let (rope, _) = create_wiki_rope();

            rope.graphviz();

            let (l, r) = rope.split(15);

            l.graphviz();
            r.graphviz();
        }

        #[test]
        fn insert() {
            let (l, m, r) = (Rope::from("Hel"), Rope::from("lo, "), Rope::from("World!"));
            let rope = l.append(&r);
            let rope = rope.insert(3, &m);

            println!("{}", rope);
        }

        #[test]
        fn delete() {
            let rope = Rope::from("Hel........lo, World!");
            let rope = rope.delete(3, 8);

            println!("{}", rope);
        }

        #[test]
        fn iter() {
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
}

/*
///
/// Rope data structure.
///
///
/// Rope byte-by-byte iterator.
/// TODO: Very, very, bad version... (for testing purposes only)
///

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
/// Return the length of the `Rope`, i.e. how many bytes it contains.
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

///
/// Output the string c_i, ..., c_i + j − 1.
///
/// # Examples
///
/// ```
/// assert_eq!(report("Hello, World!", 0, 5), "Hello");
/// ```
///

*/