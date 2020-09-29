use crate::geometry::aabb::Aabb;
use crate::geometry::Boxable;

pub struct KdNode<T> {
    bounding_box: Aabb,
    pub item: T,
    pub left: Option<Box<Self>>,
    pub right: Option<Box<Self>>,
}

pub struct KdTree<T> {
    pub root: Option<Box<KdNode<T>>>,
}

impl<T> KdNode<T> {
    pub fn set_left(&mut self, left: Box<Self>) -> &mut Self {
        self.left = Some(left);
        self
    }

    pub fn set_right(&mut self, right: Box<Self>) -> &mut Self {
        self.right = Some(right);
        self
    }

    pub fn set_left_right(&mut self, left: Box<Self>, right: Box<Self>) -> &mut Self {
        self.set_left(left);
        self.set_right(right);
        self
    }
}

impl<T: Boxable> KdTree<T> {
    fn new() -> Self {
        Self {
            root: None
        }
    }
}
