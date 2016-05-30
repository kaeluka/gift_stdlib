use giftr::refs::*;
//use giftr::refs::functional::Ref as Ref;
use giftr::refs::imperative::Ref as Ref;
use std::iter::Iterator;

use std::cmp::Ord;

#[derive(Clone,Debug)]
pub struct Heap<T: Ord+Clone> {
    cell : Ref<HeapCell<T>>,
}

impl <T: Ord+Clone> Heap<T> {
    pub fn new() -> Heap<T> {
        Heap { cell: Ref::new(HeapCell::new()) }
    }

    pub fn is_empty(&self) -> bool {
        self.cell.is_empty()
    }

    pub fn insert(&mut self, x: T) {
        self.cell.apply(|cell| cell.insert(x))
    }

    pub fn pop_min(&mut self) -> Option<T> {
        let mut ret = None;
        self.cell.apply(|cell| {
            if let Some((r, rest)) = cell.pop_min() {
                ret = Some(r);
                rest
            } else {
                HeapCell::Empty
            }
        });
        ret
    }

    pub fn to_iter(self) -> Iter<T> {
        Iter { heap: self }
    }

    pub fn iter(&self) -> Iter<T> {
        self.clone().to_iter()
    }

}

pub struct Iter<T: Ord+Clone> {
    heap: Heap<T>,
}

impl <T: Ord+Clone> Iterator for Iter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.heap.pop_min()
    }
}

#[derive(Clone, Debug)]
pub enum HeapCell<T: Ord+Clone> {
    Empty,
    Node { rank: i32, elt: T, left:  Ref<HeapCell<T>>, right: Ref<HeapCell<T>> },
}

impl <T: Ord+Clone> HeapCell<T> {
    pub fn new() -> HeapCell<T> {
        HeapCell::Empty
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            HeapCell::Empty => true,
            _           => false
        }
    }

    pub fn insert(self, x: T) -> Self {
        self.merge(HeapCell::Node { rank: 1, elt: x, left: Ref::new(HeapCell::new()), right: Ref::new(HeapCell::new()) })
    }

    fn make_node(x: T, h1: Ref<HeapCell<T>>, h2: Ref<HeapCell<T>>) -> HeapCell<T> {
        if h1.rank() >= h2.rank() {
            HeapCell::Node{ rank: h2.rank()+1, elt: x, left: h1, right: h2 }
        } else {
            HeapCell::Node{ rank: h1.rank()+1, elt: x, left: h2, right: h1 }
        }
    }

    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (HeapCell::Empty, h) => h,
            (h,  HeapCell::Empty) => h,
            (HeapCell::Node{rank: n1, elt: x, left: a1, right: b1},
             HeapCell::Node{rank: n2, elt: y, left: a2, right: b2}) => {
                if true {
                    if x <= y {
                        let h2 = HeapCell::Node{rank: n2, elt: y, left: a2, right: b2};
                        Self::make_node(x, a1, Ref::new(b1.consume().merge(h2)))
                    } else {
                        let h1 = HeapCell::Node{rank: n1, elt: x, left: a1, right: b1};
                        Self::make_node(y, a2, Ref::new(b2.consume().merge(h1)))
                    }
                } else {
                    panic!("bug")
                }
            }
        }
    }

    fn rank(&self) -> i32 {
        match *self {
            HeapCell::Empty          => 0,
            HeapCell::Node{rank, ..} => rank
        }
    }

    pub fn pop_min(self) -> Option<(T, Self)> {
        match self {
            HeapCell::Empty => None,
            HeapCell::Node{elt: x, left: l, right: r, ..} => {
                Some((x, l.consume().merge(r.consume())))
            }
        }
    }

}


#[test]
fn heap_empty() {
    let mut h : Heap<i8> = Heap::new();
    assert_eq!(None, h.pop_min())
}

#[test]
fn heap_min() {
    let mut h : Ref<Heap<i8>> = Ref::new(Heap::new());
    h.insert(4);
    h.insert(2);
    h.insert(1);
    h.insert(5);
    h.insert(3);
    assert_eq!(Some(1), h.pop_min());
    assert_eq!(Some(2), h.pop_min());
    assert_eq!(Some(3), h.pop_min());
    assert_eq!(Some(4), h.pop_min());
    assert_eq!(Some(5), h.pop_min());
    assert_eq!(None,    h.pop_min());
    assert_eq!(None,    h.pop_min());
}

#[test]
fn heap_iter() {
    let mut h : Heap<i8> = Heap::new();
    h.insert(4);
    h.insert(2);
    h.insert(1);
    h.insert(5);
    h.insert(3);

    let mut i = 1;
    for v in h.iter() {
        assert_eq!(i, v);
        i += 1;
    }

    let mut i = 1;
    for v in h.to_iter() {
        assert_eq!(i, v);
        i += 1;
    }
}

#[cfg(test)]
mod bench {
    use test;
    use test::Bencher;
    use super::Heap;
    use std::collections::VecDeque;

    #[bench]
    fn hp_insert(b: &mut Bencher) {
        let mut vec1  = VecDeque::new();
        let size = 100000;
        for i in 0..size {
            vec1.push_back(-i);
        }

        let mut hp = Heap::new();

        b.iter(
            || {
                hp.insert(vec1.pop_front().unwrap());
                hp.insert(vec1.pop_back().unwrap());
            }
        );
    }

}
