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

pub fn maximum_width_bineary_tree(curr:Option<Rc<RefCell<Node>>>,curr_dir:i32)->(i32,i32)
{
    if let Some(curr)=&curr
    {
        let(l_dir,r_dir)=maximum_width_bineary_tree(curr.borrow().left.clone(),curr_dir-1);
        let (l1_dir,r1_dir)=maximum_width_bineary_tree(curr.borrow().right.clone(),curr_dir+1);
        let mut l_g_dir=*[l_dir,r_dir,l1_dir,r1_dir,curr_dir].iter().max().unwrap();
        let mut r_g_dir=*[l_dir,r_dir,l1_dir,r1_dir,curr_dir].iter().min().unwrap();
        return (l_g_dir,r_g_dir);
    }
    else
    {
        return (0,0)
    }
}
pub fn main() {
    let nums = vec![
        1,3,2,5,3,-1,9
    ];

    let root = Node::level_order(&nums);

    let (max_dir, min_dir) = maximum_width_bineary_tree(root, 0);

    println!("Max horizontal distance: {}", max_dir);
    println!("Min horizontal distance: {}", min_dir);
    println!("Maximum width of binary tree: {}", max_dir - min_dir );
}
