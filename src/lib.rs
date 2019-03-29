extern crate rand;
use rand::Rng;

mod b_tree;

#[test]
fn random_insert(){
    let mut bt: b_tree::BTree<i32, i32> = b_tree::BTree{ root: None };
    let mut rng = rand::thread_rng();

    for i in 0..10000 {
        let mut pair: b_tree::Pair<i32, i32> = b_tree::Pair { key: rng.gen(), value: rng.gen() };
        let mut pair_2: b_tree::Pair<i32, i32> = b_tree::Pair { key: pair.key, value: pair.value };
        bt.insert(pair);

        let result = bt.search(pair_2.key);
        match result {
            Some(node) => assert_eq!(pair_2.value, node.pair.value),
            None => assert!(false, "result wasn't found"),
        }
    }
}
