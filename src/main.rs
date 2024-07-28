use data_structures::BinarySearchTree;
fn main(){
    let mut tree: BinarySearchTree<u32> = BinarySearchTree::new();
    tree.insert_node_recursibly(100);
    tree.insert_node_recursibly(200);
    println!("{:?}" , tree);
}