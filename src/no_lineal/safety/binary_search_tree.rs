use num::Integer;
use std::fmt::{Debug, Display};
use std::cmp::Ordering;
///NodeTree<T>
/// ```text
///              +----------------------+
///         |----| LEFT | VALUE | RIGHT |----|
///         |    +----------------------+    |
/// ```
 
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
    fn is_leaf(&self) -> bool{
        self.left.is_none() && self.right.is_none()
    }
    fn have_one_children(&self) -> bool{
        (self.left.is_some() && self.right.is_none()) || (self.left.is_none() && self.right.is_some())
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
    fn insert_node_iterative(&mut self , value : T){
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
    //Obtenemos una referencia mutable al nodo
    fn find_mut_node(&mut self , value : T) -> Result<Option<&mut Box<NodeTree<T>>> , String>{
        Self::find_mut_recursive(&mut self.root, value)
    }
    fn find_mut_recursive(current_node : &mut Option<Box<NodeTree<T>>> , value : T) -> Result<Option<&mut Box<NodeTree<T>>> , String>{
        match current_node{
            None => Err(String::from("No se encuentra el nodo en el arbol")),
            Some(node) => {
                //Caso base: Encontramos el nodo
                if node.value == value{
                    return Ok(Some(node));
                }
                if value >= node.value{
                    Self::find_mut_recursive(&mut node.right, value)
                }else {
                    Self::find_mut_recursive(&mut node.left, value)
                }
            }
        }

    }
    ///### Obtención del padre de un nodo en el arbol
    /// Este metodo del arbol permite obtener una referencia mutable al padre de un nodo en el arbol.
    fn find_parent_for_node(&mut self , value : T) -> Result<&mut Box<NodeTree<T>> , String>{
        Self::find_parent_to(&mut self.root, value)
    }
    fn find_parent_to(current_node : &mut Option<Box<NodeTree<T>>>, value : T) -> Result<&mut Box<NodeTree<T>> , String>{
        //Casos base
        match current_node {
            None => {Err(String::from("No se encontro el nodo"))},
            Some(node) => {
                //Si el nodo es hijo derecho o Si el nodo es hijo izquierdo
                if (node.right.as_ref().map_or(false, |n| n.value == value)) || (node.left.as_ref().map_or(false , |n| n.value == value)){
                    return Ok(node); 
                }
                //Casos de Recursión
                //Si el valor es mayor al valor del nodo actual
                if value >= node.value{
                    Self::find_parent_to(&mut node.right, value)
                }else {
                    Self::find_parent_to(&mut node.left, value)
                }
            } 
        }
    }
    ///### Eliminar un nodo del arbol binario
    /// Siempre y cuando la situación y estructura en tiempo de ejecución generada del arbol sea balanceada,
    /// se garantiza que las eliminaciones se hacen en tiempo O(log(n)) en caso de que no sea la estructura ideal
    /// se garantiza un tiempo O(n).
    /// ### Casos de eliminación
    /// - `Caso 1`: Eliminación de un nodo hoja o sin hijos
    /// Si el estado del nodo es el siguiente:
    /// ```text
    ///                      |
    ///                     del
    ///             +-------------------+
    ///         |---|LEFT | 100  | RIGHT|---|
    ///            +--------------------+
    /// 
    /// ```
    /// o sea su hijo tanto izquierdo como derecho son None entonces simplemente se remueve el nodo del arbol
    /// o se asigna como tal el nodo en None.
    /// y se disminuye el contador de nodos en el arbol
    /// - `Caso 2`: Eliminación de un nodo con un solo hijo
    /// Si el estado del nodo es el siguiente
    ///```text
    ///                          |
    ///                         del
    ///                 +-------------------+
    ///             |---|LEFT | 100  | RIGHT|---|
    ///                +--------------------+   |
    ///                           +-------------------+
    ///                       |---| LEFT | 200 | RIGHT|---|
    ///                           +-------------------+ 
    /// 
    /// ```
    /// En dicho caso se remplaza el valor del nodo hijo por el que se elimina
    /// y se elimina el nodo hijo, para mantener la coherencia del arbol.
    /// ```text
    ///                          |
    ///                 +-------------------+
    ///             |---|LEFT | 200  | RIGHT|---|
    ///                +--------------------+   |
    ///                           +-------------------+
    ///                       |---| LEFT | 100 | RIGHT|---|
    ///                           +-------------------+ 
    ///                                  ^-> None
    /// ```
    /// Esto hay que tenerlo en cuenta para el nodo izquierdo como el derecho.
    /// 
    /// - `Caso 3`: Eliminación de un nodo con dos hijos
    /// 
    /// 
    /// 
    fn remove_node(&mut self, value : T){
        match self.find_mut_node(value){
            Err(msg) => {},
            Ok(node) =>{
                match node {
                    None => {},
                    Some(node) =>{
                        //Caso en el que nodo es hoja
                        if node.is_leaf(){
                            println!("El nodo {:?} es una Hoja" , node);
                        }
                        //Caso en que el nodo posee un hijo
                        else if node.have_one_children(){
                            println!("El nodo {:?}\n Es padre de un hijo!" , node);
                        }
                        //Caso en que el nodo posee dos hijos
                        else {
                            println!("El nodo {:?} es padre de dos hijos!" , node);
                        }
                    }
                    
                }

            }

        }        
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
    /// ### Recorrido PostOrder
    /// En el recorrido inorder se recorre primero recursivamente el subarbol derecho de la raiz, luego el nodo raiz
    /// y por ultimo recursivamente el subarbol izquierdo del nodo raiz
    /// ```text
    ///                     2°------> root
    ///                         +-------------------+
    ///                     |---|LEFT | 100  | RIGHT|---|
    ///                     |   +-------------------+  |
    ///                    /\                         /\
    ///           3°--->  / \             1°----->   / \ 
    ///                  /__\                       /__\
    /// ```
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
        tree.insert_node_iterative(30);
        tree.insert_node_iterative(50);
        tree.insert_node_iterative(20);
        tree.insert_node_iterative(70);
        tree.insert_node_iterative(90);
        tree.insert_node_iterative(10);
        println!("------- INORDER --------");
        tree.inorder_tree();
        println!("------ POSTORDER -------");
        tree.postorder_tree();
        tree.remove_node(90);
        tree.remove_node(70);
        tree.remove_node(30);
    }
}

