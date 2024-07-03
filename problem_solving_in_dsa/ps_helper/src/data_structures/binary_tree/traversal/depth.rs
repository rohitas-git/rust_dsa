use std::fmt::Debug;
use std::marker::PhantomData;

use super::TraversalStrategy;
use super::TreeNode;

#[derive(Clone, Debug, Default)]
struct PreOrderTraversal<DataType>(PhantomData<DataType>);

impl<DataType> TraversalStrategy<DataType> for PreOrderTraversal<DataType>
where
    DataType: Clone + Default + Debug,
{
    // Root > LEFT > RIGHT
    fn traverse(&self, node: Option<TreeNode<DataType>>) {
        if let Some(node) = node {
            let node = node.borrow();
            println!("{:?}", node.key());
            self.traverse(node.left_child().clone());
            self.traverse(node.right_child().clone());
        }
    }
}

#[derive(Clone, Debug, Default)]
struct PostOrderTraversal<DataType>(PhantomData<DataType>);

impl<DataType> TraversalStrategy<DataType> for PostOrderTraversal<DataType>
where
    DataType: Clone + Default + Debug,
{
    // LEFT > RIGHT > Root
    fn traverse(&self, node: Option<TreeNode<DataType>>) {
        if let Some(node) = node {
            let node = node.borrow();
            self.traverse(node.left_child().clone());
            self.traverse(node.right_child().clone());
            println!("{:?}", node.key());
        }
    }
}

#[derive(Clone, Debug, Default)]
struct InOrderTraversal<DataType>(PhantomData<DataType>);

impl<DataType> TraversalStrategy<DataType> for InOrderTraversal<DataType>
where
    DataType: Clone + Default + Debug,
{
    // LEFT > Root > RIGHT
    fn traverse(&self, node: Option<TreeNode<DataType>>) {
        if let Some(node) = node {
            let node = node.borrow();
            self.traverse(node.left_child().clone());
            println!("{:?}", node.key());
            self.traverse(node.right_child().clone());
        }
    }
}
