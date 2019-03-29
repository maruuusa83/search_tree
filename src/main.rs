mod b_tree;

fn main() {
    let mut bt: b_tree::BTree<i32, i32> = b_tree::BTree { root: None };
    bt.insert(b_tree::Pair { key: 1, value: 2 });
}
