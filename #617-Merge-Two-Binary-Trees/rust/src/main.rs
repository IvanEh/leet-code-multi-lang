use std::fmt;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug)]
pub struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nodes = enumerate_ordered_with_gaps(self);

        let strings = nodes.into_iter().map(|node| {
            match node {
                Some(node) => node.val.to_string(),
                None => String::from("none")
            }
        }).collect::<Vec<_>>();

        writeln! (f, "{:?}", strings)
    }
}


impl TreeNode {
    pub fn new(val: i32) -> TreeNode {
        TreeNode {
            val: val,
            left: None,
            right: None
        }
    }
    pub fn size(&self) -> u32 {
        1u32 + self.left.as_ref().map_or(0u32, |t| t.size())
             + self.right.as_ref().map_or(0u32, |t| t.size())
    }

    pub fn with_nodes(val: i32, left: Option<TreeNode>, right: Option<TreeNode>) -> TreeNode{
        TreeNode {
            val: val,
            left: left.map(|t| Box::new(t)),
            right: right.map(|t| Box::new(t))
        }
    }
    pub fn right_leaning(val: i32, right: TreeNode) -> TreeNode {
        TreeNode {
            val: val,
            left: Some(Box::new(right)),
            right: None
        }
    }
    pub fn left_leaning(val: i32, left: TreeNode) -> TreeNode {
        TreeNode {
            val: val,
            left: None,
            right: Some(Box::new(left)),
        }
    }
    pub fn left_ref(&self) -> Option<&TreeNode> {
        self.left.as_ref().map(|t| &**t)
    }
    pub fn right_ref(&self) -> Option<&TreeNode> {
        self.right.as_ref().map(|t| &**t)
    }
}

pub fn enumerate_ordered_with_gaps(t: &TreeNode) -> Vec<Option<&TreeNode>> {
    let mut enumarated = Vec::new();
    let mut dequeue = VecDeque::new();
    dequeue.push_front(Some(t));

    while !is_all_none(&dequeue) {
        let node = dequeue.pop_back().unwrap();
        dequeue.push_front(node.and_then(|t| t.left.as_ref().map(|t| &**t)));
        dequeue.push_front(node.and_then(|t| t.right.as_ref().map(|t| &**t)));
        enumarated.push(node);
    }

    enumarated
}

fn is_all_none(nodes: &VecDeque<Option<&TreeNode>>) -> bool {
    for node in nodes {
        if node.is_some() {
            return false;
        }
    }
    return true;
}

pub fn merge_trees(t1: Option<&TreeNode>, t2: Option<&TreeNode>) -> Option<TreeNode> {
    let mut merged = merge_roots(t1, t2);

    merged.as_mut().map(|t| {
        let right = merge_trees(t1.and_then(|t1| t1.right_ref()), t2.and_then(|t2| t2.right_ref()));
        let left = merge_trees(t1.and_then(|t1| t1.left_ref()), t2.and_then(|t2| t2.left_ref()));
        TreeNode::with_nodes(t.val, left, right)
    })
}

fn merge_roots(t1: Option<&TreeNode>, t2: Option<&TreeNode>) -> Option<TreeNode> {
    match (t1, t2) {
        (None, None) => None,
        (Some(t), None) => Some(TreeNode::new(t.val)),
        (None, Some(t)) => Some(TreeNode::new(t.val)),
        (Some(t1), Some(t2)) => Some(TreeNode::new(t1.val + t2.val))
    }
}

pub fn vec_to_tree(vec: &Vec<Option<i32>>) -> Option<TreeNode> {
    let mut root = vec.get(0).and_then(|val| val.map(|val| TreeNode::new(val)));
    if let Some(ref mut root) = root {
        fill_node(root, vec, 0);
    }

    root
}

fn fill_node<'a>(node: &'a mut TreeNode, vec: &Vec<Option<i32>>, index: usize) {
    let mut left = vec.get(2*index + 1).and_then(|val| val.map(|val| TreeNode::new(val)));
    let mut right = vec.get(2*index + 2).and_then(|val| val.map(|val| TreeNode::new(val)));

    if let Some(ref mut left) = left {
        fill_node(left, vec, 2*index + 1);
    }
    if let Some(ref mut right) = right {
        fill_node(right, vec, 2*index + 2);
    }

    node.right = right.map(|node| Box::new(node));
    node.left = left.map(|node| Box::new(node));
}

fn main() {
    println! ("{}", TreeNode::with_nodes(1, Some(TreeNode::new(2)),
                                         Some(TreeNode::with_nodes(3, Some(TreeNode::new(4)), None))));
    println! ("{}", vec_to_tree(&vec! (Some(1), Some(2), None, Some(3), Some(4), None, Some(5))).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_to_tree_should_return_none_when_empty_vec() {
        assert_eq! (vec_to_tree(&Vec::new()), None);
    }

    #[test]
    fn vec_to_tree_should_return_the_same_node_when_there_is_one_value() {
        assert_eq! (vec_to_tree(&vec! (Some(1))), Some(TreeNode::new(1)));
    }

    #[test]
    fn vec_to_tree_should_return_full_node_whene_there_are_three_values() {
        assert_eq! (vec_to_tree(&vec! (Some(1), Some(2), Some(3))),
            Some(TreeNode::with_nodes(1, Some(TreeNode::new(2)), Some(TreeNode::new(3)))));
    }

    #[test]
    fn merge_trees_should_merge_with_gaps() {
        let t1 = tree_with_positive_values(vec! (1, 3, 2, 5));
        let t2 = tree_with_positive_values(vec! (2, 1, 3, 0, 4, 0, 7));
        let expected = tree_with_positive_values(vec! (3, 4, 5, 5, 4, 0, 7));

        assert_eq! (Some(expected), merge_trees(Some(&t1), Some(&t2)));
    }

    fn tree_with_positive_values(vals: Vec<u32>) -> TreeNode {
        let vals = vals.into_iter()
                .map(|val| val as i32)
                .map(|val| {
                    if val > 0 { Some(val) } else { None }
                })
                .collect::<Vec<_>>();

        vec_to_tree(&vals).unwrap()
    }

}
