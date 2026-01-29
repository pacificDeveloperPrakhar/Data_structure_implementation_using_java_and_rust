
// Definition for a binary tree node.
public class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;
    TreeNode() {}
    TreeNode(int val) { this.val = val; }
    TreeNode(int val, TreeNode left, TreeNode right) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

class Solution {
    public void insertRec(TreeNode root,int val)
    {
        if ((root.left==null)&&(root.val>val))
        {
         root.left=new TreeNode(val,null,null);
         return ;
        }
        else if ((root.right==null)&&(root.val<=val))
        {
         
         root.right=new TreeNode(val,null,null);
         return ;
        }

        if (root.val<=val)
        {
            insertRec(root.right,val);
        }
        else
        {
            insertRec(root.left,val);
        }
        
    }
    public TreeNode insertIntoBST(TreeNode root, int val) {
        if (root==null)
        {
            return new TreeNode(val,null,null);
        }
        insertRec(root,val);
        return root;
    }
}