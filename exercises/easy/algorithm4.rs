/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match self.root {
            None => self.root = Some(Box::new(TreeNode::new(value))),
            Some(ref mut root) => {
                root.insert(value);
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        match self.root {
            None => false,
            Some(ref node) => node.search(value),
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match self.value.cmp(&value) {
            //Greater to right
            Ordering::Greater => match self.right {
                None => self.right = Some(Box::new(TreeNode::new(value))),
                Some(ref mut right_node) => {
                    right_node.insert(value);
                }
            },
            //less left
            Ordering::Less => match self.left {
                None => self.left = Some(Box::new(TreeNode::new(value))),
                Some(ref mut left_node) => {
                    left_node.insert(value);
                }
            },
            _ => {} //equal donothing.
        }
    }
    fn search(&self, value: T) -> bool {
        if self.value.eq(&value) {
            true
        } else {
            match self.value.cmp(&value) {
                Ordering::Less => match self.left {
                    None => false,
                    Some(ref left_node) => left_node.search(value),
                },
                Ordering::Equal => true,
                Ordering::Greater => match self.right {
                    None => false,
                    Some(ref right_node) => right_node.search(value),
                },
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


