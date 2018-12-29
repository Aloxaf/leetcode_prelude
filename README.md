# leetcode_prelude [![](https://img.shields.io/crates/v/leetcode_prelude.svg)](https://crates.io/crates/leetcode_prelude)

Some useful macros and definition for exercising in leetcode

## How to use

Add follwing lines to your Cargo.toml

```toml
[dependencies]
leetcode_prelude = "^0.1"
```

## Examples

### Binary tree initialization
```rust
use leetcode_prelude::btree;

let btree = btree![1, 2, 2, null, null, 3, 3];
```

### Linked list initialization
```rust
use leetcode_prelude::linkedlist;

let linkedlist = linkedlist![1, 2, 3];
```

### Generate test code from leetcode's json
```rust
use leetcode_prelude::leetcode_test;

leetcode_test!(
     ["Trie", "insert", "search"]
     [[], ["apple"], ["apple"]]
     [null, null, true]
)
```

### Create a `Vec<String>`
```rust
use leetcode_prelude::vec_string;

let vec = vec_string!["1", "2", "3"];
```
