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


pub fn children_sum(curr:Option<Rc<RefCell<Node>>>)
{
    if let Some(curr)=&curr
    {
        children_sum(curr.borrow().left.clone());
        children_sum(curr.borrow().right.clone());
        let mut child_sum=0;


let node = curr.borrow();

if let Some(left) = node.left.as_ref() {
    child_sum += left.borrow().value;
}

if let Some(right) = node.right.as_ref() {
    child_sum += right.borrow().value;
}
drop(node);
        if child_sum>curr.borrow().value
        {
            curr.borrow_mut().value=child_sum;
        }
        else
        {
            let left_half=(curr.borrow().value-child_sum)/2;
            let right_half=(curr.borrow().value-child_sum)-left_half;
            let left_n=curr.borrow().left.clone();
            let right_n=curr.borrow().right.clone();
            if let Some(left)=&left_n
            {
                if let Some(right)=&right_n
                {
                    
                    let right_value=right.borrow().value;
                    let left_value=left.borrow().value;
                    left.borrow_mut().value=left_half+left_value;
                    right.borrow_mut().value=right_half+right_value;
                    children_sum(curr.borrow().left.clone());
                    children_sum(curr.borrow().right.clone());
                }
                else
                {
                    let left_value=left.borrow().value;
                    left.borrow_mut().value=left_half+right_half+left_value;
                    children_sum(curr.borrow().left.clone());
                };
            }
            else if let Some(right)=&right_n
            {
                let right_value=right.borrow().value;
                right.borrow_mut().value=left_half+right_half+right_value;
                children_sum(curr.borrow().right.clone());
            };
        }
    }
}

pub fn main() {
    // Input example
    let nums = vec![
       50 ,7 ,2 ,3 ,5 ,1 ,30
    ];

    // Build tree
    let root = Node::level_order(&nums);

    // Apply children sum property
    children_sum(root.clone());

    // Print tree in level order to verify result
    if let Some(root) = root {
        let mut queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
        queue.push_back(root);

        println!("Level-order after children_sum:");
        while let Some(node) = queue.pop_front() {
            let n = node.borrow();
            print!("{} ", n.value);

            if let Some(left) = n.left.as_ref() {
                queue.push_back(left.clone());
            }
            if let Some(right) = n.right.as_ref() {
                queue.push_back(right.clone());
            }
        }
        println!();
    }
}
