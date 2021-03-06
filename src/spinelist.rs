pub use giftr::refs::functional::Ref as Ref;
use giftr::ispine::*;
use giftr::ispine::contiguous::Contiguous as Spine;
use std::default::Default;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct SpineList<T: Clone> {
    spine : Spine<T>,
}

impl <T: Clone+Debug> SpineList<T> {
    pub fn new() -> SpineList<T> {
        SpineList { spine: Default::default() }
    }

    pub fn prepend(&mut self, x: T) {
        self.spine.add(x);
    }

    pub fn insert(&mut self, idx: usize, x: T) {
        self.spine.at()
            .skip(idx).next().unwrap().insert(x);
    }

    pub fn append(&mut self, x: T) {
        if let Some(ref mut l) = self.spine.at().last() {
            l.insert(x);
            return
        }
        self.spine.add(x)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(x) = self.spine.pop() {
            println!("popped {:?}, len was {}", x, self.len());
            Some(x)
        } else {
            None
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let len = self.len();
        if let Some(x) = self.spine.take_from(len-1).pop() {
            Some(x)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.spine.iter().count()
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { cur: self.spine.clone() }
    }

    pub fn to_iter(self) -> Iter<T> {
        let SpineList { spine } = self;
        Iter { cur: spine }
    }
}

pub struct Iter<T: Clone> {
    cur: Spine<T>,
}

impl <T: Clone> Iterator for Iter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.cur.pop()
    }
}

mod test {
    use giftr::refs::*;
    use super::{Ref, SpineList};
    #[test]
    fn lst_len() {
        println!("=== LST_LEN ==============");
        let mut lst = Ref::new(SpineList::<i32>::new());
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
        let mut lst = Ref::new(SpineList::new());
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
    fn lst_append() {
        let mut lst = SpineList::<i32>::new();
        lst.append(1);
        lst.append(2);
        lst.append(3);

        assert_eq!(Some(1), lst.pop_front());
        assert_eq!(Some(2), lst.pop_front());
        assert_eq!(Some(3), lst.pop_front());
        assert_eq!(None,    lst.pop_front());
        assert_eq!(None,    lst.pop_front());
    }



    #[test]
    fn lst_pop_back() {
        println!("=== LST_LEN ==============");
        let mut lst = Ref::new(SpineList::new());
        lst.prepend(3);
        lst.prepend(2);
        lst.prepend(1);
        println!("lst= {:?}", lst);

        assert_eq!(Some(3), lst.pop_back());
        assert_eq!(Some(2), lst.pop_back());
        assert_eq!(Some(1), lst.pop_back());
        assert_eq!(None, lst.pop_front());
        assert_eq!(None, lst.pop_front());
    }

    #[test]
    fn lst_copy() {
        println!("=== LST_COPY ==============");
        let mut lst1 = Ref::new(SpineList::new());
        lst1.prepend(1);
        let lst2 : Ref<SpineList<i32>>;
        lst1.prepend(2);

        lst2 = lst1.clone();

        lst1.prepend(3);

        assert!(3 == lst1.len());
        assert!(2 == lst2.len());
    }

    #[test]
    fn lst_iter() {
        let mut lst1 = Ref::new(SpineList::new());
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
    use super::{Ref, SpineList};

    #[bench]
    fn lst_append(b: &mut Bencher) {
        let mut lst1 : Ref<SpineList<i32>> = Ref::new(SpineList::new());
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
        let mut lst1 : SpineList<i32> = SpineList::new();
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
    fn lst_insert_5000(b: &mut Bencher) {
        let mut lst1 : SpineList<i32> = SpineList::new();
        let size = 100000;
        for i in 0..size {
            lst1.append(i);
        }
        b.iter(
            || {
                test::black_box(lst1.insert(50000, 1));
            }
        );
    }

    #[bench]
    fn lst_len(b: &mut Bencher) {
        let mut lst1 : SpineList<i32> = SpineList::new();
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

#[cfg(test)]
mod vecbench {
    use test;
    use test::Bencher;

    #[bench]
    fn vec_append(b: &mut Bencher) {
        let mut vec1  = Vec::new();
        let size = 10000;
        for i in 0..size {
            vec1.push(i);
        }
        b.iter(
            || {
                test::black_box(vec1.push(1));
            }
        );
    }

    #[bench]
    fn vec_prepend(b: &mut Bencher) {
        let mut vec1 = Vec::new();
        let size = 10000;
        for i in 0..size {
            vec1.push(i);
        }
        b.iter(
            || {
                test::black_box(vec1.insert(0, 1));
            }
        );
    }

    #[bench]
    fn vec_insert_5000(b: &mut Bencher) {
        let mut lst1 = Vec::new();
        let size = 100000;
        for i in 0..size {
            lst1.push(i);
        }
        b.iter(
            || {
                test::black_box(lst1.insert(50000, 1));
            }
        );
    }

    #[bench]
    fn vec_len(b: &mut Bencher) {
        let mut vec1 = Vec::new();
        let size = 10000;
        for i in 0..size {
            vec1.push(i);
        }
        b.iter(
            || {
                test::black_box(vec1.len());
            }
        );
    }

}
