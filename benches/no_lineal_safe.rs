use data_structures::no_lineal::safety::{
    binary_search_tree::BinarySearchTree,
    avl_tree::AVLTree
};
use criterion::{criterion_group , criterion_main , Criterion , black_box};
use std::sync::{Arc , Mutex , MutexGuard};

fn bench_insertion_operation_recursive_bst(c : &mut Criterion){
    //bad case
    c.bench_function("bad lineal case insertion BST", |b: &mut criterion::Bencher| {
        b.iter(|| {
            let tree : Arc<Mutex<BinarySearchTree<i32>>> = Arc::new(Mutex::new(BinarySearchTree::new()));
            let mut tree: MutexGuard<BinarySearchTree<i32>> = tree.lock().unwrap();
            for i in 1..100 {
                tree.insert_node_recursibly(black_box(i));
            }
            black_box(tree.len());
            *tree = BinarySearchTree::new(); // Reinicia el árbol
        });
    });

}
fn bench_deletion_operations_recursives_bst(c : &mut Criterion){
    c.bench_function("bad lineal case deletion BST", |b: &mut criterion::Bencher|{
        b.iter(||{
            let tree : Arc<Mutex<BinarySearchTree<i32>>> = Arc::new(Mutex::new(BinarySearchTree::new()));
            {
                let mut tree: MutexGuard<BinarySearchTree<i32>> = tree.lock().unwrap();
                for i in 1..=100 {
                    tree.insert_node_recursibly(black_box(i));
                }
            }
            let mut tree: MutexGuard<BinarySearchTree<i32>> = tree.lock().unwrap();
            for i in (1..=100).rev() {
                tree.remove_node_recursibly(black_box(i));
            }
            black_box(tree.len());
            *tree = BinarySearchTree::new();
        });
    });
}
fn bench_operation_recursive_insertion_avl(c : &mut Criterion){
    let avl_tree: Arc<Mutex<AVLTree<i32>>> = Arc::new(Mutex::new(AVLTree::new()));
    c.bench_function("case insertion recursive AVL", |b: &mut criterion::Bencher|{
        b.iter(||{
            let mut avl_tree: MutexGuard<AVLTree<i32>> = avl_tree.lock().unwrap();
            for i in 1..=1_00{
                avl_tree.insert_node(black_box(i));
            }
            black_box(avl_tree.len());
            *avl_tree = AVLTree::new();
        })
    });
}
fn bench_operation_recursive_deletion_avl(c : &mut Criterion){
    c.bench_function("case deletion recursive AVL", |b: &mut criterion::Bencher|{
        b.iter(||{
            let avl_tree: Arc<Mutex<AVLTree<i32>>> = Arc::new(Mutex::new(AVLTree::new()));
            let mut avl_tree: MutexGuard<AVLTree<i32>> = avl_tree.lock().unwrap();
            for i in 1..=1_00{
                avl_tree.insert_node(black_box(i));
            }
            for i in (1..=1_00).rev() {
                avl_tree.remove_node(black_box(i));
            }
            black_box(avl_tree.len());
            *avl_tree = AVLTree::new();
        })
    });
}
criterion_group!(bench , bench_insertion_operation_recursive_bst,bench_deletion_operations_recursives_bst);
criterion_group!(benching , bench_operation_recursive_insertion_avl , bench_operation_recursive_deletion_avl);
criterion_main!(benching);
