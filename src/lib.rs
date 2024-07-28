pub mod lineal{
    pub mod safety{
        pub mod linked_list;
        pub mod double_linked_list;
        pub mod stack;
    }
    pub mod unsafety{
        pub mod linked_list;
        pub mod double_linked_list;
        pub mod stack;
    }
}
pub mod no_lineal{
    pub mod safety{
        pub mod binary_search_tree;
        pub mod avl_tree;
        pub mod rb_tree;
    }
    pub mod unsafety{
        pub mod binary_search_tree;
        pub mod avl_tree;
        pub mod rb_tree;
    }
}
pub use lineal::safety::{
    double_linked_list::DoublyLinkedList , 
    linked_list::LinkedList
};
pub use no_lineal::safety::{binary_search_tree::BinarySearchTree};