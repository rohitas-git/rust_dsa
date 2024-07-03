use std::{cell::RefCell, fmt::Debug, rc::Rc};
use super::traversal::TraversalStrategy;

pub type TreeNode<DataType> = Rc<RefCell<Node<DataType>>>;

pub struct BinaryTree<D> {
    root: Option<TreeNode<D>>,
    traversal_strategy: Box<dyn TraversalStrategy<D>>,
}

#[derive(Debug,Default,Clone)]
pub struct Node<DataType> {
    key: DataType,
    left_child: Option<TreeNode<DataType>>,
    right_child: Option<TreeNode<DataType>>,
}

impl<DataType> Node<DataType> 
    where  DataType: Default + Debug + Clone
{
    fn new(key: DataType) -> TreeNode<DataType> {
        Rc::new(RefCell::new(Node {
            key,
            left_child: None,
            right_child: None,
        }))
    }

    pub fn key(&self) -> DataType{
        self.key.clone()
    }
    pub fn left_child(&self) -> Option<TreeNode<DataType>>{
        self.left_child.clone()
    }
    pub fn right_child(&self) -> Option<TreeNode<DataType>>{
        self.right_child.clone()
    }
}

impl<D> BinaryTree<D> {
    fn new(traversal_strategy: Box<dyn TraversalStrategy<D>>) -> Self {
        Self {
            root: None,
            traversal_strategy,
        }
    }

    fn set_traversal_strategy(&mut self, new_strategy: Box<dyn TraversalStrategy<D>>) {
        self.traversal_strategy = new_strategy;
    }

    fn traverse(&self) {
        self.traversal_strategy.traverse(self.root.clone());
    }
}
