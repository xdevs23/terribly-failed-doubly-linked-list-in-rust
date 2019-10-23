
use std::rc::Rc;

struct LinkedList<'a, T: ?Sized> {
    first: Option<LinkedListElement<'a, T>>,
    last: Option<LinkedListElement<'a, T>>,
    length: i128
}

struct LinkedListElement<'a, T: ?Sized> {
    prev: Option<&'a LinkedListElement<'a, T>>,
    next: Option<&'a LinkedListElement<'a, T>>,
    data: Option<&'a T>,
}

impl<'a, T: ?Sized> LinkedListElement<'a, T> {
    pub fn new() -> LinkedListElement<'a, T> {
        LinkedListElement {
            prev: None,
            next: None,
            data: None
        }
    }

    pub fn set_prev(&mut self, elem: LinkedListElement<'a, T>) {
        self.prev.replace(Rc::new(elem));
    }

    pub fn set_next(&mut self, elem: LinkedListElement<'a, T>) {
        self.prev.replace(Rc::new(elem));
    }
}

impl<'a, T: ?Sized> LinkedList<'a, T> {
    pub fn new() -> LinkedList<'a, T> {
        LinkedList {
            first: None,
            last: None,
            length: 0
        }
    }

    pub fn add(&self, data: &'a T) {
        let mut elem = LinkedListElement::new();
        elem.prev = self.last;
        elem.data = Some(data);

        if elem.prev.is_none() {
            elem.prev.unwrap().set_next(elem);
        }
        if self.length == 0 {
            self.first.replace(elem);
        }
    }

    pub fn set_last(&self, elem: LinkedListElement<'a, T>) {
        self.last.replace(elem);
    }

}

fn main() {
    println!("Hello, world!");
    let mut list = LinkedList::new();
    list.add(&"test1");
}
