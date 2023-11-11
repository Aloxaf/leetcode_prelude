#![feature(proc_macro_hygiene)]

use leetcode_test::leetcode_test_debug;
#[rustfmt::skip]
#[test]
fn test() {
    assert_eq!(
        leetcode_test_debug!(
            ["Trie","insert","search","search"]
            [[],[1, "apple"],["apple"],["app"]]
            [null,null,true,false]
        ),
        concat!(
            r#"let mut obj = Trie::new();"#, "\n",
            r#"obj.insert(1, "apple".to_owned());"#, "\n",
            r##"assert_eq!(obj.search("apple".to_owned()), true, r#"obj.search("apple".to_owned())"#);"##, "\n",
            r##"assert_eq!(obj.search("app".to_owned()), false, r#"obj.search("app".to_owned())"#);"##, "\n",
        )
    );
}

#[rustfmt::skip]
#[test]
fn test_with_new_args() {
    assert_eq!(
        leetcode_test_debug!(
            ["Trie","insert","search","search"]
            [["apple", "tree"],[1, "apple"],["apple"],["app"]]
            [null,null,true,false]
        ),
        concat!(
            r#"let mut obj = Trie::new("apple".to_owned(), "tree".to_owned());"#, "\n",
            r#"obj.insert(1, "apple".to_owned());"#, "\n",
            r##"assert_eq!(obj.search("apple".to_owned()), true, r#"obj.search("apple".to_owned())"#);"##, "\n",
            r##"assert_eq!(obj.search("app".to_owned()), false, r#"obj.search("app".to_owned())"#);"##, "\n",
        )
    );
}

#[rustfmt::skip]
#[test]
fn test_with_nested_args() {
    assert_eq!(
        leetcode_test_debug!(
            ["Graph", "shortestPath", "shortestPath", "addEdge", "shortestPath"]
            [[4, [[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]], [3, 2], [0, 3], [[1, 3, 4]], [0, 3]]
            [null, 6, -1, null, 6]
        ),
        concat!(
            r#"let mut obj = Graph::new(4, vec![vec![0, 2, 5], vec![0, 1, 2], vec![1, 2, 1], vec![3, 0, 3]]);"#, "\n",
            r##"assert_eq!(obj.shortest_path(3, 2), 6, r#"obj.shortest_path(3, 2)"#);"##, "\n",
            r##"assert_eq!(obj.shortest_path(0, 3), -1, r#"obj.shortest_path(0, 3)"#);"##, "\n",
            r#"obj.add_edge(vec![1, 3, 4]);"#, "\n",
            r##"assert_eq!(obj.shortest_path(0, 3), 6, r#"obj.shortest_path(0, 3)"#);"##, "\n",
        )
    );
}
