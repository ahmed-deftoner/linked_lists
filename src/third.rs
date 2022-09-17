use std::rc::Rc;

pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> { 
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&mut self,item: T) -> List<T> {
        List { head: Some(Rc::new(Node{
            elem: item,
            next: self.head.clone(),
        }))}
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }


    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_deref() }
    }

}