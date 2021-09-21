fn main() {
    let a = solution(
        Some(Box::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })),
        Some(Box::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })),
    );
    println!("{:?}", a);
}

// Definition for a binary tree node.
#[derive(Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

pub fn solution(p: Option<Box<TreeNode>>, q: Option<Box<TreeNode>>) -> bool {
    // TODO: implement me!
    f(&p, &q)
}

fn f(left: &Option<Box<TreeNode>>, right: &Option<Box<TreeNode>>) -> bool {
    if (left.is_none() && right.is_some()) || (left.is_some() && right.is_none()) {
        return false;
    }
    if let Some(a) = left.as_ref() {
        if a.val != right.as_ref().unwrap().val {
            return false;
        }
    } else {
        return true;
    }

    f(&left.as_ref().unwrap().left, &right.as_ref().unwrap().left)
        && f(
            &left.as_ref().unwrap().right,
            &right.as_ref().unwrap().right,
        )
}
