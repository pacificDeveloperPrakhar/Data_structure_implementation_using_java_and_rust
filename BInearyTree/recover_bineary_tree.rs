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

pub fn recovery_bst(root:Option<Rc<RefCell<Node>>>)->(Option<Rc<RefCell<Node>>>,Option<Rc<RefCell<Node>>>)
{
    if let Some(root_rfc)=&root
    
    {
    let mut root:Rc<RefCell<Node>>=root_rfc.clone();
    let mut global_min:Rc<RefCell<Node>>=root_rfc.clone();
    let mut global_max:Rc<RefCell<Node>>=root_rfc.clone();
    
        let mut root_rfc=root_rfc.borrow();
        if let Some(left)=&(root_rfc.left)
        {
            let (min,max)=recovery_bst(Some(left.clone()));
            if let Some(max_rfc)=max
            {
              global_min=max_rfc.clone();
              if max_rfc.borrow().value>=root_rfc.value
              {
                drop(root_rfc);
                swap(Some(root.clone()),Some(max_rfc.clone()));
                root_rfc=root.borrow();
              }
              
            }
        }

         if let Some(right)=&(root_rfc.right)
        {
            let (min,max)=recovery_bst(Some(right.clone()));
            if let Some(min_rfc)=min
            {
              global_max=min_rfc.clone();
              if min_rfc.borrow().value>=root_rfc.value
              {
                drop(root_rfc);
                swap(Some(root.clone()),Some(min_rfc.clone()));
                root_rfc=root.borrow();
              }

            }
        } 

      return (Some(global_min.clone()),Some(global_max.clone()))
    }
    else
    {
        return (None,None);
    }
}

pub fn swap(n1:Option<Rc<RefCell<Node>>>,n2:Option<Rc<RefCell<Node>>>)
{
    let mut temp=0;
    if let Some(n1)=&n1
    {
        if let Some(n2)=&n2
        {
         
            {let mut n1=n1.borrow_mut();
            temp=n1.value;
            let n2=n2.borrow();
            n1.value=n2.value;
            }
            
            {
                let mut n2=n2.borrow_mut();
                n2.value=temp;
            }

        }
    }
}
pub fn main() {
    // Example BST with two nodes swapped
    // Level order: [3, 1, 4, -1, -1, 2]
    let nums = vec![1,2,-1,-1,3];

    let root = Node::level_order(&nums);

    let (min_node, max_node) = recovery_bst(root.clone());

    if let Some(n) = min_node {
        println!("Min swapped node value: {}", n.borrow().value);
    }

    if let Some(n) = max_node {
        println!("Max swapped node value: {}", n.borrow().value);
    }
}
