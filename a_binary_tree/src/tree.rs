use std::cmp::{Eq, Ord};
use std::hash::Hash;
use std::boxed::Box;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct BinaryTree<K, V> 
where K: Eq + Ord + Hash + Clone,
      V: Clone {
    root: Option<Box<Node<K, V>>>
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Node<K, V> 
where
    K: Eq + Ord + Hash + Clone,
    V: Clone {
    key: K,
    value: V,
    right_child: Option<Box<Node<K, V>>>,
    left_child: Option<Box<Node<K, V>>>,
}

impl<K, V> BinaryTree<K, V> 
where 
    K: Eq + Ord + Hash + Clone,
    V: Clone {
    pub fn new() -> Self {
        Self {
            root: None
        }
    }

    pub fn find(&self, key: &K) -> Option<V> {
        self.find_internal(self.root.clone(), key)
    }
    fn find_internal(&self, root: Option<Box<Node<K, V>>>, key: &K) -> Option<V> {
        if root.is_none() {
            None
        } else {
            let boxclone = root.as_ref().unwrap().clone();
            match boxclone.key.cmp(&key) {
                Ordering::Less => {
                    self.find_internal(root.unwrap().left_child, key)
                },
                Ordering::Equal => {
                    Some(root.unwrap().value)
                },
                Ordering::Greater => {
                    self.find_internal(root.unwrap().right_child, key)
                },
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.insert_internal(&mut self.root.clone(), key, value);
    }
    fn insert_internal(&mut self, root: &mut Option<Box<Node<K, V>>>, key: K, value: V) {
        if root.is_none() {
            BinaryTree::create_node_at(root, key, value);
            return;
        }

        let mut boxclone = root.clone().unwrap();
        let mutnode = boxclone.as_mut();
        match mutnode.key.cmp(&key) {
            Ordering::Less => {
                self.insert_internal(&mut mutnode.left_child, key, value);
            },
            Ordering::Equal => {
                BinaryTree::create_node_at(root, key, value);
            },
            Ordering::Greater => {
                self.insert_internal(&mut mutnode.right_child, key, value);
            }
        };
    }

    pub fn remove(&mut self, key: &K) -> Result<(), ()> {
        let _ = key;
        Ok(())
    }


    fn create_node_at(at_root: &mut Option<Box<Node<K, V>>>, key: K, value: V) {
        *at_root = Some(
            Box::new(
                Node {
                    key,
                    value,
                    left_child: None,
                    right_child: None,
                }
            )
        )
    }
}