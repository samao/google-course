pub fn btree_run() {
    let mut btree = BTree::new();
    btree.insert(2);
    btree.insert(1);
    btree.insert(2);
    btree.insert(3);
    let mut m = (0..=4).map(|i| i as i32).collect::<Vec<i32>>();
    while let Some(o) = m.pop() {
        btree.insert(o);
    }

    // let _a = dbg!(btree);
}

#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

#[derive(Debug)]
struct Node<T: Ord> {
    left: Subtree<T>,
    right: Subtree<T>,
    value: T,
}

#[derive(Debug)]
struct BTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            left: Subtree::new(),
            right: Subtree::new(),
            value,
        }
    }
}

impl<T: Ord> Subtree<T> {
    fn new() -> Self {
        Self(None)
    }

    fn insert(&mut self, value: T) {
        match self.0 {
            None => {
                self.0 = Some(Box::new(Node::new(value)));
            }
            Some(ref mut node) => {
                if value < node.value {
                    node.left.insert(value);
                } else if value > node.value {
                    node.right.insert(value);
                }
            }
        }
    }

    fn len(&self) -> i32 {
        match self.0 {
            None => 0,
            Some(ref node) => 1 + node.left.len() + node.right.len(),
        }
    }

    fn has(&self, value: &T) -> bool {
        match self.0 {
            None => false,
            Some(ref node) => {
                if value == &node.value {
                    true
                } else if value < &node.value {
                    node.left.has(value)
                } else {
                    node.right.has(value)
                }
            }
        }
    }
}

impl<T: Ord> BTree<T> {
    fn new() -> Self {
        Self {
            root: Subtree::new(),
        }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn len(&self) -> i32 {
        self.root.len()
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut btree = BTree::new();
        assert_eq!(btree.len(), 0);
        btree.insert(2);
        assert_eq!(btree.len(), 1);
        btree.insert(1);
        assert_eq!(btree.len(), 2);
        btree.insert(2);
        assert_eq!(btree.len(), 2);
    }

    #[test]
    fn has() {
        let mut btree = BTree::new();
        fn check_has(tree: &BTree<i32>, exp: &[bool]) {
            let got: Vec<bool> = (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&btree, &[false, false, false, false, false]);
        btree.insert(0);
        check_has(&btree, &[true, false, false, false, false]);
        btree.insert(4);
        check_has(&btree, &[true, false, false, false, true]);
        btree.insert(4);
        check_has(&btree, &[true, false, false, false, true]);
        btree.insert(3);
        check_has(&btree, &[true, false, false, true, true])
    }

    #[test]
    fn unbalanced() {
        let mut btree = BTree::new();
        for i in 0..100 {
            btree.insert(i);
        }
        assert_eq!(btree.len(), 100);
        assert!(btree.has(&50));
    }
}
