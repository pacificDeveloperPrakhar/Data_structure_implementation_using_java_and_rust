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

pub fn boundary(curr:Option<Rc<RefCell<Node>>>)->()
{
    if let Some(curr)=&curr
    {
        println!("{}",curr.borrow().value);
        if let Some(left)=(&curr.borrow().left)
        {
         if let Some(right)=(&curr.borrow().right)
         {
          let left=left.borrow();
          let right=right.borrow();
          if (left.left==None&&left.right==None)&&(right.left==None&&right.left==None)
          {
            return;
          }
         };
         boundary(Some(left.clone()));
         return;
        };
        if let Some(right)=(&curr.borrow().right)
        {
         boundary(Some(right.clone()));
 
        };

    }
}


pub fn right_boundary(curr:Option<Rc<RefCell<Node>>>,stack:&mut Vec<i32>)
{
 if let Some(curr)=&curr
 {
    stack.push(curr.borrow().value);
    if let Some(right)=(&curr.borrow().right)
    {
     if let Some(left)=(&curr.borrow().left)
     {
        let left=left.borrow();
        let right=right.borrow();
        if (left.left==None&&left.right==None)&&(right.left==None&&right.right==None)
        {
            return ();
        }
     }
     right_boundary(Some(right.clone()),stack);
     return ();
    };
    right_boundary(curr.borrow().left.clone(),stack);
 }
}

pub fn leaf_boundary_nodes(curr:Option<Rc<RefCell<Node>>>)
{
    if let Some(curr)=&curr
    {
        if curr.borrow().left==None&&curr.borrow().right==None
        {
            println!("{}",curr.borrow().value);
        }
        leaf_boundary_nodes(curr.borrow().left.clone());
        leaf_boundary_nodes(curr.borrow().right.clone());
    }
}
pub fn main() {
    let nums = vec![1, 2, 7, 3, -1, -1, 8, -1, 4, 9, -1, 5, 6, 10, 11];
    let root = Node::level_order(&nums);

    // 1️⃣ Root + left boundary
    boundary(root.clone());

    // 2️⃣ Leaf nodes
    leaf_boundary_nodes(root.clone());
    
    // 3️⃣ Right boundary (stored in stack, then printed in reverse)
    let mut stack: Vec<i32> = Vec::new();
    right_boundary(root.unwrap().borrow().right.clone(), &mut stack);

    while let Some(val) = stack.pop() {
        println!("{}", val);
    }
}
