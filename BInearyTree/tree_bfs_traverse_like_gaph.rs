use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{VecDeque, HashMap};

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
#[derive(PartialEq)]
pub struct NodeW {
    parent: Option<Rc<RefCell<NodeW>>>,
    left: Option<Rc<RefCell<NodeW>>>,
    right: Option<Rc<RefCell<NodeW>>>,
    value: i32,
}

impl NodeW {
    pub fn traverse(
        curr: Option<Rc<RefCell<Node>>>,
        parent: Option<Rc<RefCell<NodeW>>>,
    ) {
        if let Some(curr) = curr {
            let curr_val = curr.borrow().value;

            if let Some(left) = curr.borrow().left.clone() {
                let left_n = Rc::new(RefCell::new(NodeW {
                    left: None,
                    right: None,
                    parent: parent.clone(),
                    value: left.borrow().value,
                }));
                parent.as_ref().unwrap().borrow_mut().left = Some(left_n.clone());
                Self::traverse(Some(left), Some(left_n));
            }

            if let Some(right) = curr.borrow().right.clone() {
                let right_n = Rc::new(RefCell::new(NodeW {
                    left: None,
                    right: None,
                    parent: parent.clone(),
                    value: right.borrow().value,
                }));
                parent.as_ref().unwrap().borrow_mut().right = Some(right_n.clone());
                Self::traverse(Some(right), Some(right_n));
            }
        }
    }

    pub fn new(root: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Self>>> {
        if let Some(root) = root {
            let root_n = Rc::new(RefCell::new(NodeW {
                left: None,
                right: None,
                parent: None,
                value: root.borrow().value,
            }));
            Self::traverse(Some(root), Some(root_n.clone()));
            Some(root_n)
        } else {
            None
        }
    }
}

pub fn traverse_level_order(curr: Option<Rc<RefCell<NodeW>>>) {
    let mut queue: VecDeque<Rc<RefCell<NodeW>>> = VecDeque::new();
    let mut hs:HashMap<i32,bool>=HashMap::new();
    if let Some(curr) = curr {
        queue.push_back(curr);
        let mut num_level = 1;

        while !queue.is_empty() {
            let mut next_level = 0;

            for _ in 0..num_level {
                if let Some(node) = queue.pop_front() {
                    let n = node.borrow();
                    println!("{}", n.value);

                    if let Some(left) = n.left.clone() {
                        if let Some(val)=hs.get(&(left.borrow().value))
                        {
                            continue;
                        }
                        else
                        {
                            hs.insert(left.borrow().value,true)
                        };
                        queue.push_back(left.clone());
                        next_level += 1;
                    }
                    if let Some(right) = n.right.clone() {
                        if let Some(val)=hs.get(&(right.borrow().value))
                        {
                            continue;
                        }
                        else
                        {
                            hs.insert(right.borrow().value,true)
                        };
                        queue.push_back(right.clone());
                        next_level += 1;
                    }
                    if let Some(parent)=&n.parent.clone()
                    {
                        if let Some(val)=hs.get(&(parent.borrow().value))
                        {
                            continue;
                        }
                        else
                        {
                            hs.insert(parent.borrow().value,true)
                        };
                        queue.push_back(parent.clone());
                        next_level += 1;
                    }
                }
            }
            num_level = next_level;
        }
    }
}

pub fn find_node(curr:Option<Rc<RefCell<NodeW>>>,target:i32)->Option<Rc<RefCell<NodeW>>>
{
    if let Some(curr)=&curr
    {
        if curr.borrow().value==target
        {
            return Some(curr.clone());
        }
        let left_n=find_node(curr.borrow().left.clone(),target);
        let right_n=find_node(curr.borrow().right.clone(),target);

        if left_n != None
        {
            return left_n.clone();
        }
        else if right_n !=None
        {
            return right_n.clone();
        };
        return None;
    }
    else
    {
        return None;
    }
}

pub fn main() {
    // Level-order input (-1 means null)
    let nums = vec![3, 5, 1, 6, 2, 0, 8, -1, -1, 7, 4];

    // Build the binary tree
    let root = Node::level_order(&nums);
    let root=NodeW::new(root.clone());
    let target_n=find_node(root.clone(),5);
    // Traverse using your NodeW-based level traversal
    traverse_level_order(target_n.clone());
}
