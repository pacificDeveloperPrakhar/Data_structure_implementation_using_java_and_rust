use std::vec::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
#[derive(PartialEq)]
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

pub fn kt_smallest(root: Option<Rc<RefCell<Node>>>, k: usize) -> (i32, usize) {
    if root == None {
        return (-1, k);
    }
    if k == 0 {
        return (-1, 0);
    }

    if let Some(root) = &root {
        let root = root.borrow();
        let mut k_global = k;
        let mut global_result = -1;

        // LEFT
        if let Some(node) = &root.left {
            let (result, new_k) = kt_smallest(Some(node.clone()), k_global);
            k_global = new_k;

            if k_global <=0 {
                return (result, 0);
            }
            global_result = result;
        }

        k_global -= 1;
        // ROOT
        if k_global <= 0 {
            return (root.value, 0);
        }
        global_result = root.value;

        // RIGHT
        if let Some(node) = &root.right {
            let (result, new_k) = kt_smallest(Some(node.clone()), k_global);
            k_global = new_k;

            if k_global <= 0 {
                return (result, 0);
            }
            global_result = result;
        }

        return (global_result, k_global);
    } else {
        return (-1, k);
    }
}

pub fn main() {
    // BST (level order representation)
    //        5
    //       / \
    //      3   6
    //     / \
    //    2   4
    //   /
    //  1
    let nums = vec![6,2,8,0,4,7,9,-1,-1,3,5];
    let k = 3;

    let tree = Node::level_order(&nums);

    let (ans, _) = kt_smallest(tree, );
    println!("{}th smallest element is {}", k, ans);
}
