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

pub fn diameter(root:Option<Rc<RefCell<Node>>>)->(i32,i32)
{
   if let Some(root)=&root
   {
    let mut dia_max=0;
    let mut dep_max=0;
    let (l_dia,l_dep) = diameter(root.borrow().left.clone());
    let (r_dia, r_dep) = diameter(root.borrow().right.clone());


    dep_max = *[l_dep,r_dep].iter().max().unwrap();


    dia_max = *[l_dia,r_dia,l_dep+r_dep+1].iter().max().unwrap();
    return (dia_max,dep_max+1);

   }
   else
   {
    return (0,0);
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
    // Level-order representation of tree
    // Tree:
    //        1
    //       / \
    //      2   3
    //     / \
    //    4   5
    let nums = vec![1,2,3,4,5];

    // Build tree
    let root = Node::level_order(&nums);

    // Call diameter function
    let (left_len, right_len) = diameter(root);

    println!("Left length  : {}", left_len);
    println!("Right length : {}", right_len);
}
