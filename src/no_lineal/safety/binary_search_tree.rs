use num::Integer;
use std::fmt::{Debug, Display};
use std::cmp::Ordering;
use std::mem;
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
    fn right_is_some(&self) -> bool{
        self.right.is_some()
    }
    fn left_is_some(&self) -> bool{
        self.left.is_some()
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
pub struct BinarySearchTree<T>{
    root : Option<Box<NodeTree<T>>>,
    size : usize
}
impl <T> BinarySearchTree<T>
where T : Integer + Clone + Copy + Display + Debug{
    pub fn new() -> Self{
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
    pub fn insert_node_iterative(&mut self , value : T){
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
    pub fn find_node(&self , value : T) -> Result<&Option<Box<NodeTree<T>>> , String> {
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
    pub fn find_mut_node(&mut self , value : T) -> Result<&mut Option<Box<NodeTree<T>>> , String>{
        Self::find_mut_recursive(&mut self.root, value)
    }
    fn find_mut_recursive(current_node : &mut Option<Box<NodeTree<T>>> , value : T) -> Result<&mut Option<Box<NodeTree<T>>> , String>{
        //Casos bases
        //El arbol binario se encuentra vacio
        if current_node.is_none(){
            return Err(String::from("El arbol se encuentra vacio"));
        }
        if current_node.as_ref().map_or(false, |n| n.value == value){
            Ok(current_node)
        }
        else if current_node.as_ref().map_or(false, |n| value >= n.value){
            let right_node: &mut Box<NodeTree<T>> = current_node.as_mut().unwrap();
            Self::find_mut_recursive(&mut right_node.right, value)
        }else {
            let left_node: &mut Box<NodeTree<T>> = current_node.as_mut().unwrap();
            Self::find_mut_recursive(&mut left_node.left, value)
        }

    }
    ///### Obtención del padre de un nodo en el arbol
    /// Este metodo del arbol permite obtener una referencia mutable al padre de un nodo en el arbol.
    pub fn find_parent_for_node(&mut self , value : T) -> Result<&mut Box<NodeTree<T>> , String>{
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
    fn get_greater_left(node : &mut Option<Box<NodeTree<T>>>) -> Option<T>{
        let mut current_node: &mut Option<Box<NodeTree<T>>> = node;
        println!("Buscando el minimo en el subarbol derecho");
        let min_value: Option<T> = {
            while let Some(ref mut unwrapping_node) = current_node{
                if unwrapping_node.right_is_none(){
                    return Some(unwrapping_node.value);
                }
                current_node = &mut unwrapping_node.left;
            };
            None
        };
        current_node.take();
        min_value
    }
    fn get_smaller_right(node : &mut Option<Box<NodeTree<T>>>) -> Option<T>{
        let mut current_node: &mut Option<Box<NodeTree<T>>> = node;
        println!("Buscando el maximo del subarbol izquierdo");
        while let Some(ref mut unwrapping_node) = current_node{
            if unwrapping_node.left_is_none(){
                return Some(unwrapping_node.value);
            }
            current_node = &mut unwrapping_node.right;
        };  
        None
    }
    fn remove_node_by_value(node: &mut Option<Box<NodeTree<T>>>, value: &T) {
        if let Some(ref mut boxed_node) = node {
            if &boxed_node.value == value {
                // Este es el nodo que queremos eliminar
                if boxed_node.is_leaf() {
                    *node = None;
                } else if boxed_node.have_one_children() {
                    if boxed_node.right_is_some() {
                        *node = boxed_node.right.take();
                    } else {
                        *node = boxed_node.left.take();
                    }
                } else {
                    // Este caso no debería ocurrir si get_greater_left y get_smaller_right
                    // están implementados correctamente, pero lo manejamos por si acaso
                    if let Some(replacement_value) = Self::get_smaller_right(&mut boxed_node.right) {
                        boxed_node.value = replacement_value;
                        Self::remove_node_by_value(&mut boxed_node.right, &replacement_value);
                    }
                }
            } else if value < &boxed_node.value {
                Self::remove_node_by_value(&mut boxed_node.left, value);
            } else {
                Self::remove_node_by_value(&mut boxed_node.right, value);
            }
        }
    }
    fn remove_node(&mut self, value : T){
        match self.find_mut_node(value){
            Err(msg) => {},
            Ok(node) =>{
                match node {
                    None => {},
                    Some(unwrap_node) =>{
                        let del_value: T = unwrap_node.value;
                        if unwrap_node.is_leaf(){
                            *node = None;
                            self.size -= 1;
                            //del_value
                        }else if unwrap_node.have_one_children(){
                            //El unico hijo se encuentra a la derecha del nodo actual
                            if unwrap_node.right_is_some(){
                                unwrap_node.value = unwrap_node.right.as_ref().unwrap().value;
                                unwrap_node.right = None;
                                self.size -= 1;
                            }
                            //El unico hijo se encuentra a la izquierda del nodo actual
                            else {
                                unwrap_node.value = unwrap_node.left.as_ref().unwrap().value;
                                unwrap_node.left = None;
                                self.size -= 1                                
                            }

                        }else {
                            //Falta eliminar los nodos predecesores y sucesores
                            if let Some(value) = Self::get_greater_left(&mut unwrap_node.left){
                                Self::remove_node_by_value(&mut unwrap_node.left, &value);
                                unwrap_node.value = value;
                                self.size -= 1;

                            }else if let Some(value) = Self::get_smaller_right(&mut unwrap_node.right){
                                Self::remove_node_by_value(&mut unwrap_node.left, &value);
                                unwrap_node.value = value;
                                self.size -= 1;
                            }
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
    }//Caso en el que nodo es hoja
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
        tree.insert_node_iterative(40);
        tree.insert_node_iterative(70);
        tree.insert_node_iterative(90);
        tree.insert_node_iterative(10);
        tree.remove_node(30);
        tree.inorder_tree();
    }
}

