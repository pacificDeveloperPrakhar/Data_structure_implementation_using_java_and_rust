use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Node {
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    value: i32,
}

impl Node {
    pub fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            left: None,
            right: None,
            value,
        }))
    }

    pub fn level_order(nums: &Vec<i32>) -> Option<Rc<RefCell<Node>>> {
        if nums.is_empty() || nums[0] == -1 {
            return None;
        }

        let root = Node::new(nums[0]);
        let mut queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
        queue.push_back(root.clone());

        let mut i = 1;

        while i < nums.len() {
            let curr = queue.pop_front().unwrap();
            let mut curr_node = curr.borrow_mut();

            // left child
            if i < nums.len() && nums[i] != -1 {
                let left = Node::new(nums[i]);
                curr_node.left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;

            // right child
            if i < nums.len() && nums[i] != -1 {
                let right = Node::new(nums[i]);
                curr_node.right = Some(right.clone());
                queue.push_back(right);
            }
            i += 1;
        }

        Some(root)
    }
}

/* ---------- PRINT TREE (LEVEL ORDER) ---------- */
fn print_tree(root: Option<Rc<RefCell<Node>>>) {
    if root.is_none() {
        println!("Empty tree");
        return;
    }

    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());

    while let Some(node_rc) = queue.pop_front() {
        let node = node_rc.borrow();
        print!("{} ", node.value);

        if let Some(left) = &node.left {
            queue.push_back(left.clone());
        }
        if let Some(right) = &node.right {
            queue.push_back(right.clone());
        }
    }
    println!();
}


pub fn ceil_tree(curr: Option<Rc<RefCell<Node>>>, key: i32) -> i32 {
    if curr.is_none() {
        return -1;
    }

    let curr_rc = curr.unwrap();
    let curr_node = curr_rc.borrow();

    let mut left = -1;
    let mut right = -1;

    if curr_node.value > key {
        left = ceil_tree(curr_node.left.clone(), key);
    }

    if curr_node.value <= key {
        right = ceil_tree(curr_node.right.clone(), key);
    }

    if left != -1 {
        return left;
    } else if right != -1 {
        return right;
    } else if key <= curr_node.value {
        return curr_node.value;
    }

    -1
}

pub fn main() {
    let nums = vec![10, 5, 15, 2, 6, -1, -1];
    let key = 7;

    let tree = Node::level_order(&nums);

    print!("Tree (level-order): ");
    print_tree(tree.clone());

    let ans = ceil_tree(tree, key);
    println!("Ceil of {} is {}", key, ans);
}
