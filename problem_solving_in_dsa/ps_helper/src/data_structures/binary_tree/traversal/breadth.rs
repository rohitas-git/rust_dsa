use std::fmt::Debug;
use std::marker::PhantomData;

use super::TraversalStrategy;
use super::TreeNode;

#[derive(Debug, Clone, Default)]
struct BreadthFirstTraversal<DataType>(PhantomData<DataType>);

impl<DataType> TraversalStrategy<DataType> for BreadthFirstTraversal<DataType>
where
    DataType: Default + Debug + Clone,
{
    fn traverse(&self, root: Option<TreeNode<DataType>>) {
        if let Some(root) = root {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root.clone());

            while let Some(current) = queue.pop_front() {
                let current = current.borrow();
                println!("{:?}", current.key());

                if let Some(left) = current.left_child().clone() {
                    queue.push_back(left);
                }

                if let Some(right) = current.right_child().clone() {
                    queue.push_back(right);
                }
            }
        }
    }
}
