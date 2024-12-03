use std::cmp;

pub struct AvlTree<V> {
    root: Option<Box<AvlTreeNode<V>>>,
}

impl<V: Ord + Copy> AvlTree<V> {
    pub fn new() -> AvlTree<V> {
        AvlTree { root: None }
    }

    pub fn insert(&mut self, value: V) {
        match self.root {
            None => {
                self.replace(Box::new(AvlTreeNode::new(value)));
                return;
            }
            Some(ref mut node) => {
                match value.cmp(&node.val) {
                    cmp::Ordering::Less => {
                        node.left.insert(value);
                    }
                    cmp::Ordering::Greater => {
                        node.right.insert(value);
                    }
                    cmp::Ordering::Equal => return,
                };

                node.update_height();
            }
        }

        self.balance(&value);
    }

    pub fn contains(&self, value: V) -> bool {
        match self.root {
            None => false,
            Some(ref node) => match value.cmp(&node.val) {
                std::cmp::Ordering::Less => node.left.contains(value),
                std::cmp::Ordering::Greater => node.right.contains(value),
                std::cmp::Ordering::Equal => true,
            },
        }
    }

    fn get_height(&self) -> Height {
        self.root.as_ref().map_or(0, |node| node.height)
    }

    fn get_val(&self) -> Option<&V> {
        self.root.as_ref().map(|node| &node.val)
    }

    fn replace(&mut self, other: Box<AvlTreeNode<V>>) -> Option<Box<AvlTreeNode<V>>> {
        self.root.replace(other)
    }

    fn take_into_subtree(&mut self) -> AvlTree<V> {
        AvlTree {
            root: self.root.take(),
        }
    }

    fn take_root(&mut self) -> Option<Box<AvlTreeNode<V>>> {
        self.root.take()
    }

    fn rotate_left(&mut self) {
        let mut x = self.take_root().expect("Can't rotate left: root is empty");

        let mut y = x
            .right
            .take_root()
            .expect("Can't rotate left: no right child");
        let t2 = y.left.take_into_subtree();

        x.right = t2;
        x.update_height();
        y.left.replace(x);
        y.update_height();

        self.replace(y);
    }

    fn rotate_right(&mut self) {
        let mut y = self.take_root().expect("Can't rotate right: root is empty");

        let mut x = y
            .left
            .root
            .take()
            .expect("Can't rotate right: no left child");
        let t2 = x.right.take_into_subtree();

        y.left = t2;
        y.update_height();
        x.right.replace(y);
        x.update_height();

        self.replace(x);
    }

    fn balance(&mut self, value: &V) {
        match self.root {
            None => return,
            Some(ref mut node) => {
                let rotation = node.get_rotation(value);
                match rotation {
                    Rotate::Left(inner_rotation) => match inner_rotation {
                        InnerRotate::Left => {
                            self.rotate_right();
                        }
                        InnerRotate::Right => {
                            node.left.rotate_left();
                            self.rotate_right();
                        }
                    },
                    Rotate::Right(inner_rotation) => match inner_rotation {
                        InnerRotate::Left => {
                            node.right.rotate_right();
                            self.rotate_left();
                        }
                        InnerRotate::Right => {
                            self.rotate_left();
                        }
                    },
                    Rotate::Not => (),
                }
            }
        }
    }
}

impl<V: Ord + Copy> IntoIterator for AvlTree<V> {
    type Item = V;
    type IntoIter = <Vec<V> as IntoIterator>::IntoIter;

    fn into_iter(mut self) -> Self::IntoIter {
        let mut cur_tree = self.take_into_subtree();

        let mut stack: Vec<AvlTree<V>> = Vec::new();
        let mut queue: Vec<V> = Vec::new();

        while cur_tree.root.is_some() || stack.len() > 0 {
            while cur_tree.root.is_some() {
                let mut node = cur_tree;
                cur_tree = node.root.as_mut().unwrap().left.take_into_subtree();
                stack.push(node);
            }

            let handle_node = *stack.pop().unwrap().root.unwrap();

            queue.push(handle_node.val);

            cur_tree = handle_node.right;
        }

        queue.into_iter()
    }
}

type Height = u64;

struct AvlTreeNode<V> {
    val: V,
    height: Height,
    left: AvlTree<V>,
    right: AvlTree<V>,
}

enum InnerRotate {
    Left,
    Right,
}

enum Rotate {
    Left(InnerRotate),
    Right(InnerRotate),
    Not,
}

impl<V: Ord + Copy> AvlTreeNode<V> {
    fn new(value: V) -> AvlTreeNode<V> {
        AvlTreeNode {
            val: value,
            height: 1,
            left: AvlTree::new(),
            right: AvlTree::new(),
        }
    }

    fn update_height(&mut self) {
        self.height = 1 + cmp::max(self.left.get_height(), self.right.get_height());
    }

    fn get_rotation(&self, value: &V) -> Rotate {
        let balance = self.left.get_height() as i64 - self.right.get_height() as i64;

        match balance {
            1.. => {
                let left_val = &self
                    .left
                    .get_val()
                    .expect("get_rotation: left does not exist, but bal > 1");
                match value.cmp(left_val) {
                    cmp::Ordering::Less => Rotate::Left(InnerRotate::Left),
                    cmp::Ordering::Greater => Rotate::Left(InnerRotate::Right),
                    _ => panic!("get_rotation: new value and node value are equal: not allowed"),
                }
            }
            ..-1 => {
                let right_val = &self
                    .right
                    .get_val()
                    .expect("get_rotation: right does not exist, but bal < -1");
                match value.cmp(right_val) {
                    cmp::Ordering::Less => Rotate::Right(InnerRotate::Left),
                    cmp::Ordering::Greater => Rotate::Right(InnerRotate::Right),
                    _ => panic!("get_rotation: new value and node value are equal: not allowed"),
                }
            }
            _ => Rotate::Not,
        }
    }
}
