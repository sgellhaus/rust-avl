use std::{
    cmp,
    fmt::{self, Formatter},
};

pub struct AvlTree<V> {
    root: Option<Box<AvlTreeNode<V>>>,
}

impl<V: Ord + Copy + fmt::Display> AvlTree<V> {
    pub fn new() -> AvlTree<V> {
        AvlTree { root: None }
    }

    pub fn min(&self) -> Option<V> {
        match self.root {
            None => None,
            Some(ref node) => match node.left.root {
                None => Some(node.val),
                Some(_) => node.left.min(),
            },
        }
    }

    pub fn max(&self) -> Option<V> {
        match self.root {
            None => None,
            Some(ref node) => match node.right.root {
                None => Some(node.val),
                Some(_) => node.right.max(),
            },
        }
    }

    pub fn insert(&mut self, value: V) {
        match self.root {
            None => {
                self.root.replace(Box::new(AvlTreeNode::new(value)));
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

    fn rotate_left(&mut self) {
        let mut x = self.root.take().expect("Can't rotate left: root is empty");

        let mut y = x
            .right
            .root.take()
            .expect("Can't rotate left: no right child");
        let t2 = y.left.root.take();

        x.right.root = t2;
        x.update_height();
        y.left.root.replace(x);
        y.update_height();

        self.root.replace(y);
    }

    fn rotate_right(&mut self) {
        let mut y = self.root.take().expect("Can't rotate right: root is empty");

        let mut x = y
            .left
            .root
            .take()
            .expect("Can't rotate right: no left child");
        let t2 = x.right.root.take();

        y.left.root = t2;
        y.update_height();
        x.right.root.replace(y);
        x.update_height();

        self.root.replace(x);
    }

    fn balance(&mut self, value: &V) {
        match self.root {
            None => return,
            Some(ref mut node) => {
                let balance = node.left.get_height() as i64 - node.right.get_height() as i64;

                match balance {
                    2.. => {
                        let left_val = node
                            .left
                            .get_val()
                            .expect("balance: left does not exist, but bal > 1");
                        match value.cmp(left_val) {
                            cmp::Ordering::Less => {
                                self.rotate_right();
                            }
                            cmp::Ordering::Greater => {
                                node.left.rotate_left();
                                self.rotate_right();
                            }
                            _ => panic!("balance: new value and node value are equal: not allowed"),
                        }
                    }
                    ..-1 => {
                        let right_val = node
                            .right
                            .get_val()
                            .expect("balance: right does not exist, but bal < -1");
                        match value.cmp(right_val) {
                            cmp::Ordering::Less => {
                                node.right.rotate_right();
                                self.rotate_left();
                            }
                            cmp::Ordering::Greater => {
                                self.rotate_left();
                            }
                            _ => panic!("balance: new value and node value are equal: not allowed"),
                        }
                    }
                    -1..=1 => (),
                }
            }
        }
    }

    fn get_level_string(&self, descend: usize, level: usize, node_char_width: usize) -> String {
        match self.root {
            None => match descend {
                0 => format!("{:node_char_width$}", ""),
                _ => {
                    let space_between_nodes =
                        ((2 as usize).pow(level as u32) - 1) * node_char_width;
                    format!(
                        "{}{:space_between_nodes$}{}",
                        self.get_level_string(descend - 1, level, node_char_width),
                        "",
                        self.get_level_string(descend - 1, level, node_char_width)
                    )
                }
            },
            Some(ref node) => match descend {
                0 => {
                    format!("{:^node_char_width$}", format!("{}", node))
                }
                _ => {
                    let space_between_nodes =
                        ((2 as usize).pow(level as u32) - 1) * node_char_width;
                    format!(
                        "{}{:space_between_nodes$}{}",
                        node.left
                            .get_level_string(descend - 1, level, node_char_width),
                        "",
                        node.right
                            .get_level_string(descend - 1, level, node_char_width)
                    )
                }
            },
        }
    }
}

impl<V: Ord + Copy + fmt::Display> fmt::Display for AvlTree<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.root {
            None => writeln!(f, ""),
            Some(ref node) => {
                let node_char_width = format!(
                    "{}",
                    self.max().expect("No max even though root node exists")
                )
                .len();
                let mut tree = String::new();
                for descend in 0..node.height {
                    let initial_space = ((2 as usize).pow((node.height - descend - 1) as u32) - 1)
                        * node_char_width;
                    tree.push_str(&format!(
                        "{:initial_space$}{}\n",
                        "",
                        &self.get_level_string(descend, node.height - descend, node_char_width)
                    ));
                }
                write!(f, "{}", tree)
            }
        }
    }
}

impl<V: Ord + Copy + fmt::Display> IntoIterator for AvlTree<V> {
    type Item = V;
    type IntoIter = <Vec<V> as IntoIterator>::IntoIter;

    fn into_iter(mut self) -> Self::IntoIter {
        let mut cur_node = self.root.take();

        let mut stack: Vec<AvlTreeNode<V>> = Vec::new();
        let mut queue: Vec<V> = Vec::new();

        loop {
            while let Some(mut node) = cur_node {
                cur_node = node.left.root.take();
                stack.push(*node);
            }

            match stack.pop() {
                Some(mut node) => {
                    queue.push(node.val);
                    cur_node = node.right.root.take();
                }
                None => match cur_node {
                    Some(_) => continue,
                    None => break
                },
            }
        }

        queue.into_iter()
    }
}

type Height = usize;

struct AvlTreeNode<V> {
    val: V,
    height: Height,
    left: AvlTree<V>,
    right: AvlTree<V>,
}

impl<V: Ord + Copy + fmt::Display> AvlTreeNode<V> {
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
}

impl<V: fmt::Display> fmt::Display for AvlTreeNode<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}
