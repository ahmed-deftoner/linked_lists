use std::mem;

pub struct List {
    head: Link
}

type Link = Option<Box<Node>>;

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
            next: self.head.take(),
        });
        self.head = Some(newnode);
    }

    pub fn pop(&mut self) -> Option<i32>{
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic(){
        let mut list = List::new();
        assert_eq!(list.pop(),None);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(),Some(3));
        assert_eq!(list.pop(),Some(2));
        assert_eq!(list.pop(),Some(1));
        assert_eq!(list.pop(),None);

    }
}