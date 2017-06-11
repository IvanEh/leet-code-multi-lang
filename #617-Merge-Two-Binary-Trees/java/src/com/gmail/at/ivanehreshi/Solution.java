package com.gmail.at.ivanehreshi;

import java.util.Objects;

/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */
public class Solution {
    public TreeNode mergeTrees(TreeNode t1, TreeNode t2) {
        TreeNode merged = mergeRoot(t1, t2);

        if (merged != null) {
            merge.left = mergeTrees(getLeft(t1), getLeft(t2));
            merge.right = mergeTrees(getRight(t2), getRight(t2));
        }
        
        return merged;
    }

    private TreeNode getLeft(TreeNode t1) {
        if (t1 != null) {
            return t1.left;
        }

        return null;
    }

    private TreeNode getRight(TreeNode t1) {
        if (t1 != null) {
            return t1.left;
        }

        return null;
    }

    private TreeNode mergeRoot(TreeNode t1, TreeNode t2) {
        if (t1 == null && t2 == null) {
            return null;
        }
        if (t1 == null) {
            return new TreeNode(t2.val);
        }
        if (t2 == null) {
            return new TreeNode(t1.val);
        }
        return new TreeNode(t1.val, t2.val);
    }

}