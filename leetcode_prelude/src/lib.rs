mod btree;
mod linkedlist;

pub use btree::TreeNode;
pub use linkedlist::ListNode;
pub use leetcode_test::leetcode_test;

/// Create a Vec<String>
#[macro_export]
macro_rules! vec_string {
    ($($e:expr), *) => {vec![$($e.to_owned()), *]};
}

#[macro_export]
macro_rules! assert_eq_sorted {
    ($left:expr, $right:expr) => ({
        let (mut v1, mut v2) = ($left, $right);
        v1.sort();
        v2.sort();
        assert_eq!(v1, v2)
    });
    ($left:expr, $right:expr,) => ({
        assert_eq_sorted!($left, $right)
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        let (mut v1, mut v2) = ($left, $right);
        v1.sort();
        v2.sort();
        assert_eq!(v1, v2, $($arg)+)
    });
}

#[cfg(test)]
mod tests {
    use super::{assert_eq_sorted, vec_string};

    #[test]
    fn test_vec_string() {
        assert_eq!(vec!["a".to_owned(), "b".to_owned()], vec_string!["a", "b"]);
    }

    #[test]
    fn assert_eq_sorted() {
        assert_eq_sorted!(vec![1, 2], vec![2, 1]);
    }
}
