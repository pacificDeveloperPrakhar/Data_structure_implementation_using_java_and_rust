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

pub fn lowest_common_ancestor(root:Option<Rc<RefCell<Node>>>,p:i32,q:i32)->(bool,i32)
{
    if let Some(root)=&root
    {
     let root=root.borrow();
     let mut left_res=0;
     let mut right_res=0;
     let mut is_left_ancestor=false;
     let mut is_right_ancestor=false;
     if let Some(left)=&root.left
     {
        (is_left_ancestor,left_res)=lowest_common_ancestor(Some(left.clone()),p,q);
        
    }
    if let Some(right)=&root.right
    {
         (is_right_ancestor,right_res)=lowest_common_ancestor(Some(right.clone()),p,q);
     }

     if (root.value==q)||(root.value==p)
     {
        if is_left_ancestor!=true
        {
            left_res=root.value;
            is_left_ancestor=true
        }
        else if is_right_ancestor!=true
        {
            right_res=root.value;
            is_right_ancestor=true;
        }
     }

     if is_left_ancestor&&is_right_ancestor
     {
        return (true,root.value);
     }
     if is_left_ancestor
     {
        return (true,left_res);
     }
     if is_right_ancestor
     {
        return (true,right_res);
     }
    }
    return (false,-1);
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

    let (res, ans) = lowest_common_ancestor(tree, 2,4);
    println!(" {}", ans);
}
