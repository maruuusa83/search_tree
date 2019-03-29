extern crate rand;

pub mod common;
pub mod naive_tree;

use common::{Pair, SearchTree};
use naive_tree::NaiveTree;
use rand::Rng;

fn random_insert<T: SearchTree<i32, i32>>(mut st: T) {
    let mut rng = rand::thread_rng();

    for _i in 0..10000 {
        let mut pair: Pair<i32, i32> = Pair { key: rng.gen(), value: rng.gen() };
        let mut pair_2: Pair<i32, i32> = Pair { key: pair.key, value: pair.value };
        st.insert(pair);

        let result = st.search(pair_2.key);
        match result {
            Some(node) => assert_eq!(pair_2.value, node.pair.value),
            None => assert!(false, "result wasn't found"),
        }
    }
}

#[test]
fn naive_tree() {
    let nt: NaiveTree<i32, i32> = NaiveTree{ root: None };
    random_insert(nt);
}

