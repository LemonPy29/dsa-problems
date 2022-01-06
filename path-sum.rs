use::std::rc::Rc;
use::std::cell::RefCell;

type Tree = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

impl Node {
    pub fn new(val: i32, left: Tree, right: Tree) -> Tree {
        Some(Rc::new(RefCell::new(Node { val, left, right })))
    }
}

fn dfs(root: Tree, mut current_sum: i32, target: i32) -> bool {
    if let Some(node) = root {
        let borrow = node.borrow();
        current_sum += borrow.val;
        if borrow.left.is_none() && borrow.right.is_none() {
            if current_sum == target { true } else { false }
        } else {
            dfs(node.borrow().left.clone(), current_sum, target) ||
                dfs(node.borrow().right.clone(), current_sum, target)
        }
    } else {
        false
    }
}

fn has_path_sum(root: Tree, target: i32) -> bool {
    let current_sum = 0;
    dfs(root, current_sum, target)
}

fn main() {
    let test = None;
    println!("{}", has_path_sum(test, 0));
}
