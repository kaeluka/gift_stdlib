use giftr::refs::*;
pub use giftr::refs::functional::Ref as Ref;
//pub use giftr::refs::imperative::Ref as Ref;
use std::default::Default;

use std::mem::replace;
use std::iter::Iterator;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Node<T: Clone> {
    next: Option<Ref<Node<T>>>,
    elt: Option<T>,
}

impl <T: Clone> Node<T> {
    fn new(x: T) -> Node<T> {
        Node { next: Default::default(), elt: Some(x) }
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
                ret = next.elt.take();
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

impl <T: Clone> Drop for Node<T> {
    fn drop(&mut self) {
        let mut optcur : Option<_> = self.next.take();
        while let Some(cur) = optcur {
            match Rc::try_unwrap(cur._ptr) {
                Ok(ref mut cur) => optcur = cur.next.take(), //nxt,
                Err(_)          => break
            }
        }
    }
}


#[derive(Clone, Debug)]
pub struct List<T: Clone> {
    len : i32,
    first : Option<Ref<Node<T>>>,
}

impl <T: Clone> List<T> {
    pub fn new() -> List<T> {
        List { len: 0, first: None }
    }

    pub fn prepend(&mut self, x: T) {
        self.len += 1;
        let new_next = replace(&mut self.first, None);
        let new_first = Node { elt: Some(x), next: new_next };
        self.first = Some(Ref::new(new_first));
    }

    pub fn append(&mut self, x:T) {
        self.len += 1;
        if let Some(ref mut node) = self.first {
            node.append(x)
        } else {
            self.first = Some(Ref::new(Node::new(x)))
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.len -= 1;
        let mut ret = None;
        let optfirst = self.first.take();
        if let Some(first) = optfirst {
            let mut n = first.into_inner();
            self.first = n.next.take();
            ret = n.elt.take();
        }
        ret
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.len -= 1;
        let mut ret = None;
        let mut first_is_last = false;
        if let Some(ref mut first) = self.first {
            if first.is_last() {
                println!("first is last");
                first_is_last = true;
                ret = first.elt.take();
            } else {
                println!("first is not last");
                ret = first.pop_back();
            }
        }
        if first_is_last {
            self.first = None;
        }
        ret
    }

    pub fn len(&self) -> i32 {
        self.len
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { cur: self.first.clone() }
    }

    pub fn to_iter(mut self) -> Iter<T> {
        Iter { cur: self.first.take() }
    }

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

mod test {
    use giftr::refs::*;
    use super::{Ref, List};
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
    }

}

#[cfg(test)]
mod bench {
    use test;
    use test::Bencher;
    use giftr::refs::*;
    use super::{Ref, List};

    #[bench]
    fn lst_append(b: &mut Bencher) {
        let mut lst1 : Ref<List<i32>> = Ref::new(List::new());
        let size = 10000;
        for i in 0..size {
            lst1.append(i);
        }
        b.iter(
            || {
                test::black_box(lst1.append(1));
            }
        );
    }

    #[bench]
    fn lst_prepend(b: &mut Bencher) {
        let mut lst1 : List<i32> = List::new();
        let size = 10000;
        for i in 0..size {
            lst1.append(i);
        }
        b.iter(
            || {
                test::black_box(lst1.prepend(1));
            }
        );
    }

    #[bench]
    fn lst_len(b: &mut Bencher) {
        let mut lst1 : List<i32> = List::new();
        let size = 10000;
        for i in 0..size {
            lst1.append(i);
        }
        b.iter(
            || {
                test::black_box(lst1.len());
            }
        );
    }

}

