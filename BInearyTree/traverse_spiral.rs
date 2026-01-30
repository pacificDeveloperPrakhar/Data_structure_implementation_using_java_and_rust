use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;


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

pub fn main() {
    let nums = vec![1, 2, 3, 4, 5, -1, 6];

    let root = Node::level_order(&nums);

    let result = traverse(root);

    println!("{:?}", result);
}

pub fn traverse(root:Option<Rc<RefCell<Node>>>)->Vec<Vec<i32>>
{
    if let Some(root)=&root
    {
        let mut q1:VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
        let mut q2:VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
        let mut ans:Vec<Vec<i32>>=Vec::new();
        q1.push_back(root.clone());
        while (q1.is_empty()!=true)||(q2.is_empty()!=true)
        {
            let mut v1:Vec<i32>=vec![];
            while  q1.is_empty()!=true
            {
             let curr=q1.pop_front().unwrap();
             v1.push(curr.borrow().value);
             if let Some(left)=&curr.borrow().left
             {
                q2.push_back(left.clone());
             };
             if let Some(right)=&curr.borrow().right
             {
                q2.push_back(right.clone());
             };
            } 
            ans.push(v1);
            let mut v1:Vec<i32>=vec![];
            while  q2.is_empty()!=true
            {
             let curr=q2.pop_front().unwrap();
             v1.push(curr.borrow().value);
             if let Some(left)=&curr.borrow().left
             {
                q1.push_back(left.clone());
             };
             if let Some(right)=&curr.borrow().right
             {
                q1.push_back(right.clone());
             };
            } 
            ans.push(v1);
        }
        return ans;
    }
    else
    {
        return Vec::new()
    }
}