use std::fmt::Debug;
use super::binary_tree::TreeNode;
mod breadth;
mod depth;

pub trait TraversalStrategy<DataType>: Debug  {
    fn traverse(&self, node: Option<TreeNode<DataType>>);
}