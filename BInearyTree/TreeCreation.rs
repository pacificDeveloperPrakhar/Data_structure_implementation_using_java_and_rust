pub struct Node {
    left:  Box<Option<Node>>,
    right: Box<Option< Node>>,
    value: i32,
}

impl Node {
    pub fn new(value: i32) -> Node {
        Node {
            left: Box::new(None),
            right: Box::new(None),
            value,
        }
    }
    pub fn level_order(nums:&Vec<i32>)->Option<Node>
    {
        let mut queue:Vec<&mut Box<Option<Node>>>=vec![];
        let mut i=0;
        if nums[i]==-1
        {
         return None;
        }

        let mut root=Box::new(Some(Node::new(nums[i])));
        queue.push(&mut root);
        i+=1;
        while queue.len()>0
        {
         if let mut Some(n)=*(*(queue.remove(0)))
         {

             if i<nums.len()&&nums[i]!=-1
             {
                 n.left=Box::new(Some(Node::new(nums[i])));
                 queue.push(&mut n.left);
             }
                i+=1;
                if i<nums.len()&&nums[i]!=-1
                {
                    n.right=Box::new(Some(Node::new(nums[i])));
                    queue.push(&mut n.right);
                    i+=1;
                }
            }
                }
        
         return *root;
    }
}

pub fn main()
{

}