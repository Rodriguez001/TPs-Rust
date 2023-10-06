use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    v: T,
    p: Link<T>,
    n: Link<T>,
}

struct DLinkedList<T> {
    h: Link<T>,
    t: Link<T>,
}

impl<T> DLinkedList<T> {
    fn new() -> Self {
        DLinkedList { h: None, t: None }
    }

    fn push_front(&mut self, v: T) {
        let new_node = Node {
            v,
            p: None,
            n: None,
        };
        let new_node = Rc::new(RefCell::new(new_node));

        match self.t.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().n = Some(new_node.clone());
                new_node.borrow_mut().p = Some(old_tail);
                self.t = Some(new_node);
            },
            None => {
                self.h = Some(new_node.clone());
                self.t = Some(new_node);
            }
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.h.take().map(|old_head| {
            match old_head.borrow_mut().n.take() {
                Some(new_head) => {
                    new_head.borrow_mut().p = None;
                    self.h = Some(new_head);
                },
                None => {
                    self.t.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().v
        })
    }
}

fn main() {
    let mut list = DLinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    println!("{:?}", list);
    while let Some(val) = list.pop_front() {
        println!("{}", val);
    }
}
