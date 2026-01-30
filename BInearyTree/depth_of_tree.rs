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

pub fn depth_rec(curr:Option<Rc<RefCell<Node>>>)->i32
{
    if let Some(curr)=&curr
    {
        let mut max=0;
        max=maximum(depth_rec(curr.borrow().left.clone()),max);
        max=maximum(depth_rec(curr.borrow().right.clone()),max);
        return max+1;
    }
    else
    {
        return 0;
    }
}

pub fn maximum(x:i32,y:i32)->i32
{
    if x>y
    {
        return x;
    }
    else 
    {
        return y;
    }
}

pub fn main() {
    // level-order input, -1 means null
    let nums = vec![3, 9, 20, -1, -1, 15, 7];

    // build tree
    let root = Node::level_order(&nums);

    // compute depth
    let depth = depth_rec(root);

    println!("Depth of tree: {}", depth);
}
