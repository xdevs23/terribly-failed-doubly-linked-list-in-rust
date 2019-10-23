use std::rc::Rc;

struct LinkedList<'a, T: ?Sized> {
    all: Vec<LinkedListElement<'a, T>>,
    first: Option<usize>,
    last: Option<usize>,
    length: usize
}

struct LinkedListElement<'a, T: ?Sized> {
    id: usize,
    data: &'a T,
    has_prev: bool,
    has_next: bool,
    prev: Option<usize>,
    next: Option<usize>
}

impl<'a, T> LinkedList<'a, T> {
    pub fn add(self, data: &'a T) {
        let mut list = self;
        let elem = LinkedListElement {
            id: list.length,
            data: data,
            has_next: false,
            has_prev: false,
            next: None,
            prev: None
        };

        if elem.has_prev {
            let mut next = list.all[elem.prev.unwrap()].next;
            next.replace(next.unwrap() + 1);
        }

        if list.length == 0 {
            list.first.replace(elem.id);
        }
        list.last.replace(elem.id);
        list.length += 1;
    }

    pub fn get(self, index: usize) -> &'a T {
        return self.all[index].data;
    }
}

fn main() {
    println!("Hello, world!");
    let data = "test1";
    let list = LinkedList {
        first: None,
        last: None,
        length: 0,
        all: Vec::new()
    };

    list.add(&data);
    let newdata = list.get(0);
    println!("{}", *newdata);
    
}
