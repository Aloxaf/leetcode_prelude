#![feature(proc_macro_hygiene)]

use leetcode_test::leetcode_test_debug;

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
