pub struct Pair<T: PartialOrd, U> {
    pub key : T,
    pub value: U,
}
pub struct Node<T: PartialOrd, U> {
    pub left: Option<Box<Node<T, U>>>,
    pub right: Option<Box<Node<T, U>>>,
    pub pair: Pair<T, U>,
}

pub struct BTree<T: PartialOrd, U> {
    pub root: Option<Box<Node<T, U>>>,
}

impl<T: PartialOrd, U> BTree<T, U> {
    pub fn insert(&mut self, pair: Pair<T, U>) {
        fn _insert<T: PartialOrd, U>(pair: Pair<T, U>, pos: &mut Option<Box<Node<T, U>>>) {
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


    pub fn search(&self, key: T) -> &Option<Box<Node<T, U>>> {
        fn _search<T: PartialOrd, U>(pos: &Option<Box<Node<T, U>>>, key: T) -> &Option<Box<Node<T, U>>> {
            match pos {
                Some(ref node) => {
                    if (node.pair.key == key) {
                        return pos;
                    }

                    if (node.pair.key < key) {
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

