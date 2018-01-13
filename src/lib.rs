#![feature(nll)]

#[derive(Debug)]
pub struct BTree {
    minimum_degree: usize,
    root: Option<BTreeNode>,
}

impl BTree {
    pub fn new(minimum_degree: usize) -> BTree {
        BTree{
            root: None,
            minimum_degree: minimum_degree,
        }
    }

    pub fn search(&self, key: usize) -> Option<&BTreeNode> {
        match self.root {
            None => None,
            Some(ref root) => root.search(key),
        }
    }

    pub fn insert(&mut self, key: usize) {
        match self.root {
            None => {
                let mut node = BTreeNode::new(self.minimum_degree, true);
                node.insert_key(key);
                self.root = Some(node);
            },
            Some(ref mut root) => {
                // Create a new root if filled
                if root.is_full() {
                    // rendundant code here to get around rust's ownership checks
                    let mut left = BTreeNode::new(root.minimum_degree, true);
                    let mut right = BTreeNode::new(root.minimum_degree, true);

                    let mut index = 0;
                    while index < root.capacity() {
                        let k = root.remove_key(0);
                        if index < self.minimum_degree -1 {
                            left.insert_key(k);
                        } else if index > self.minimum_degree - 1{
                            right.insert_key(k);
                        } else {
                            root.num_keys += 1;
                            root.keys.push(k);
                        }
                        index += 1;
                    }
                    root.children.push(left);
                    root.children.push(right);
                    root.is_leaf = false;
                }

                let mut x = root;
                loop {
                    if x.is_leaf {
                        break;
                    }

                    let mut i = 0;
                    while i < x.num_keys && key > x.keys[i] {
                        i += 1;
                    }

                    if x.children[i].is_full() {
                        // Split the node if it's full
                        let child = x.children.remove(i);
                        let (k, lnode, rnode) = child.split();
                        let inserted_index = x.insert_key(k);
                        x.children.insert(inserted_index, lnode);
                        x.children.insert(inserted_index + 1, rnode);
                    } else {
                        x = &mut x.children[i];
                    }
                }

                // Insert key
                x.insert_key(key);
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct BTreeNode {
    minimum_degree: usize,
    num_keys: usize,
    keys: Vec<usize>,
    children: Vec<BTreeNode>,
    is_leaf: bool,
}

impl BTreeNode {
    fn new(minimum_degree: usize, is_leaf: bool) -> BTreeNode {
        BTreeNode{
            minimum_degree: minimum_degree,
            num_keys: 0,
            keys: Vec::with_capacity(minimum_degree * 2 - 1),
            children: Vec::with_capacity(minimum_degree * 2),
            is_leaf: is_leaf,
        }
    }

    fn capacity(&self) -> usize {
        self.minimum_degree * 2 - 1
    }

    fn insert_key(&mut self, key: usize) -> usize {
        let mut i = 0;
        while i < self.num_keys && key > self.keys[i] {
            i += 1;
        }
        self.num_keys += 1;
        self.keys.insert(i, key);
        return i;
    }

    fn remove_key(&mut self, index: usize) -> usize {
        self.num_keys -= 1;
        return self.keys.remove(index);
    }


    fn is_full(&self) -> bool {
        self.keys.len() == self.minimum_degree * 2 - 1
    }

    fn split(self) -> (usize, BTreeNode, BTreeNode) {
        let key: usize = self.keys[self.minimum_degree-1];
        let mut left = BTreeNode::new(self.minimum_degree, self.is_leaf);
        let mut right = BTreeNode::new(self.minimum_degree, self.is_leaf);

        for (index, k) in self.keys.iter().enumerate() {
            if index < self.minimum_degree - 1 {
                left.insert_key(*k);
            } else if index > self.minimum_degree - 1 {
                right.insert_key(*k);
            }
        }

        let mut index = 0;
        for child in self.children {
            if index < self.minimum_degree {
                left.children.push(child);
            } else {
                right.children.push(child);
            }
            index += 1;
        }

        return (key, left, right);
    }

    fn search(&self, key: usize) -> Option<&BTreeNode> {
        // Find the first key greater than or equal to k
        let mut i = 0;
        while i < self.num_keys && key > self.keys[i] {
            i += 1;
        }

        // If key is on this node, return the node
        if self.keys[i] == key {
            return Some(self);
        }

        if self.is_leaf == true {
            return None;
        }

        return self.children[i].search(key);
    }
}

#[cfg(test)]
mod tests {
    use super::BTree;

    #[test]
    fn it_has_ordered_inserts() {
        let mut btree = BTree::new(3);
        btree.insert(2);
        btree.insert(1);
        btree.insert(3);
        assert_eq!(btree.root.unwrap().keys, vec![1, 2, 3]);
    }

    #[test]
    fn it_splits_the_root() {
        let mut btree = BTree::new(3);
        btree.insert(1);
        btree.insert(2);
        btree.insert(3);
        btree.insert(4);
        btree.insert(5);
        btree.insert(6);
        let root = &btree.root.unwrap();
        assert_eq!(root.keys, vec![3]);
        assert_eq!(root.children[0].keys, vec![1, 2]);
        assert_eq!(root.children[1].keys, vec![4, 5, 6]);
    }

    #[test]
    fn it_splits_the_child() {
        let mut btree = BTree::new(3);
        btree.insert(1);
        btree.insert(2);
        btree.insert(3);
        btree.insert(4);
        btree.insert(5);
        btree.insert(6);
        btree.insert(7);
        btree.insert(8);
        btree.insert(9);
        let root = &btree.root.unwrap();
        assert_eq!(root.keys, vec![3, 6]);
        assert_eq!(root.children[0].keys, vec![1, 2]);
        assert_eq!(root.children[1].keys, vec![4, 5]);
        assert_eq!(root.children[2].keys, vec![7, 8, 9]);
    }
}
