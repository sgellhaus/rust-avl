use avl::AvlTree;

fn main() {
    let mut tree = AvlTree::new();

    tree.insert(11);
    tree.insert(21);
    tree.insert(34);
    tree.insert(14);
    tree.insert(11);

    println!("{}", tree.contains(1));
    println!("{}", tree.contains(11));

    let v: Vec<i32> = tree.into_iter().collect();

    println!("{:?}", v);
}
