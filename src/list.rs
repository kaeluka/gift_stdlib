use giftr::refs::*;
use giftr::refs::functional::Ref as Ref;
//use giftr::refs::imperative::Ref as Ref;
use std::mem::replace;
use std::iter::Iterator;

#[derive(Clone, Debug)]
pub struct Node<T: Clone> {
    next: Option<Ref<Node<T>>>,
    elt: Option<T>,
}

impl <T: Clone> Node<T> {
    fn new(x: T) -> Node<T> {
        Node { next: None, elt: Some(x) }
    }

    fn len(&self) -> i32 {
        if let Some(ref node) = self.next {
            1 + node.len()
        } else {
            1
        }
    }

    fn append(&mut self, x: T) {
        if let Some(ref mut next) = self.next {
            next.append(x)
        } else {
            self.next = Some(Ref::new(Node::new(x)))
        }
    }

    fn is_last(&self) -> bool {
        self.next.is_none()
    }

    fn pop_back(&mut self) -> Option<T> {
        assert!(self.next.is_some());
        let mut ret = None;
        let mut took_here = false;
        if let Some(ref mut next) = self.next {
            println!("have next");

            if next.is_last() {
                ret = _move_opt(&mut next.elt);
                took_here = true;
            } else {
                ret = next.pop_back();
            }
        }
        if took_here {
            self.next = None
        }
        ret
    }
}

#[derive(Clone, Debug)]
pub struct List<T: Clone> {
    first : Option<Ref<Node<T>>>,
}

impl <T: Clone> List<T> {
    pub fn new() -> List<T> {
        List { first: None }
    }

    pub fn prepend(&mut self, x: T) {
        let new_next = replace(&mut self.first, None);
        let new_first = Node { elt: Some(x), next: new_next };
        self.first = Some(Ref::new(new_first));
    }

    pub fn append(&mut self, x:T) {
        if let Some(ref mut node) = self.first {
            node.append(x)
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let mut ret = None;
        let optfirst = _move_opt(&mut self.first);
        if let Some(first) = optfirst {
            let Node { elt, next } = first.into_inner();
            self.first = next;
            ret = elt;
        }
        ret
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let mut ret = None;
        let mut first_is_last = false;
        if let Some(ref mut first) = self.first {
            if first.is_last() {
                println!("first is last");
                first_is_last = true;
                ret = _move_opt(&mut first.elt);
            } else {
                println!("first is not last");
                ret = first.pop_back();
            }
        }
        if first_is_last {
            self.first = None;
        }
        ret

//        if let Some(ref mut first) = self.first {
//            if first.is_last() {
//                let mut first = _move_opt(first).expect("");
//
//                _move_opt(&mut first.elt)
//            } else {
//                first.pop_back()
//            }
//        } else {
//            None
//        }
    }

    pub fn len(&self) -> i32 {
        if let Some(ref first) = self.first {
            first.len()
        } else {
            0
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { cur: self.first.clone() }
    }

    pub fn to_iter(mut self) -> Iter<T> {
        Iter { cur: _move_opt(&mut self.first) }
    }

}

pub fn to_iter<T: Clone>(s: Ref<List<T>>) -> Iter<T> {
    let mut s = s;
    Iter { cur: _move_opt(&mut s.first) }
}

pub struct Iter<T: Clone> {
    cur: Option<Ref<Node<T>>>,
}

impl <T: Clone> Iterator for Iter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let mut new_cur = None;
        let mut ret = None;
        if let Some(ref mut cur) = self.cur {
            //FIXME get rid of clone!
            ret = cur.elt.clone();
            new_cur = replace(&mut cur.next, None);
        }
        self.cur = new_cur;
        ret
    }
}

#[test]
fn lst_len() {
    println!("=== LST_LEN ==============");
    let mut lst = Ref::new(List::new());
    assert_eq!(0, lst.len());

    lst.prepend(1);
    assert_eq!(1, lst.len());

    lst.prepend(2);
    assert_eq!(2, lst.len());

    lst.prepend(3);
    assert_eq!(3, lst.len());

    lst.pop_front();
    assert_eq!(2, lst.len());
}

#[test]
fn lst_pop_front() {
    println!("=== LST_LEN ==============");
    let mut lst = Ref::new(List::new());
    lst.prepend(3);
    lst.prepend(2);
    lst.prepend(1);

    assert_eq!(Some(1), lst.pop_front());
    assert_eq!(Some(2), lst.pop_front());
    assert_eq!(Some(3), lst.pop_front());
    assert_eq!(None, lst.pop_front());
    assert_eq!(None, lst.pop_front());
}

#[test]
fn lst_pop_back() {
    println!("=== LST_LEN ==============");
    let mut lst = Ref::new(List::new());
    lst.prepend(3);
    lst.prepend(2);
    lst.prepend(1);

    assert_eq!(Some(3), lst.pop_back());
    assert_eq!(Some(2), lst.pop_back());
    assert_eq!(Some(1), lst.pop_back());
    assert_eq!(None, lst.pop_front());
    assert_eq!(None, lst.pop_front());
}

#[test]
fn lst_copy() {
    println!("=== LST_COPY ==============");
    let mut lst1 = Ref::new(List::new());
    lst1.prepend(1);
    let lst2 : Ref<List<i32>>;
    lst1.prepend(2);

    lst2 = lst1.clone();

    lst1.prepend(3);

    assert!(3 == lst1.len());
    assert!(2 == lst2.len());
}

#[test]
fn lst_iter() {
    let mut lst1 = Ref::new(List::new());
    lst1.prepend(3);
    lst1.prepend(2);
    lst1.prepend(1);

    let mut cnt = 1;
    for v in lst1.iter() {
        println!("v={}, cnt={}", v, cnt);
        assert_eq!(v, cnt);
        cnt = cnt+1;
    }

    let mut cnt = 1;
    for v in to_iter(lst1) {
        println!("v={}, cnt={}", v, cnt);
        assert_eq!(v, cnt);
        cnt = cnt+1;
    }
}
