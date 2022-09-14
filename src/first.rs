use std::mem;

pub struct List {
    head: Link
}

enum Link {
    Empty,
    More(Box<Node>)
}

struct Node {
    elem: i32,
    next: Link
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self,item: i32) {
        let newnode = Box::new(Node{
            elem: item,
            next: mem::replace(&mut self.head, Link::Empty)
        });
        self.head = Link::More(newnode);
    }

    pub fn pop(&mut self) -> Option<i32>{
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                return Some(node.elem);
            },
        }
    }
}