use avl::AvlTree;

fn main() {
    let mut tree = AvlTree::new();

    tree.insert(21);
    tree.insert(34);
    tree.insert(14);
    tree.insert(11);
    tree.insert(15);
    tree.insert(16);
    tree.insert(22);
    tree.insert(23);
    tree.insert(35);
    tree.insert(24);
    tree.insert(25);
    tree.insert(1000);
    tree.insert(1001);
    println!("{}", tree);

    tree.remove(&23);
    println!("{}", tree);
    
    tree.remove(&35);
    println!("{}", tree);
    tree.remove(&34);
    println!("{}", tree);

    tree.remove(&1000);
    println!("{}", tree);
    // let v: Vec<i32> = tree.into_iter().collect();

    // println!("{:?}", v);
}
