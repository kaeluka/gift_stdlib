
use giftr::refs::*;
//use giftr::refs::functional::Ref as Ref;
use giftr::refs::imperative::Ref as Ref;
use std::iter::Iterator;
use std::mem::swap;

use list::List;

#[derive(Clone, Debug)]
pub struct Deque<T: Clone> {
    front : Ref<List<T>>,
    back  : Ref<List<T>>,
}

impl <T: Clone> Deque<T> {
    pub fn new() -> Deque<T> {
        Deque { front: Ref::new(List::new()), back: Ref::new(List::new())}
    }

    pub fn push_front(&mut self, x: T) {
        self.front.prepend(x);
    }

    pub fn push_back(&mut self, x: T) {
        self.back.prepend(x);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let front = self.front.pop_front();
        if front.is_some() {
            front
        } else {
            // FIXME we should reshuffle here, right?
            self.back.pop_back()
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let back = self.back.pop_front();
        if back.is_some() {
            back
        } else {
            // FIXME we should reshuffle here, right?
            self.front.pop_back()
        }
    }

    pub fn reverse(&mut self) {
        swap(&mut self.front, &mut self.back);
    }

    fn to_iter(self) -> Iter<T> {
        Iter { deque: Ref::new(self) }
    }

    pub fn iter(&self) -> Iter<T> {
        self.clone().to_iter()
    }

    pub fn rev_iter(&self) -> Iter<T> {
        let mut cln = self.clone();
        cln.reverse();
        cln.to_iter()
    }
}

pub struct Iter<T: Clone> {
    deque : Ref<Deque<T>>,
}

impl <T: Clone> Iterator for Iter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.deque.pop_front()
    }
}


#[test]
fn deque_pop_front() {
    let mut d = Deque::new();
    d.push_front(2);
    d.push_front(1);
    d.push_back(3);

    assert_eq!(Some(1), d.pop_front());
    assert_eq!(Some(2), d.pop_front());
    assert_eq!(Some(3), d.pop_front());
    assert_eq!(None, d.pop_front());
}

#[test]
fn deque_pop_back() {
    let mut d = Deque::new();
    d.push_front(2);
    d.push_front(1);
    d.push_back(3);

    assert_eq!(Some(3), d.pop_back());
    assert_eq!(Some(2), d.pop_back());
    assert_eq!(Some(1), d.pop_back());
    assert_eq!(None, d.pop_back());
}

#[test]
fn deque_reverse() {
    let mut d = Deque::new();
    d.push_front(2);
    d.push_front(1);
    d.push_back(3);
    d.push_back(4);

    d.reverse();

    assert_eq!(Some(4), d.pop_front());
    assert_eq!(Some(3), d.pop_front());
    assert_eq!(Some(2), d.pop_front());
    assert_eq!(Some(1), d.pop_front());
    assert_eq!(None, d.pop_back());
}

#[test]
fn deque_iter() {
    let mut d = Ref::new(Deque::new());
    d.push_front(2);
    d.push_front(1);
    d.push_back(3);
    d.push_back(4);

    let mut i = 1;
    for v in d.iter() {
        assert_eq!(i, v);
        i += 1;
    }
//
//    let mut i = 4;
//    for v in d.rev_iter() {
//        assert_eq!(i, v);
//        i -= 1;
//    }
}
