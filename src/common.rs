pub struct Pair<K: PartialOrd, V> {
    pub key : K,
    pub value: V,
}

pub struct Node<K: PartialOrd, V> {
    pub left: Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>,
    pub pair: Pair<K, V>,
}

pub trait SearchTree<K: PartialOrd, V> {
    fn insert(&mut self, pair: Pair<K, V>);
    fn search(&self, key: K) -> &Option<Box<Node<K, V>>>;
}
