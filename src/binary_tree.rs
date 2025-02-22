#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
    fn insert_left(&mut self, value: T) {
        self.left = Some(Box::new(TreeNode::new(value)));
    }

    fn insert_right(&mut self, value: T) {
        self.right = Some(Box::new(TreeNode::new(value)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_left_right() {
        let mut root: TreeNode<&str> = TreeNode::new("node string");
        root.insert_left("left");
        root.insert_right("right");

        println!("{:#?}", root);
    }
}
