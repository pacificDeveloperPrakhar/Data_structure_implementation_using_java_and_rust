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

pub fn largest_bst(curr:Option<Rc<RefCell<Node>>>)->(i32,i32,bool,usize)
{
    if let Some(curr)=&curr
    {
    let mut root=curr.clone();
    let mut curr=curr.borrow();
    let mut depth=1;
    let mut g_max=curr.value;
    let mut g_min=curr.value;
    let mut lg_valid=false;
    let mut rg_valid=false;
    let mut is_valid=false;
    let (l_min,l_max,l_valid,l_depth)=largest_bst(curr.left.clone());
    let (r_min,r_max,r_valid,r_depth)=largest_bst(curr.right.clone());
    
    if curr.right!=None
    {
     if (r_valid)&&(r_min>=curr.value)
     {
        depth=maximum(depth,r_depth+1);
        lg_valid=true;
        g_max=r_max;
    }
    else
    {
        depth=maximum(depth,r_depth);
        
    }
}
else
{
    rg_valid=true;
}
if curr.left!=None
{
    if (l_valid)&&(l_max<curr.value)
    {
        depth=maximum(depth,l_depth+1);
        rg_valid=true;
        g_min=l_min;
     }
     else 
     {
        depth=maximum(depth,l_depth);
        
     }
    }
    else
    {
        lg_valid=true;
    }
 return (g_min,g_max,rg_valid|lg_valid,depth);
}
return (i32::MIN,i32::MAX,true,0);
}

pub fn maximum(curr: usize, max: usize) -> usize {
    if max < curr {
        return curr;
    }
    max
}

pub fn main() {
    // Example binary tree (level order)
    // Tree:
    //        10
    //       /  \
    //      5   15
    //     / \    \
    //    1   8    7   (violates BST on right)
    let nums = vec![10, 5, 15, 1, 8, -1, 7];

    let root = Node::level_order(&nums);

    let (min_val, max_val, is_bst, size) = largest_bst(root);

    println!(
        "min: {}, max: {}, is_bst: {}, size: {}",
        min_val, max_val, is_bst, size
    );
}
