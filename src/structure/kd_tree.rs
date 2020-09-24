struct KdNode<T> {
    item: T,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

pub struct KdTree<T> {
    root: Option<Box<KdNode<T>>>,
}
