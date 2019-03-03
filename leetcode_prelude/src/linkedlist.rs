/// Definition for singly-linked list.
///
/// # Note
///
/// I add Ord PartialOrd for sort Vec<TreeNode> when testing
/// Please don't rely on it
use std::fmt;

#[derive(PartialEq, Eq, Ord, PartialOrd)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl fmt::Debug for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = vec![self.val];
        let mut p = self;
        while let Some(next) = p.next.as_ref() {
            v.push(next.val);
            p = next;
        }
        write!(f, "{:?}", v)
    }
}

/// Create a linked list with ListNode
///
/// # Example
///
/// ```rust
/// use leetcode_prelude::linkedlist;
///
/// let list = linkedlist![1, 2, 3];
/// ```
#[macro_export]
macro_rules! linkedlist {
    () => {
        None
    };
    ($($e:expr), *) => {
        {
            let mut head = Box::new($crate::ListNode::new(0));
            let mut ref_head = &mut head;

            $(
            ref_head.next = Some(Box::new($crate::ListNode::new($e)));
            ref_head = ref_head.next.as_mut().unwrap();
            )*

            let _ = ref_head; // 避免 `unused_assignments`
            head.next
        }
    };
}
