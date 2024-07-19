// 定义二叉树节点结构
#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: std::fmt::Display> TreeNode<T> {
    // 创建新节点
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // 插入新节点
    fn insert(&mut self, value: T)
    where
        T: PartialOrd,
    {
        if value < self.value {
            match self.left {
                Some(ref mut left) => left.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }
        } else {
            match self.right {
                Some(ref mut right) => right.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }
    }

    // 前序遍历
    fn pre_order(&self) {
        print!("{} ", self.value);
        if let Some(ref left) = self.left {
            left.pre_order();
        }
        if let Some(ref right) = self.right {
            right.pre_order();
        }
    }

    // 中序遍历
    fn in_order(&self) {
        if let Some(ref left) = self.left {
            left.in_order();
        }
        print!("{} ", self.value);
        if let Some(ref right) = self.right {
            right.in_order();
        }
    }

    // 后序遍历
    fn post_order(&self) {
        if let Some(ref left) = self.left {
            left.post_order();
        }
        if let Some(ref right) = self.right {
            right.post_order();
        }
        print!("{} ", self.value);
    }
}

fn main() {
    let mut root = TreeNode::new(10);
    root.insert(5);
    root.insert(15);
    root.insert(2);
    root.insert(7);
    root.insert(12);
    root.insert(20);

    // println!("完整数据:{:?}", root);

    println!("前序遍历:");
    root.pre_order();
    println!();

    println!("中序遍历:");
    root.in_order();
    println!();

    println!("后序遍历:");
    root.post_order();
    println!();
}
