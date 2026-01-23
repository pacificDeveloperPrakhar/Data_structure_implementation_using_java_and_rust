use std::vec::*;

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
pub fn main() {
    // Example BST
    //        10
    //       /  \
    //      5   15
    //     / \
    //    2   6
    let nums = vec![2,1,3];

    let tree = Node::level_order(&nums);

    let (max, min, is_valid) = validate_bineary_tree(tree);

    println!("Is valid BST: {}", is_valid);
    println!("Min value: {}", min);
    println!("Max value: {}", max);
}


pub fn validate_bineary_tree(root:Option<Rc<RefCell<Node>>>)->(i32,i32,bool)
{
    if let None=&root
    {
        return (i32::MAX,i32::MIN,true);
    }
    

    let mut global_min=i32::MAX;
    let mut global_max=i32::MIN;

    if let Some(curr)=&root{
       let node=curr.borrow();
    
    if let Some(left)=&node.left
    {
     let (max,min,is_valid)=validate_bineary_tree(Some(left.clone()));
     if node.value<=max
     {
        return (-1,-1,false);
     }
     else if is_valid!=true
     {
        return (-1,-1,false);
     }
     global_min=min;
    }
    else
    {
        global_min=node.value;
    }
    if let Some(right)=&node.right
    {
     let (max,min,is_valid)=validate_bineary_tree(Some(right.clone()));
     if node.value>min
     {
        return (-1,-1,false);
     }
     else if is_valid!=true
     {
        return (-1,-1,false);
     }
     global_max=max;
    }
    else
    {
        global_max=node.value;
    }
    }

    return (global_max,global_min,true);
}