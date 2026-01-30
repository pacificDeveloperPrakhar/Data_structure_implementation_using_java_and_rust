use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
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

            if i < nums.len() && nums[i] != -1 {
                let left = Node::new(nums[i]);
                curr_node.left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;

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

fn main() {
    let nums = vec![1, 2, 3, -1, 4, -1, 5];

    let root = Node::level_order(&nums);

    println!("Preorder traversal:");
    traverse(root);
}


pub fn traverse(root: Option<Rc<RefCell<Node>>>) {
    if let Some(root) = root {
        let mut stack: Vec<Rc<RefCell<Node>>> = Vec::new();
        stack.push(root);

        while !stack.is_empty() {
            let curr = stack.pop().unwrap();
            println!("{}", curr.borrow().value);

            if let Some(right) = &curr.borrow().right.clone() {
                stack.push(right.clone());
            };

            if let Some(left) = &curr.borrow().left.clone() {
                stack.push(left.clone());
            };
        }
    }
}
