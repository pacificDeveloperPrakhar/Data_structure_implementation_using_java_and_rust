use std::collections::HashMap;
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

pub fn bottom_view(
    curr: Option<Rc<RefCell<Node>>>,
    hs: &mut HashMap<i32, Vec<(i32, i32)>>,
    depth: i32,
    dir: i32,
) {
    if let Some(curr) = curr {
        hs.entry(dir)
            .or_insert(Vec::new())
            .push((depth, curr.borrow().value));

        bottom_view(curr.borrow().left.clone(), hs, depth + 1, dir - 1);
        bottom_view(curr.borrow().right.clone(), hs, depth + 1, dir + 1);
    }
}

fn main() {
    // Given input
    let nums = vec![
        1, 2, 3, 4, 10, 9, 11, -1, 5,
        -1, -1, -1, -1, -1, -1, -1, 6
    ];

    let root = Node::level_order(&nums);

    let mut hs: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

    // Build bottom view map
    bottom_view(root, &mut hs, 0, 0);

    // Sort keys (horizontal distance)
    let mut keys: Vec<i32> = hs.keys().cloned().collect();
    keys.sort();

    // Print result
    println!("Bottom view (deepest first per column):");
    for key in keys {
        if let Some(vec) = hs.get_mut(&key) {
            // Sort by depth DESC
            vec.sort_by(|a, b| b.0.cmp(&a.0));

            print!("dir {} -> ", key);
            for (depth, value) in vec.iter() {
                print!("({},{}) ", depth, value);
            }
            println!();
        }
    }
}
