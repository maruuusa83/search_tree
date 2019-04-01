extern crate rand;

pub mod common;
pub mod naive_tree;

use common::{Pair, SearchTree};
use naive_tree::NaiveTree;
use rand::Rng;

fn insert<T: SearchTree<i32, i32>>(mut st: T, index: &[i32]) {
    let mut rng = rand::thread_rng();

    for i in 0..index.len() {
        let mut pair: Pair<i32, i32> = Pair { key: index[i], value: rng.gen() };
        let mut pair_2: Pair<i32, i32> = Pair { key: pair.key, value: pair.value };
        st.insert(pair);

        let result = st.search(pair_2.key);
        match result {
            Some(node) => assert_eq!(pair_2.value, node.pair.value),
            None => assert!(false, "result wasn't found"),
        }
    }
}


fn random_insert<T: SearchTree<i32, i32>>(mut st: T) {
    let mut index = [0i32; 10000];
    for i in 0..index.len() {
        index[i] = i as i32;
    }

    let mut rng = rand::thread_rng();

    for i in 0..index.len() {
        let rand: usize = rng.gen();
        let j = rand % index.len();
        index.swap(i, j);
    }

    insert(st, &index);
}

fn sequential_insert<T: SearchTree<i32, i32>>(mut st: T) {
    let mut index = [0i32; 10000];
    for i in 0..index.len() {
        index[i] = i as i32;
    }

    insert(st, &index);
}


#[test]
fn naive_tree() {
    let nt: NaiveTree<i32, i32> = NaiveTree{ root: None };
    random_insert(nt);

    let nt2: NaiveTree<i32, i32> = NaiveTree{ root: None };
    sequential_insert(nt2);
}

