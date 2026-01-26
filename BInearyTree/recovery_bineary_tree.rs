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

pub struct State
{
    prev:Option<Rc<RefCell<Node>>>,
    n_to_change:Vec<(Rc<RefCell<Node>>,Rc<RefCell<Node>>)>
}

pub fn recovery_bst(root:Option<Rc<RefCell<Node>>>,s:&mut State){
    if let Some(root)=&root
    {
        let root_rc=root.clone();
        // s.n_to_change.push(root.clone());
        let root=root.borrow();
        if let Some(left)=&(root.left)
        {
            recovery_bst(Some(left.clone()),s);
        }

        if let Some(prev)=&s.prev
        {
            if prev.borrow().value>root.value
            {
                
                s.n_to_change.push((prev.clone(),root_rc.clone()));
            }

            
        }
        s.prev = Some(root_rc.clone());

        if let Some(right)=&(root.right)
        {
            recovery_bst(Some(right.clone()),s);
 
        }
        
    }
}

pub fn recovery_bst_helper(root:Option<Rc<RefCell<Node>>>)
{
    if let Some(root)=root
    {

        let mut s = State {
            prev: None,
            n_to_change: Vec::new(),
        };
        
        // Run recovery traversal
        recovery_bst(Some(root.clone()), &mut s);
        if s.n_to_change.len()==1
        {
            swap(Some(s.n_to_change[0].0.clone()),Some(s.n_to_change[0].1.clone()));
        }
        else if s.n_to_change.len()==2
        {
            swap(Some(s.n_to_change[0].0.clone()),Some(s.n_to_change[1].1.clone()));
        }
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
    // Example BST where two nodes are swapped
    // Inorder before recovery: 3 1 2
    // Inorder after recovery : 1 2 3
    let nums = vec![1, 3, -1, -1, 2];

    let root = Node::level_order(&nums);

    // Call your recovery helper
    recovery_bst_helper(root);
}
