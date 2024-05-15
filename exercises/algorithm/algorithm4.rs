/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DONE
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
    T: Ord+Clone,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    // 每个节点的值都大于其左子树中所有节点的值。
    // 每个节点的值都小于其右子树中所有节点的值。
    // 每个节点的左、右子树也是二叉搜索树。
    fn insert(&mut self, value: T) {
        //TODO
        let node = Box::new(TreeNode::new(value));
        match &self.root {
            None => {
                self.root = Some(node);
            },
            _ => {
                let tmp = self.root.as_mut();
                Self::insert_node(self.root.as_mut().unwrap(), node);
            }
        }
    }

    fn insert_node(current: &mut Box<TreeNode<T>>, new_node: Box<TreeNode<T>>){
        if new_node.value < current.value {
            match current.left {
                None => current.left = Some(new_node),
                _ => Self::insert_node(current.left.as_mut().unwrap(), new_node),
            }
        }
        else if new_node.value > current.value {
            match current.right {
                None => current.right = Some(new_node),
                _ => Self::insert_node(current.right.as_mut().unwrap(), new_node),
            }
        }
        else {
            return ;
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match self.root {
            None => false,
            _ => Self::sub_search(self.root.as_ref().unwrap(), value.clone()),
        }
    }
    fn sub_search(current_node: &Box<TreeNode<T>>, value: T) -> bool {
        if current_node.value == value {
            true
        }
        else {
            let mut l = false;
            let mut r = false;
            match current_node.left {
                None => l=false,
                _ => l=Self::sub_search(current_node.left.as_ref().unwrap(), value.clone()),
            }
            match current_node.right {
                None => r=false,
                _ => r=Self::sub_search(current_node.right.as_ref().unwrap(), value.clone()),
            }
            l||r
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
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


