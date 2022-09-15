
pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self,item: T) {
        let newnode = Box::new(Node{
            elem: item,
            next: self.head.take(),
        });
        self.head = Some(newnode);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
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