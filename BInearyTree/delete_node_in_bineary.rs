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

pub fn insert_node(
    root: Option<Rc<RefCell<Node>>>,
    node: Option<Rc<RefCell<Node>>>,
) {
    if root.is_none() || node.is_none() {
        return;
    }

    let root_rc = root.unwrap();
    let node_rc = node.unwrap();

    let mut root_ref = root_rc.borrow_mut();
    let node_val = node_rc.borrow().value;

    if root_ref.value <= node_val {
        if let Some(right) = &root_ref.right {
            insert_node(Some(right.clone()), Some(node_rc));
        } else {
            root_ref.right = Some(node_rc);
        }
    } else {
        if let Some(left) = &root_ref.left {
            insert_node(Some(left.clone()), Some(node_rc));
        } else {
            root_ref.left = Some(node_rc);
        }
    }
}

pub fn delete_node(
    curr: Option<Rc<RefCell<Node>>>,
    root: Option<Rc<RefCell<Node>>>,
    key: i32,
) {
    if let Some(curr_rc) = curr {
        let mut curr_ref = curr_rc.borrow_mut();

        if let Some(left) = &(curr_ref.left) {
            if left.borrow().value == key {
                curr_ref.left = None;
                if let Some(ll) = &left.borrow().left {
                    insert_node(root.clone(), Some(ll.clone()));
                }
                if let Some(lr) = &left.borrow().right {
                    insert_node(root.clone(), Some(lr.clone()));
                }
            }
        }

        if let Some(right) = &(curr_ref.right) {
            if right.borrow().value == key {
                curr_ref.right = None;
                if let Some(rl) = &right.borrow().left {
                    insert_node(root.clone(), Some(rl.clone()));
                }
                if let Some(rr) = &right.borrow().right {
                    insert_node(root.clone(), Some(rr.clone()));
                }
            }
        }

        if curr.value==key
        {

        }
    }
}

pub fn main() {
    // BST (level order)
    //        6
    //       / \
    //      2   8
    //     / \ / \
    //    0  4 7  9
    //      / \
    //     3   5
    let nums = vec![6, 2, 8, 0, 4, 7, 9, -1, -1, 3, 5];

    let root = Node::level_order(&nums);

    // delete node with key = 4
    delete_node(root.clone(), root.clone(), 4);
}
