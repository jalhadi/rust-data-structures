use std::mem;

#[derive(Debug, PartialEq)]
struct BST<T: Ord>(Option<Box<Node<T>>>);

#[derive(Debug, PartialEq)]
struct Node<T: Ord> {
    data: T,
    left_child: BST<T>,
    right_child: BST<T>,
}

impl<T: Ord + Copy> BST<T> {
    pub fn new() -> Self {
        BST(None)
    }

    pub fn push(&mut self, data: T) {
        match &mut self.0 {
            None => {
                self.0 = Some(Box::new(Node {
                    data,
                    left_child: BST::new(),
                    right_child: BST::new(),
                }));
            }
            Some(node) => {
                if node.data == data {
                    return;
                }
                let target_node = if data < node.data {
                    &mut node.left_child
                } else {
                    &mut node.right_child
                };
                target_node.push(data);
            }
        }
    }

    pub fn find(&self, data: T) -> bool {
        match &self.0 {
            None => false,
            Some(node) => {
                if node.data == data {
                    return true;
                }
                let target_node = if data < node.data {
                    &node.left_child
                } else {
                    &node.right_child
                };
                target_node.find(data)
            }
        }
    }

    fn find_and_remove_largest(&mut self) -> Option<T> {
        match &mut self.0 {
            None => None,
            Some(node) => {
                if node.right_child.0 != None {
                    node.right_child.find_and_remove_largest()
                } else {
                    let data = Some(node.data);
                    let a = mem::replace(&mut node.left_child, BST(None));
                    *self = a;
                    data
                }
            }
        }
    }

    fn find_and_remove_smallest(&mut self) -> Option<T> {
        match &mut self.0 {
            None => None,
            Some(node) => {
                if node.left_child.0 != None {
                    node.right_child.find_and_remove_smallest()
                } else {
                    let data = Some(node.data);
                    let a = mem::replace(&mut node.right_child, BST(None));
                    *self = a;
                    data
                }
            }
        }
    }

    pub fn remove(&mut self, data: T) -> bool {
        match &mut self.0 {
            None => false,
            Some(node) => {
                if node.data == data {
                    if node.right_child.0 != None {
                        match node.right_child.find_and_remove_smallest() {
                            Some(data) => {
                                node.data = data;
                            }
                            _ => {}
                        }
                    } else if node.left_child.0 != None {
                        match node.left_child.find_and_remove_largest() {
                            Some(data) => {
                                node.data = data;
                            }
                            _ => {}
                        }
                    } else {
                        self.0 = None;
                    }
                    return true;
                }

                let target_node = if data < node.data {
                    &mut node.left_child
                } else {
                    &mut node.right_child
                };
                target_node.remove(data)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Node;
    use super::BST;

    #[test]
    fn create_new_bst() {
        let mut bst = BST::new();

        bst.push(5);
        bst.push(5);
        bst.push(10);
        bst.push(-1);
        assert_eq!(
            bst,
            BST(Some(Box::new(Node {
                data: 5,
                left_child: BST(Some(Box::new(Node {
                    data: -1,
                    left_child: BST(None),
                    right_child: BST(None),
                }))),
                right_child: BST(Some(Box::new(Node {
                    data: 10,
                    left_child: BST(None),
                    right_child: BST(None),
                }))),
            })))
        );
    }

    #[test]
    fn find_item() {
        let mut bst = BST::new();

        bst.push(3);
        bst.push(20);
        bst.push(-13);
        bst.push(77);

        assert!(bst.find(3));
        assert!(bst.find(20));
        assert!(bst.find(-13));
        assert!(bst.find(77));

        assert!(!bst.find(5));
        assert!(!bst.find(78));
        assert!(!bst.find(101));
        assert!(!bst.find(-3));
    }

    #[test]
    fn remove_items() {
        let mut bst = BST::new();

        bst.push(3);
        bst.push(20);
        bst.push(-13);
        bst.push(123);
        bst.push(4);
        bst.push(5);
        bst.push(5);
        bst.push(7);

        bst.remove(20);
        bst.remove(5);
        bst.remove(7);

        assert_eq!(
            bst,
            BST(Some(Box::new(Node {
                data: 3,
                left_child: BST(Some(Box::new(Node {
                    data: -13,
                    left_child: BST(None),
                    right_child: BST(None),
                }))),
                right_child: BST(Some(Box::new(Node {
                    data: 123,
                    left_child: BST(Some(Box::new(Node {
                        data: 4,
                        left_child: BST(None),
                        right_child: BST(None),
                    }))),
                    right_child: BST(None),
                }))),
            })))
        );
    }
}
