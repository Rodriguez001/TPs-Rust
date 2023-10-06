use std::fmt;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

struct List<T> {
    head: Link<T>,
}

pub struct Iter<'a, T: 'a> {
    next: &'a Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push_front(&mut self, elem: T) {
        self.head = Some(Box::new(Node {
            elem: elem,
            next: self.head.take(),
        }));
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current_node = self.head.take();

        while let Some(mut current_node_inner) = current_node.take() {
            let next = current_node_inner.next.take();
            current_node_inner.next = prev.take();
            prev = Some(current_node_inner);
            current_node = next;
        }

        self.head = prev.take();
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: &self.head }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.as_ref() {
            Some(next) => {
                self.next = &next.next;
                Some(&next.elem)
            }
            None => None,
        }
    }

    
}

impl<T> fmt::Debug for List<T>
    where T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for item in self {
            write!(f, "{:?} ", item)?;
        }
        Ok(())
    }
}

fn first<T>(mut it: Iter<T>) -> Option<&T> {
    return it.next();
}

fn last<T>(it: Iter<T>) -> Option<&T> {
    let mut last = None;
    for i in it {
        last = Some(i);
    }
    last
}

fn nth_element<T>(it: Iter<T>, index: usize) -> Option<&T> {
    let mut last = None;
    let mut cpt = 1;
    for i in it {
        if cpt == index {
            last = Some(i);
        }
        cpt += 1;
    }
    last
}

fn main() {
    let mut l = List::new();
    l.push_front(3);
    l.push_front(2);
    l.push_front(1);
    l.push_front(12);

    println!("{:?}", first(l.iter()).unwrap());
    println!("{:?}", nth_element(l.iter(),2).unwrap());
    println!("{:?}", last(l.iter()).unwrap());
    for item in l.iter() {
        print!("{} ", item);
    }
    println!();
    l.reverse();
    for item in l.iter() {
        print!("{} ", item);
    }
    println!();
}