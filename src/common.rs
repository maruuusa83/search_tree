pub struct Pair<K: PartialOrd, V> {
    pub key : K,
    pub value: V,
}

pub struct Node<K: PartialOrd, V> {
    pub left: Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>,
    pub pair: Pair<K, V>,
}

pub trait NodeTrait<K: PartialOrd, V> {
    type N;

    fn left(&self) -> &Option<Box<Self::N>>;
    fn right(&self) -> &Option<Box<Self::N>>;
    fn pair(&self) -> &Pair<K, V>;
}

impl<K: PartialOrd, V> NodeTrait<K, V> for Node<K, V> {
    type N = Node<K, V>;

    fn left(&self) -> &Option<Box<Self::N>> { return &self.left; }
    fn right(&self) -> &Option<Box<Self::N>> { return &self.right; }
    fn pair(&self) -> &Pair<K, V> { return &self.pair; }
}

pub trait SearchTree<K: PartialOrd, V> {
    type N: NodeTrait<K, V>;

    fn insert(&mut self, pair: Pair<K, V>);
    fn search(&self, key: K) -> &Option<Box<Self::N>>;
}

