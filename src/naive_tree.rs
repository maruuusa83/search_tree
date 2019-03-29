use common::{Pair, Node, SearchTree};

pub struct NaiveTree<K: PartialOrd, V> {
    pub root: Option<Box<Node<K, V>>>,
}

impl<K: PartialOrd, V> SearchTree<K, V> for NaiveTree<K, V> {
    fn insert(&mut self, pair: Pair<K, V>) {
        fn _insert<K: PartialOrd, V>(pair: Pair<K, V>, pos: &mut Option<Box<Node<K, V>>>) {
            match pos {
                Some(ref mut node) => {
                    if node.pair.key < pair.key {
                        _insert(pair, &mut node.right);
                    }
                    else {
                        _insert(pair, &mut node.left);
                    }
                },
                None => {
                    *pos = Some(Box::new(Node { left: None, right: None, pair: pair }));
                }
            }
        }

        _insert(pair, &mut self.root);
    }


    fn search(&self, key: K) -> &Option<Box<Node<K, V>>> {
        fn _search<K: PartialOrd, V>(pos: &Option<Box<Node<K, V>>>, key: K) -> &Option<Box<Node<K, V>>> {
            match pos {
                Some(ref node) => {
                    if node.pair.key == key {
                        return pos;
                    }

                    if node.pair.key < key {
                        return _search(&node.right, key);
                    }
                    else {
                        return _search(&node.left, key);
                    }
                },
                None => {
                    return pos;
                }
            }
        }

        return _search(&self.root, key);
    }
}

