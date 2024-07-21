use num::Integer;
use std::fmt::{Debug, Display};
use std::cmp::Ordering;
///NodeTree<T>
/// ```text
///              +----------------------+
///              |      |       |       |
///              | LEFT | VALUE | RIGHT |
///              |      |       |       |
///              +----------------------+
///             /                        \
///            /                         \
///          |/_                         _\/
/// ```
/// 
/// 
/// 
#[derive(Debug)]
struct NodeTree<T>{
    value : T,
    left : Option<Box<NodeTree<T>>>,
    right : Option<Box<NodeTree<T>>>
}

impl<T> NodeTree<T>
where T : Integer + Clone + Copy + Display {
    fn new(value : T) -> Self{
        Self{
            value,
            left: None, 
            right: None
        }
    }
    fn right_is_none(&self) -> bool{
        self.right.is_none()
    }
    fn left_is_none(&self) ->bool{
        self.left.is_none()
    }

    fn get_right(&self) -> Option<&Box<NodeTree<T>>>{
        match self.right{
            None => {None},
            Some(_) => self.right.as_ref()
        }
    }
    fn get_mut_right(&mut self) -> Option<&mut Box<NodeTree<T>>>{
        match self.right {
            None => None,
            Some(_) => self.right.as_mut()
        }
    }
    fn get_left(&self) -> Option<&Box<NodeTree<T>>>{
        match self.left{
            None => {None},
            Some(_) => self.left.as_ref()
        }
    }
    fn get_mut_left(&mut self) -> Option<&mut Box<NodeTree<T>>>{
        match self.left {
            None => None,
            Some(_) => self.left.as_mut()
            
        }
    }
    
}
#[derive(Debug)]
struct BinarySearchTree<T>{
    root : Option<Box<NodeTree<T>>>,
    size : usize
}
impl <T> BinarySearchTree<T>
where T : Integer + Clone + Copy + Display + Debug{
    fn new() -> Self{
        Self{
            root : None, 
            size : 0
        }
    }
    ///## Insert_Node
    /// La inserción en una arbol binario de busqueda posee un complejidad temporal de O(log(n)) y en el pero de los casos O(n)
    /// #### Casos de inserción
    /// - `Caso 1`: El nodo raiz es None
    /// ```text
    ///     insert_node(100);
    ///                     root
    ///             +-------------------+
    ///         |---|LEFT | 100  | RIGHT|---|
    ///            +--------------------+
    /// ```
    /// - `Caso 2`: El valor a insertar es mayor al nodo actual
    /// ```text
    ///     insert_node(200)
    ///                     root
    ///             +-------------------+
    ///         |---|LEFT | 100  | RIGHT|---|
    ///            +--------------------+   |
    ///                           +-------------------+
    ///                       |---| LEFT | 200 | RIGHT|---|
    ///                           +-------------------+ 
    /// ```
    /// - `Caso 3`: El valor a insertar es menor al nodo actual
    /// ```text
    ///         insert_node(90)
    ///                             root
    ///                     +-------------------+
    ///                 |---|LEFT | 100  | RIGHT|---|
    ///                 |  +--------------------+   |
    ///     +-----------------+             +-------------------+
    /// |---|LEFT | 90 | RIGHT|---|    |---| LEFT | 200 | RIGHT|---|
    ///     +-----------------+            +-------------------+ 
    /// 
    /// ```
    /// Tip: Tenemos que iterar constantemente de root al siguiente nodo hasta encontrar un espacio adecuado
    fn insert_node(&mut self , value : T){
        let new_node: Option<Box<NodeTree<T>>> = Some(Box::new(NodeTree::new(value)));
        match self.root {
            None => {
                self.root = new_node;
                self.size += 1;
            },
            Some(ref mut node) => {
                let mut current: &mut Option<Box<NodeTree<T>>> = if value >= node.value{
                        &mut node.right
                    }else {
                        &mut node.left
                    };
                while let Some(node) = current{
                    if value >= node.value{
                        current = &mut node.right;
                    }else {
                        current = &mut node.left;
                    }
                }
                *current = new_node;
                self.size += 1;
            }
        }
    }
    fn find_node(&self , value : T) -> Result<&Option<Box<NodeTree<T>>> , String> {
        let mut current = &self.root;
        while let Some(node) = current{
            if node.value == value{
                break;
            }
            else if value >= node.value {
                current = &node.right;
            }
            else {
                current = &node.left;
            }
        }
        if current.is_none(){
            Err(String::from("No se encuentra el valor en el arbol"))
        }else {
            Ok(current)
        }
    }
    fn remove_node(){
        todo!("")
    }
    /// ### Recorrido Inorder
    /// En el recorrido inorder se recorre primero recursivamente el subarbol izquierdo de la raiz, luego el nodo raiz
    /// y por ultimo recursivamente el subarbol derecho del nodo raiz
    /// ```text
    ///                     2°------> root
    ///                         +-------------------+
    ///                     |---|LEFT | 100  | RIGHT|---|
    ///                     |   +-------------------+  |
    ///                    /\                         /\
    ///           1°--->  / \             3°----->   / \ 
    ///                  /__\                       /__\
    /// ```
    fn inorder_tree(&self){
        Self::inorder(&self.root);
    }
    fn inorder(node : &Option<Box<NodeTree<T>>>){
        match node {
            Some(ref node) => {
                Self::inorder(&node.left);
                println!("{}" , node.as_ref().value);
                Self::inorder(&node.right);
            },
            None => {}
        }
    }
    fn postorder_tree(&self){
        Self::postorder(&self.root);
    }
    fn postorder(node : &Option<Box<NodeTree<T>>>){
        match node {
            Some(ref node) => {
                Self::postorder(&node.right);
                println!("{}" , node.as_ref().value);
                Self::postorder(&node.left);
            }, 
            None => {}
            
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn mut_to_node(){
        let mut node1 : NodeTree<i32> = NodeTree::new(30);
        node1.right = Some(Box::new(NodeTree::new(50)));
        node1.left = Some(Box::new(NodeTree::new(10)));
        {
            let mut node_left: &mut Box<NodeTree<i32>> = node1.get_mut_left().unwrap();
            node_left.right = None;
            node_left.left = None;
        }
        {
            let mut node_right: &mut Box<NodeTree<i32>> = node1.get_mut_right().unwrap();
            node_right.right = Some(Box::new(NodeTree::new(60)));
            node_right.left = Some(Box::new(NodeTree::new(40)));
        }
        println!("{:?}" , node1);
    }
    #[test]
    fn test_insert(){
        let mut tree : BinarySearchTree<i32> = BinarySearchTree::new();
        tree.insert_node(30);
        tree.insert_node(50);
        tree.insert_node(20);
        tree.insert_node(70);
        tree.insert_node(90);
        tree.insert_node(10);
        println!("------- INORDER --------");
        tree.inorder_tree();
        println!("------ POSTORDER -------");
        tree.postorder_tree();
    }
}

