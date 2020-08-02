use std::mem;

#[derive(Debug, Eq, PartialEq)]
struct List<T: Eq>(Option<Box<Node<T>>>);

#[derive(Debug, Eq, PartialEq)]
struct Node<T: Eq> {
    data: T,
    next: List<T>,
}

impl<T: Eq> List<T> {
    pub fn new() -> Self {
        List(None)
    }

    pub fn insert(&mut self, data: T) {
        let old_list = mem::replace(&mut self.0, None);
        let new_node = Some(Box::new(Node {
            data,
            next: List(old_list),
        }));
        mem::replace(&mut self.0, new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let old_head = self.0.take();
        match old_head {
            None => None,
            Some(node) => {
                mem::replace(self, node.next);
                Some(node.data)
            }
        }
    }

    pub fn includes(&self, data: T) -> bool {
        match &self.0 {
            None => false,
            Some(node) => {
                if node.data == data {
                    return true;
                }
                node.next.includes(data)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    pub fn insert_and_remove_list() {
        let mut list = List::<i32>::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
    }

    #[test]
    pub fn includes() {
        let mut list = List::<i32>::new();
        list.insert(2);
        list.insert(4);
        list.insert(6);
        list.insert(8);
        list.insert(10);
        list.insert(12);

        assert!(list.includes(8));
        list.pop();
        list.pop();
        list.pop();
        assert!(!list.includes(8));
    }
}
