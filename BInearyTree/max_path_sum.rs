use std::vec::*;
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
        if nums.is_empty() || nums[0] == -2 {
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
pub fn max_path(root:Option<Rc<RefCell<Node>>>)->(i32,i32)
{
    if let Some(root)=&root{
        let mut dep_max=0;
        let mut max_sum=0;
        let (l_dep,l_sum)=max_path(root.borrow().left.clone());
        let (r_dep,r_sum)=max_path(root.borrow().right.clone());
        dep_max=*[l_dep,r_dep,0].iter().max().unwrap()+root.borrow().value;
        max_sum=*[l_sum,r_sum,l_dep+r_dep+root.borrow().value,root.borrow().value,l_dep,r_dep,dep_max].iter().max().unwrap();
        return (dep_max,max_sum);
    }
    else
    {
        return (0,0);
    }
}

pub fn main() {
    // Level-order representation of tree
    // Tree:
    //        1
    //       / \
    //      2   3
    //     / \
    //    4   5
    let nums = vec![2,-1];

    // Build tree
    let root = Node::level_order(&nums);

    // Call diameter function
    let (left_len, right_len) = max_path(root);

    println!("Left length  : {}", left_len);
    println!("Right length : {}", right_len);
}
