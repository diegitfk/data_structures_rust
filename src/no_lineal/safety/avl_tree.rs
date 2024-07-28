//! ### Sobre los arboles AVL
//! ### ¿Cual es el verdadero problema de un Arbol de Busqueda Binario?
//! - Uno de los principales problemas por los cuales un BST no es la mejor
//! de las opciones es debido a sus peores casos de complejidad temporal.
//! en un BST basta con que insertemos de manera incremental los valores de un nodo
//! para que el arbol tenga tanto operaciones como inserción, eliminacion y busqueda 
//! en complejidad temporal O(n), si ese es el caso nos sentiriamos conformes con una LinkedList.
//! de hecho en el peor de los casos de veria algo así.
//! ```
//!                          root
//!                 +-------------------+
//!             |---|LEFT | 100  | RIGHT|---|
//!                +--------------------+   |
//!                               +-------------------+
//!                           |---| LEFT | 200 | RIGHT|---|
//!                               +-------------------+ 
//!                                                    +-------------------+
//!                                                |---| LEFT | 300 | RIGHT|---|
//!                                                    +-------------------+ 
//!                                                                     +-------------------+
//!                                                                 |---| LEFT | 400 | RIGHT|---|
//!                                                                     +-------------------+ 
//! ```
//! ### Solución al caso O(n) con AVL
//! Bueno un arbol AVL sigue principios de balance acorde a criterios condicionales veamos cuales son ellos
//! ###### Altura de un Nodo
//! Sip, AVL ingresa el concepto de Altura de un Nodo en el arbol binario, dicha "Altura" es el sinonimo de encontrar
//! `el camino más largo desde ese nodo hasta la hoja`, bueno para mayor criterio, existe una formula que nos ayuda con 
//! el calculo de la altura Nodica, y es la siguiente.
//! ```
//!     max{height(left_child) , height(right_child)} + 1
//! ```
//! okey pero no hemos definido correctamente, cual es el verdadero proposito de un Arbol AVL
//! ###### Proposito de un arbol AVL
//! El criterio es el siguiente y nuestra misión es la siguiente.
//! `Las alturas de los hijos izquierdos y derechos de cada nodo obligatoriamente deben difererir en +|- 1`
//! todo esto conforme a la altura del actual nodo.
//! ```
//!                         
//!                             +-------------------+
//!                         |---|LEFT | 100  | RIGHT|---|
//!                         |  +--------------------+   |     
//!                        /\                          /\
//!                       /Hl\    |Hl - Hr| <= 1      /  \
//!                      /__ \-------------|         / Hr \
//!                                        |------->/_____\
//!
//! ```
//! Si `Hl` > `Hr` => Hl - Hr = 1 y Hr - Hl = -1
//! Si `Hr` > `Hl` => `Hr` - `Hl` = 1 y `Hr` - `Hl` = -1
//! Por ello definimos el criterio como el valor absoluto de `Hl - Hr`
//! Hipoteticamente este es nuestro BST ideal!!, parece realmente un sueño, debido a que garantiza en sus mejores operaciones
//! `O(log n)` que es infinitamente mejor a `O(n)`
//! ###### Rotaciones la escencia de arboles autoequilibrados
//! Para hacer posible dicho criterio, tenemos una herramienta muy poderosa que son las rotaciones de los nodos en el arbol
//! siempre y cuando se cumplan dichos criterios tenemos 2 rotaciones con sus simetricos opuestos.
//! - `Rotaciones Simples`: Son rotaciones que siguen una sola orientación en el momento del balanceo
//! ###### 1._`Rotacion simple hacia la derecha`
//! Dado el caso donde el factor de balance del nodo desbalanceado sea 2, como en este caso.
//! ```
//!                                     +-------------------+
//!                                 |---|LEFT |  A   | RIGHT|---| H = (1 , -1) + 1 = 2 (Nodo no balanceado)
//!                                 |  +--------------------+   |
//!                     +-----------------+             
//!                 |---|LEFT |  B  | RIGHT|---| H = (0 , -1) + 1 = 1 (Balanceado)   
//!                 |   +-----------------+
//!     +-----------------+             
//! |---|LEFT | C  | RIGHT|---|  H= (-1 , -1) + 1 = 0  (Balanceado)
//!     +-----------------+
//! ```
//! Aplicamos una rotacion hacia la derecha de la siguiente manera teniendo en cuenta al nodo A y al Nodo B
//! ```
//!                     +-------------------+
//!                 |---|LEFT |  B  | RIGHT |---| <- H = 1 (Balanceado)
//!                 |  +--------------------+   |
//!     +-----------------+             +-------------------+
//! |---|LEFT | C  | RIGHT|---|    |---| LEFT |  A  | RIGHT |---|
//!     +-----------------+            +-------------------+ 
//!       ↑- H= 0(Balanceado)              ↑-H= 0 (Balanceado)
//! ```
//! Ahora cuando se ejecutan dichas rotaciones es necesario, comprender correctamente la reasignacion de los subarboles.
//! ```
//!                                     +-------------------+
//!                                 |---|LEFT |  Y   | RIGHT|---| H = (1 , -1) + 1 = 2 (Nodo no balanceado)
//!                                 |  +--------------------+   |
//!                     +-----------------+                    /\
//!                 |---|LEFT |  X  | RIGHT|---|              /C\
//!                 |   +-----------------+    |             /__\
//!                /\                         /\
//!               /A\                        /B\
//!              /__\                       /__\
//!     
//! ```
//! Queda de la siguiente manera
//!```
//!                     +-------------------+
//!                 |---|LEFT |  X  | RIGHT |---|
//!                 |  +--------------------+   |
//!                /\                    +-------------------+
//!               /A\               |---| LEFT |  Y  | RIGHT |---|
//!              /__\               |   +-------------------+    |
//!                                /\                           /\
//!                               /B\                          /C\
//!                              /__\                         /__\
//! 
//! ```
//!###### 2._`Rotacion Simple hacia la izquierda`
//! Dado el caso en el que el factor de balance del nodo desbalanceado es de -2 como en este caso.
//! ```
//!                         
//!                 +-------------------+
//!             |---|LEFT |  A   | RIGHT|---|
//!                +--------------------+   |
//!                               +-------------------+
//!                           |---| LEFT |  B  | RIGHT|---|
//!                               +-------------------+ 
//!                                                    +-------------------+
//!                                                |---| LEFT |  C  | RIGHT|---|
//!                                                    +-------------------+ 
//! ```
//! Aplicamos una rotación hacia la izquierda de la siguiente manera
//! ```
//!                     +-------------------+
//!                 |---|LEFT |  B  | RIGHT |---| <- H = 1 (Balanceado)
//!                 |  +--------------------+   |
//!     +-----------------+             +-------------------+
//! |---|LEFT | C  | RIGHT|---|    |---| LEFT |  A  | RIGHT |---|
//!     +-----------------+            +-------------------+ 
//!       ↑- H= 0(Balanceado)              ↑-H= 0 (Balanceado)
//! 
//! ```
//! En este caso las consideraciones de los subarboles son las siguiente,
//! en el caso original tenemos esto.
//! ```
//!                                     +-------------------+
//!                                 |---|LEFT |  Y   | RIGHT|---| H = (1 , -1) + 1 = 2 (Nodo no balanceado)
//!                                 |  +--------------------+   |
//!                                /\                   +-----------------+
//!                               /A\              |---|LEFT |  X  | RIGHT|---|             
//!                              /__\              |   +-----------------+    |             
//!                                               /\                         /\
//!                                              /B\                        /C\
//!                                             /__\                       /__\
//! ```
//! La rotacion cambia los punteros de la siguiente manera.
//!```
//!                          +-------------------+
//!                     |---|LEFT |  X  | RIGHT |---|
//!                     |  +--------------------+   |
//!            +-------------------+               /\
//!       |---| LEFT |  Y  | RIGHT |---|          /C\
//!       |   +-------------------+    |         /__\
//!      /\                           /\
//!     /A\                          /B\
//!    /__\                         /__\
//! 
//! ```
//! - `Rotaciones Dobles`:
use num::Integer;
use std::{cmp::Ordering, fmt::{Debug, Display}, isize};
#[derive(Debug)]
struct AVLNode<T>{
    left : Option<Box<AVLNode<T>>>,
    value : T,
    right : Option<Box<AVLNode<T>>>,
    height : isize
}
impl <T> AVLNode<T>
where T : Integer + Clone + Copy + Display + Debug + Ord {
    ///En teoria todos los nodos insertados, se insertan como Hojas, es por ello que no necesitamos especificar realmente un
    ///calculo complejo de la altura. quedese con esta idea, cada nodo creado es una Hoja.
    fn new(value : T) -> Self{
        Self { 
            left: None, 
            value, 
            right: None,
            height : 0
        }
    }
    pub fn height(&self , node : &Option<Box<AVLNode<T>>>) -> isize{
        match node {
            None => { //Caso Base: El nodo en el arbol es nulo
                -1
            },
            Some(n) => { //Caso Recursivo: Retornamos la altura del nodo
                n.height
            }
            
        }
    }
    //Esta función define la altura del nodo en el arbol acorde a sus subarboles.
    pub fn update_height(&mut self , node : &mut Option<Box<AVLNode<T>>>){
        match node {
            None => {},
            Some(n) => {
                let height_left = self.height(&n.left);
                let height_right = self.height(&n.right);
                n.height = *[height_left, height_right].iter().max().unwrap() + 1;
            }
        }
    }
}
#[derive(Debug)]
struct AVLTree<T>{
    root : Option<Box<AVLNode<T>>>,
    size : usize 
}
impl <T> AVLTree<T> 
where T : Integer + Clone + Copy + Display + Debug + Ord{
    fn new() -> Self{
        Self{
            root : None,
            size : 0
        }
    }
    pub fn height(node : &mut Option<Box<AVLNode<T>>>) -> isize{
        match node {
            None => { //Caso Base: El nodo en el arbol es nulo
                -1
            },
            Some(n) => { //Caso Recursivo: Retornamos la altura del nodo
                n.height
            }
            
        }
    }
    //Esta función define la altura del nodo en el arbol acorde a sus subarboles.
    pub fn update_height_node(node : &mut Option<Box<AVLNode<T>>>){
        match node {
            None => {},
            Some(n) => {
                let height_left = Self::height(&mut n.left);
                let height_right = Self::height(&mut n.right);
                n.height = *[height_left, height_right].iter().max().unwrap() + 1;
            }
        }
    }
    fn balance_factor(node : &mut Option<Box<AVLNode<T>>>) -> isize{
        match node {
            None =>{todo!("")},
            Some(n) => {
                Self::height(&mut n.left) - Self::height(&mut n.right)
            }
        }
    }

    ///Esta función recibe como parametros el nodo desbalanceado, para ejecutar una rotación simple hacia la derecha
    ///de balance.s
    pub fn simple_rotation_right(mut node : Option<Box<AVLNode<T>>>){
        match node{
            None => {},
            Some(ref mut n) => {
                let mut left_child = n.left.take(); //Nodo izquierdo
                let split_subtree = left_child.as_mut().map_or_else(
                    || {
                        None
                    }, //Si left es None, no hay subarbol derecho 
                    |some_node: &mut Box<AVLNode<T>>| -> Option<Box<AVLNode<T>>> { //si left es some retorna el Opcionalmente el subarbol derecho
                        some_node.right.take()
                    }
                );
                //Si el subarbol derecho es some al nodo desbalanceado le asignamos en el puntero izquierdo el subarbol
                if split_subtree.is_some(){
                    n.left = split_subtree;
                }else {
                    n.left = None; //Sino asignamos el puntero izquierdo en None.
                }
                //Si left_child es Some, asignamos left_child.right en el nodo desbalanceado
                if let Some(ref mut node_left) = left_child{
                    node_left.right = node;
                    Self::update_height_node(&mut node_left.right); //Actualizamos al nodo desbalanceado.
                }
                println!("{:?}" , left_child);
            }
        }
    }
    fn simple_rotation_left(mut node : Option<Box<AVLNode<T>>>){

    }
    fn right_left_rotation(){

    }
    fn left_right_rotation(){

    }
    pub fn insert_node(&mut self , value : T){
        self.root = Self::insert_recursive(self.root.take(), value);
        self.size += 1;
    }
    fn insert_recursive(mut node : Option<Box<AVLNode<T>>> , value : T) -> Option<Box<AVLNode<T>>>{
        match node {
            None => {
            //Caso base donde insertamos el nodo
            Some(Box::new(AVLNode::new(value)))
            },
            Some(ref mut n) => {
                match n.value.cmp(&value) {
                    Ordering::Equal => {todo!("")},
                    Ordering::Greater => { // Caso recursivo
                        n.left = Self::insert_recursive(n.left.take(), value);
                        Self::update_height_node(&mut n.left);
                        Self::update_height_node(&mut node);
                        println!("{}" , Self::balance_factor(&mut node));
                        node
                    },
                    Ordering::Less => { //Caso Recursivo
                        n.right = Self::insert_recursive(n.right.take(), value);
                        Self::update_height_node(&mut n.right); //Actualización de la altura del nodo
                        Self::update_height_node(&mut node); //Actualización de la altura del nodo
                        println!("{}" , Self::balance_factor(&mut node));
                        node
                    }
                }
            }            
        }
    }

}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_insertion(){
        let mut tree: AVLTree<i32> = AVLTree::new();
        tree.insert_node(100);
        tree.insert_node(50);
        tree.insert_node(40);
        println!("{:?}" , tree);
    }
    #[test]
    fn test_simple_rotation_right(){
        let mut tree: AVLTree<i32> = AVLTree::new();
        tree.insert_node(100);
        tree.insert_node(50); //left_child no posee sub arbol derecho
        tree.insert_node(40);
        AVLTree::simple_rotation_right(tree.root.take());
        tree.insert_node(100);
        tree.insert_node(50); //left_child posee sub arbol derecho
        tree.insert_node(40);
        tree.insert_node(60);
        AVLTree::simple_rotation_right(tree.root.take());
        tree.insert_node(100);
        tree.insert_node(120);
        tree.insert_node(50); //están todos los arboles requeridos
        tree.insert_node(40);
        tree.insert_node(60);
        AVLTree::simple_rotation_right(tree.root.take());
    }
}