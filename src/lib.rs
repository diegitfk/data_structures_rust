pub mod lineal{
    pub mod linked_list;
    pub mod double_linked_list;
}
pub mod no_lineal{
    pub mod binary_search_tree;
    pub mod avl_tree;
    pub mod rb_tree;
}
pub use lineal::linked_list::LinkedList;
pub use lineal::double_linked_list;
pub use no_lineal::binary_search_tree;
pub use no_lineal::avl_tree;
pub use no_lineal::rb_tree;