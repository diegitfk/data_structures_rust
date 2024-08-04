use data_structures::no_lineal::safety::binary_search_tree::BinarySearchTree;
fn main(){
    let mut tree: BinarySearchTree<u32> = BinarySearchTree::new();
    tree.insert_node_recursibly(100);
    tree.insert_node_recursibly(200);
    println!("{:?}" , tree);
}