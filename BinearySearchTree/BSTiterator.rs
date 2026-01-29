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
#[derive(PartialEq, Debug)]
pub struct BSTIterator
{
    curr:Rc<RefCell<Node>>,
    next:Option<Rc<RefCell<BSTIterator>>>
}

impl BSTIterator {
    pub fn construct_bst(
        curr: Option<Rc<RefCell<Node>>>,
        curr_bst: Option<Rc<RefCell<BSTIterator>>>,
    ) -> Option<Rc<RefCell<BSTIterator>>> {
        if let Some(curr)=curr
        {
            let mut this_bst=Rc::new(RefCell::new(BSTIterator
            {
             curr:curr.clone(),
             next:None
            }));

            if (curr.borrow().left==None)&&(curr.borrow().right==None)
            {
                if let Some(curr_bst)=&curr_bst
                {
                    println!("node passed as parameter {}",curr.borrow().value);
                    println!("bst passed as the paraneter {}",curr_bst.borrow().curr.borrow().value);
                    curr_bst.borrow_mut().next=Some(this_bst.clone());
                }
                return Some(this_bst.clone());
            }

            let left_bst=Self::construct_bst(curr.borrow().left.clone(),curr_bst.clone());
            println!("{}", left_bst.clone()!=None);
            if let Some(left_bst)=&left_bst
            {
              left_bst.borrow_mut().next=Some(this_bst.clone());
              println!("{:?}",curr_bst.clone()!=None);
            }
            let right_bst=Self::construct_bst(curr.borrow().right.clone(),Some(this_bst.clone()));

            if let Some(right_bst)=&right_bst
            {
                this_bst=right_bst.clone();
            }
            return Some(this_bst.clone());

        }
        else
        {
            return curr_bst.clone();
        }
}
}
pub fn main() {
    let nums = vec![4, 2, 6, 1, 3, 5, 7];
    let root = Node::level_order(&nums).unwrap();
    let start_bst=Node::new(0);
    let root_bst=Rc::new(RefCell::new(BSTIterator
    {
        curr:start_bst.clone(),
        next:None
    }));

    let it = BSTIterator::construct_bst(Some(root.clone()),Some(root_bst.clone()));
    
    let mut curr = Some(root_bst.clone());
    while let Some(node) = curr {
        // print!("{:?} ", node.borrow().next);
        println!("{} ", node.borrow().curr.borrow().value);

        curr = node.borrow().next.clone();
    }
}
