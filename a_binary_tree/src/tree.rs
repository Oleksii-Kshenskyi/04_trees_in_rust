use std::cmp::{Eq, Ord};
use std::hash::Hash;
use std::boxed::Box;
use std::cmp::Ordering;

enum DeletionType {
    LeafNode,
    HasLeftChild,
    HasRightChild,
    HasBothChildren,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum NodeStatus {
    IsLeftChild,
    IsRoot,
    IsRightChild,
}

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
    status: NodeStatus,
    right_child: Option<Box<Node<K, V>>>,
    left_child: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V>
where 
    K: Eq + Ord + Hash + Clone,
    V: Clone {
        fn determine_deletion_type(&self) -> DeletionType {
            let childrenvec: Vec<Option<Box<Node<K, V>>>> = vec![self.left_child.clone(), self.right_child.clone()];
            if childrenvec.iter().all(|x| { x.is_none() }) {
                DeletionType::LeafNode
               }
            else if childrenvec.iter().all(|x| { x.is_some() }) {
                DeletionType::HasBothChildren
            }
            else if self.left_child.is_some() && self.right_child.is_none() {
                DeletionType::HasLeftChild
            }
            else if self.right_child.is_some() && self.left_child.is_none() {
                DeletionType::HasRightChild
            }
            else {
                unreachable!("BinaryTree::determine_deletion_type(): mistake in the algorithm?")
            }
        }
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

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
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
        self.insert_internal(&mut self.root.clone(), &mut NodeStatus::IsRoot, key, value);
    }
    fn insert_internal(&mut self, root: &mut Option<Box<Node<K, V>>>, status: &mut NodeStatus, key: K, value: V) {
        if root.is_none() {
            BinaryTree::create_node_at(root, status.clone(), key, value);
            return;
        }

        let mut boxclone = root.clone().unwrap();
        let mutnode = boxclone.as_mut();
        match mutnode.key.cmp(&key) {
            Ordering::Less => {
                self.insert_internal(&mut mutnode.left_child, &mut NodeStatus::IsLeftChild, key, value);
            },
            Ordering::Equal => {
                BinaryTree::create_node_at(root, status.clone(), key, value);
            },
            Ordering::Greater => {
                self.insert_internal(&mut mutnode.right_child, &mut NodeStatus::IsRightChild, key, value);
            }
        };
    }

    pub fn delete(&mut self, key: &K) -> Result<(), ()> {
        self.delete_internal(&mut self.root.clone(), &mut None, key)
    }
    fn delete_internal(&mut self, root: &mut Option<Box<Node<K, V>>>, parent: &mut Option<Box<Node<K, V>>>, key: &K) -> Result<(), ()> {
        if root.is_none() {
            return Err(());
        }

        let mut boxclone = root.clone().unwrap();
        let mutnode = boxclone.as_mut();
        match mutnode.key.cmp(&key) {
            Ordering::Less => {
                return self.delete_internal(&mut mutnode.left_child, &mut self.root.clone(), key);
            },
            Ordering::Equal => {
                self.perform_deletion(&mut root.clone(), parent);
                return Ok(());
            },
            Ordering::Greater => {
                return self.delete_internal(&mut mutnode.right_child, &mut self.root.clone(), key);
            }
        }
    }
    fn perform_deletion(&mut self, delete_me: &mut Option<Box<Node<K, V>>>, parent: &mut Option<Box<Node<K, V>>>) {
        let delete_me_node = *delete_me.clone().unwrap();
        match delete_me_node.determine_deletion_type() {
            DeletionType::LeafNode => {
                match delete_me_node.status {
                    NodeStatus::IsLeftChild => {
                        let mut parent_box = parent.clone().unwrap();
                        let parent_node = parent_box.as_mut();

                        parent_node.left_child = None;

                        *delete_me = None;
                    },
                    NodeStatus::IsRoot => {
                        *delete_me = None;
                    },
                    NodeStatus::IsRightChild => {
                        let mut parent_box = parent.clone().unwrap();
                        let parent_node = parent_box.as_mut();

                        parent_node.right_child = None;

                        *delete_me = None;
                    },
                }
            },
            DeletionType::HasLeftChild => {
                match delete_me_node.status {
                    NodeStatus::IsLeftChild => {
                        let mut parent_box = parent.clone().unwrap();
                        let parent_node = parent_box.as_mut();
                        let delete_me_left_child = delete_me.clone().unwrap().left_child;

                        parent_node.left_child = delete_me_left_child;

                        *delete_me = None;
                    },
                    NodeStatus::IsRoot => {
                        let delete_me_left_child = delete_me.clone().unwrap().left_child;

                        self.root = delete_me_left_child;

                        *delete_me = None;
                    },
                    NodeStatus::IsRightChild => {
                        let mut parent_box = parent.clone().unwrap();
                        let parent_node = parent_box.as_mut();
                        let delete_me_left_child = delete_me.clone().unwrap().left_child;

                        parent_node.right_child = delete_me_left_child;

                        *delete_me = None;
                    },
                }
            },
            DeletionType::HasRightChild => {
                match delete_me_node.status {
                    NodeStatus::IsLeftChild => {
                        let mut parent_box = parent.clone().unwrap();
                        let parent_node = parent_box.as_mut();
                        let delete_me_right_child = delete_me.clone().unwrap().right_child;

                        parent_node.left_child = delete_me_right_child;

                        *delete_me = None;
                    },
                    NodeStatus::IsRoot => {
                        let delete_me_right_child = delete_me.clone().unwrap().right_child;

                        self.root = delete_me_right_child;

                        *delete_me = None;
                    },
                    NodeStatus::IsRightChild => {
                        let mut parent_box = parent.clone().unwrap();
                        let parent_node = parent_box.as_mut();
                        let delete_me_right_child = delete_me.clone().unwrap().right_child;

                        parent_node.right_child = delete_me_right_child;

                        *delete_me = None;
                    },
                }
            },
            DeletionType::HasBothChildren => {
                match delete_me_node.status {
                    NodeStatus::IsLeftChild => {
                        let mut parent_box = parent.clone().unwrap();
                        let parent_node = parent_box.as_mut();
                        let delete_me_left_child = delete_me.clone().unwrap().left_child;
                        let delete_me_right_child = delete_me.clone().unwrap().right_child;

                        parent_node.left_child = delete_me_left_child.clone();

                        let mut dmlc_clone = delete_me_left_child.clone();
                        let dmlc_node = dmlc_clone.as_mut().unwrap().as_mut();

                        dmlc_node.right_child = delete_me_right_child;

                        *delete_me = None;
                    },
                    NodeStatus::IsRoot => {
                        let delete_me_box = delete_me.clone().unwrap();
                        let delete_me_node = delete_me_box.as_ref();
                        let delete_me_left_child = delete_me_node.left_child.clone();
                        let delete_me_right_child = delete_me_node.right_child.clone();

                        self.root = delete_me_left_child.clone();

                        let mut dmlc_clone = delete_me_left_child.clone();
                        let dmlc_node = dmlc_clone.as_mut().unwrap().as_mut();

                        dmlc_node.right_child = delete_me_right_child;

                        *delete_me = None;
                    },
                    NodeStatus::IsRightChild => {
                        let mut parent_box = parent.clone().unwrap();
                        let parent_node = parent_box.as_mut();
                        let delete_me_right_child = delete_me.clone().unwrap().right_child;
                        let delete_me_left_child = delete_me.clone().unwrap().left_child;

                        parent_node.right_child = delete_me_right_child.clone();

                        let mut dmrc_clone = delete_me_right_child.clone();
                        let dmrc_node = dmrc_clone.as_mut().unwrap().as_mut();

                        dmrc_node.left_child = delete_me_left_child;

                        *delete_me = None;
                    },
                }
            },
        }
    }


    fn create_node_at(at_root: &mut Option<Box<Node<K, V>>>, status: NodeStatus, key: K, value: V) {
        *at_root = Some(
            Box::new(
                Node {
                    key,
                    value,
                    status,
                    left_child: None,
                    right_child: None,
                }
            )
        )
    }
}