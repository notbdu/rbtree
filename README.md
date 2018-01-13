# B-Tree

Sample implementation of an index only in memory [B-Tree](https://en.wikipedia.org/wiki/B-tree) in rust.

## Usage

```
let mut btree = BTree::new(3);
btree.insert(1);
let found = btree.search(1).unwrap();
```

## Roadmap
* Deletion
* Value insertion
* Serialization to disk
