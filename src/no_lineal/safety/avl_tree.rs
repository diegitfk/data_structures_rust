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
use std::{cmp::Ordering, fmt::{Debug, Display}, isize, ops::Deref};
#[derive(Debug)]
pub struct AVLNode<T>{
    left : Option<Box<AVLNode<T>>>,
    value : T,
    right : Option<Box<AVLNode<T>>>,
    height : isize
}
impl <T> AVLNode<T>
where T : Integer + Clone + Copy + Display + Debug + Ord {
    ///En teoria todos los nodos insertados, se insertan como Hojas, es por ello que no necesitamos especificar realmente un
    ///calculo complejo de la altura. quedese con esta idea, cada nodo creado es una Hoja.
    pub fn new(value : T) -> Self{
        Self { 
            left: None, 
            value, 
            right: None,
            height : 0
        }
    }
}
#[derive(Debug)]
pub struct AVLTree<T>{
    root : Option<Box<AVLNode<T>>>,
    size : usize 
}
impl <T> AVLTree<T> 
where T : Integer + Clone + Copy + Display + Debug + Ord{
    pub fn new() -> Self{
        Self{
            root : None,
            size : 0
        }
    }
    pub(crate) fn height(node : &mut Option<Box<AVLNode<T>>>) -> isize{
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
    pub(crate) fn update_height_node(node : &mut Option<Box<AVLNode<T>>>){
        match node {
            None => {},
            Some(n) => {
                let height_left = Self::height(&mut n.left);
                let height_right = Self::height(&mut n.right);
                n.height = *[height_left, height_right].iter().max().unwrap() + 1;
            }
        }
    }
    pub(crate) fn balance_factor(node : &mut Option<Box<AVLNode<T>>>) -> isize{
        match node {
            None => 0 ,
            Some(n) => {
                Self::height(&mut n.left) - Self::height(&mut n.right)
            }
        }
    }

    /// Realiza una rotación simple a la derecha en un nodo desbalanceado de un árbol AVL.
    ///
    /// La rotación a la derecha se utiliza para balancear el árbol cuando se detecta un desbalance
    /// en el subárbol izquierdo del nodo desbalanceado.
    ///
    /// # Parámetros
    ///
    /// - `node`: `Option<Box<AVLNode<T>>>` - El nodo desbalanceado en el que se debe realizar la rotación
    ///   a la derecha. Este nodo es del tipo `Option<Box<AVLNode<T>>>`, donde `AVLNode<T>` es una
    ///   estructura que representa un nodo en el árbol AVL.
    ///
    /// # Retorno
    ///
    /// - `Option<Box<AVLNode<T>>>` - El nuevo nodo raíz después de realizar la rotación a la derecha.
    ///   Este nodo es del tipo `Option<Box<AVLNode<T>>>`.
    ///
    /// # Ejemplo de uso
    ///
    /// ```rust
    /// let mut tree = AVLTree::new();
    /// tree.insert_node(30);
    /// tree.insert_node(20);
    /// tree.insert_node(10);
    ///
    /// println!("{:?}", tree.root);
    ///
    /// let new_root = AVLTree::right_rotation(tree.root.take());
    ///
    /// println!("{:?}", new_root);
    /// ```
    ///
    /// # Caso Simple: Estructura antes de la rotación
    ///
    /// El árbol AVL antes de la rotación tiene la siguiente estructura:
    ///
    /// ```
    ///     30
    ///    /
    ///   20
    ///  /
    /// 10
    /// ```
    ///
    /// # Proceso de la Rotación a la Derecha (Caso Simple)
    ///
    /// 1. **Realización de la rotación a la derecha:**
    ///
    ///    Se realiza una rotación simple a la derecha en el nodo `30`:
    ///
    ///    - El nodo `20` se convierte en la nueva raíz.
    ///    - El nodo `30` se convierte en el hijo derecho de `20`.
    ///    - El subárbol izquierdo de `30` (si existe) se convierte en el subárbol derecho de `20`.
    ///
    ///    La estructura del árbol después de la rotación será:
    ///
    ///    ```
    ///      20
    ///     /  \
    ///    10   30
    ///    ```
    ///
    /// # Caso Complejo: Estructura antes de la rotación con Subárboles
    ///
    /// Si el árbol tiene subárboles en los nodos involucrados, la estructura puede ser:
    ///
    /// ```
    ///      30
    ///     /
    ///    20
    ///   /  \
    /// 10   25
    ///        \
    ///        27
    /// ```
    ///
    /// # Proceso de la Rotación a la Derecha (Caso Complejo)
    ///
    /// 1. **Realización de la rotación a la derecha:**
    ///
    ///    Se realiza una rotación simple a la derecha en el nodo `30`:
    ///
    ///    - El nodo `20` se convierte en la nueva raíz.
    ///    - El nodo `30` se convierte en el hijo derecho de `20`.
    ///    - El subárbol izquierdo de `30` (en este caso, `25`) se convierte en el subárbol derecho de `20`.
    ///    - Los subárboles del nodo `25` se mantienen como están.
    ///
    ///    La estructura del árbol después de la rotación será:
    ///
    ///```
    ///      20
    ///     /  \
    ///    10   30
    ///         / \
    ///        25  27
    /// ```

    pub(crate) fn simple_rotation_right(mut unbalanced_node : Option<Box<AVLNode<T>>>) -> Option<Box<AVLNode<T>>>{
        match unbalanced_node{
            None => None,
            Some(ref mut n) => {
                let mut left_child = n.left.take(); //Nodo izquierdo
                let split_subtree = left_child.as_mut().and_then(|some_node: &mut Box<AVLNode<T>>| -> Option<Box<AVLNode<T>>> {some_node.right.take()});
                //Si el subarbol derecho es some al nodo desbalanceado le asignamos en el puntero izquierdo el subarbol
                n.left = split_subtree;
                //Si left_child es Some, asignamos left_child.right en el nodo desbalanceado
                if let Some(ref mut node_left) = left_child{
                    node_left.right = unbalanced_node;
                    Self::update_height_node(&mut node_left.right); //Actualizamos al nodo desbalanceado.
                }
                Self::update_height_node(&mut left_child);
                left_child //Se retorna el nodo que ocupa el lugar del nodo que recibimos debido a que lo rotamos,
                //Con ello en la reasignación el padre apuntara a este.
            }
        }
    }
    /// Realiza una rotación simple a la izquierda en un nodo desbalanceado de un árbol AVL.
    ///
    /// La rotación a la izquierda se utiliza para balancear el árbol cuando se detecta un desbalance
    /// en el subárbol derecho del nodo desbalanceado.
    ///
    /// # Parámetros
    ///
    /// - `node`: `Option<Box<AVLNode<T>>>` - El nodo desbalanceado en el que se debe realizar la rotación
    ///   a la izquierda. Este nodo es del tipo `Option<Box<AVLNode<T>>>`, donde `AVLNode<T>` es una
    ///   estructura que representa un nodo en el árbol AVL.
    ///
    /// # Retorno
    ///
    /// - `Option<Box<AVLNode<T>>>` - El nuevo nodo raíz después de realizar la rotación a la izquierda.
    ///   Este nodo es del tipo `Option<Box<AVLNode<T>>>`.
    ///
    /// # Ejemplo de uso
    ///
    /// ```rust
    /// let mut tree = AVLTree::new();
    /// tree.insert_node(10);
    /// tree.insert_node(20);
    /// tree.insert_node(30);
    ///
    /// println!("{:?}", tree.root);
    ///
    /// let new_root = AVLTree::left_rotation(tree.root.take());
    ///
    /// println!("{:?}", new_root);
    /// ```
    ///
    /// # Caso Simple: Estructura antes de la rotación
    ///
    /// El árbol AVL antes de la rotación tiene la siguiente estructura:
    ///
    /// ```
    ///     10
    ///       \
    ///        20
    ///          \
    ///           30
    /// ```
    ///
    /// # Proceso de la Rotación a la Izquierda (Caso Simple)
    ///
    /// 1. **Realización de la rotación a la izquierda:**
    ///
    ///    Se realiza una rotación simple a la izquierda en el nodo `10`:
    ///
    ///    - El nodo `20` se convierte en la nueva raíz.
    ///    - El nodo `10` se convierte en el hijo izquierdo de `20`.
    ///    - El subárbol derecho de `10` (si existe) se convierte en el subárbol izquierdo de `20`.
    ///
    ///    La estructura del árbol después de la rotación será:
    ///
    ///    ```
    ///      20
    ///     /  \
    ///    10   30
    ///    ```
    ///
    /// # Caso Complejo: Estructura antes de la rotación con Subárboles
    ///
    /// Si el árbol tiene subárboles en los nodos involucrados, la estructura puede ser:
    ///
    /// ```
    ///      10
    ///        \
    ///         20
    ///        /  \
    ///       15   25
    ///             \
    ///             30
    /// ```
    ///
    /// # Proceso de la Rotación a la Izquierda (Caso Complejo)
    ///
    /// 1. **Realización de la rotación a la izquierda:**
    ///
    ///    Se realiza una rotación simple a la izquierda en el nodo `10`:
    ///
    ///    - El nodo `20` se convierte en la nueva raíz.
    ///    - El nodo `10` se convierte en el hijo izquierdo de `20`.
    ///    - El subárbol derecho de `10` (si existe) se convierte en el subárbol izquierdo de `20`.
    ///    - Los subárboles del nodo `25` se mantienen como están.
    ///
    ///    La estructura del árbol después de la rotación será:
    ///
    ///    ```
    ///      20
    ///     /  \
    ///    10   25
    ///      \    \
    ///      15   30
    ///    ```
    pub(crate) fn simple_rotation_left(mut unbalanced_node : Option<Box<AVLNode<T>>>) -> Option<Box<AVLNode<T>>>{
        match unbalanced_node{
            None => None,
            Some(ref mut n) => {
                let mut right_child = n.right.take();
                let split_tree = right_child.as_mut().and_then(|node_unwp: &mut Box<AVLNode<T>>|-> Option<Box<AVLNode<T>>> {node_unwp.left.take()});
                n.right = split_tree;
                if let Some(ref mut right_node) = right_child{
                    right_node.left = unbalanced_node;
                    Self::update_height_node(&mut right_node.left);
                }
                Self::update_height_node(&mut right_child);
                right_child
            }

        }

    }
    /// Realiza una rotación doble derecha-izquierda (right-left) en un nodo desbalanceado de un árbol AVL.
    ///
    /// La rotación derecha-izquierda se utiliza para balancear el árbol cuando se detecta un desbalance
    /// en el subárbol derecho del nodo desbalanceado.
    ///
    /// # Parámetros
    ///
    /// - `node`: `Option<Box<AVLNode<T>>>` - El nodo desbalanceado en el que se debe realizar la rotación
    ///   derecha-izquierda. Este nodo es del tipo `Option<Box<AVLNode<T>>>`, donde `AVLNode<T>` es una
    ///   estructura que representa un nodo en el árbol AVL.
    ///
    /// # Retorno
    ///
    /// - `Option<Box<AVLNode<T>>>` - El nuevo nodo raíz después de realizar la rotación derecha-izquierda.
    ///   Este nodo es del tipo `Option<Box<AVLNode<T>>>`.
    ///
    /// # Ejemplo de uso
    ///
    /// ```rust
    ///     let mut tree : AVLTree<i32> = AVLTree::new();
    ///     tree.insert_node(100);
    ///     tree.insert_node(120);
    ///     tree.insert_node(110);
    ///     tree.insert_node(130);
    ///     tree.insert_node(115);
    ///     tree.insert_node(109);
    ///     println!("{:?}", tree.root);
    ///     let new_root : Option<Box<AVLNode<i32>>> = AVLTree::right_left_rotation(tree.root.take());
    ///     println!("{:?}", new_root);
    /// ```
    ///
    /// # Estructura antes de la rotación
    ///
    /// El árbol AVL antes de la rotación tiene la siguiente estructura:
    ///
    /// ```
    ///         100
    ///           \
    ///           120
    ///          /  \
    ///        110  130
    ///        / \
    ///      109 115
    /// ```
    ///
    /// # Proceso de la rotación derecha-izquierda
    ///
    /// 1. **Rotación simple a la derecha en el subárbol derecho:**
    ///
    ///    Se realiza una rotación simple a la derecha en el nodo `120`:
    ///
    ///    ```
    ///         100
    ///           \
    ///           110
    ///          /  \
    ///        109  120
    ///             /
    ///           115
    ///             \
    ///             130
    ///    ```
    ///
    /// 2. **Rotación simple a la izquierda en el nodo desbalanceado:**
    ///
    ///    Se realiza una rotación simple a la izquierda en el nodo `100`:
    ///
    ///    ```
    ///         110
    ///        /  \
    ///      100  120
    ///      /    / \
    ///    109  115 130
    ///    ```
    pub(crate) fn right_left_rotation(mut unbalanced_node : Option<Box<AVLNode<T>>>) -> Option<Box<AVLNode<T>>>{
        let right_child: Option<Box<AVLNode<T>>> = unbalanced_node.as_mut().and_then(|n| {n.right.take()});
        let balance_right_node: Option<Box<AVLNode<T>>> = Self::simple_rotation_right(right_child);
        //Se reasigna la mutacion del balance del hijo derecha al nodo desbalanceado en right
        match unbalanced_node {
            None => {},
            Some(ref mut node) => {
                node.right = balance_right_node;
            }
        }
        let new_root: Option<Box<AVLNode<T>>> = Self::simple_rotation_left(unbalanced_node);
        new_root

    }
    /// Realiza una rotación izquierda-derecha en un nodo desbalanceado de un árbol AVL.
    ///
    /// La rotación izquierda-derecha se utiliza para balancear el árbol cuando se detecta un desbalance
    /// en el subárbol izquierdo del nodo desbalanceado, y dentro de este subárbol izquierdo, hay un
    /// desbalance en el subárbol derecho.
    ///
    /// # Parámetros
    ///
    /// - `unbalanced_node`: `Option<Box<AVLNode<T>>>` - El nodo desbalanceado en el que se debe realizar la
    ///   rotación izquierda-derecha. Este nodo es del tipo `Option<Box<AVLNode<T>>>`, donde `AVLNode<T>`
    ///   es una estructura que representa un nodo en el árbol AVL.
    ///
    /// # Retorno
    ///
    /// - `Option<Box<AVLNode<T>>>` - El nuevo nodo raíz después de realizar la rotación izquierda-derecha.
    ///   Este nodo es del tipo `Option<Box<AVLNode<T>>>`.
    ///
    /// # Ejemplo de uso
    ///
    /// ```rust
    /// let mut tree = AVLTree::new();
    /// tree.insert_node(30);
    /// tree.insert_node(10);
    /// tree.insert_node(40);
    /// tree.insert_node(9);
    /// tree.insert_node(20);
    ///
    /// println!("Antes de la rotación: {:?}", tree.root);
    ///
    /// let new_root = AVLTree::left_right_rotation(tree.root.take());
    ///
    /// println!("Después de la rotación: {:?}", new_root);
    /// ```
    ///
    /// # Caso Simple: Estructura antes de la rotación
    ///
    /// El árbol AVL antes de la rotación tiene la siguiente estructura:
    ///
    /// ```
    ///     30
    ///    /
    ///   10
    ///    \
    ///     20
    /// ```
    ///
    /// # Proceso de la Rotación Izquierda-Derecha (Caso Simple)
    ///
    /// 1. **Rotación Simple a la Izquierda en el Hijo Izquierdo**:
    ///    - Se realiza una rotación simple a la izquierda en el nodo `10`, cuyo hijo derecho es `20`.
    ///    - Esto convierte a `20` en la nueva raíz del subárbol izquierdo, con `10` como su hijo izquierdo.
    ///
    ///    La estructura después de la rotación a la izquierda en `10` es:
    ///
    /// ```
    ///       20
    ///      /
    ///     10
    ///    /
    ///   30
    /// ```
    ///
    /// 2. **Rotación Simple a la Derecha en el Nodo Desbalanceado**:
    ///    - Luego, se realiza una rotación simple a la derecha en el nodo `30`
    ///    - El resultado es el nuevo árbol equilibrado con `20` como la nueva raíz.
    ///
    ///    La estructura final del árbol después de la rotación será:
    ///
    ///```
    ///      20
    ///     /  \
    ///    10   30
    ///```
    ///
    /// # Caso Complejo: Estructura antes de la rotación con Subárboles
    ///
    /// Si el árbol tiene subárboles en los nodos involucrados, la estructura puede ser:
    ///
    ///```
    ///     30
    ///    /  \
    ///   10   40
    ///  /  \
    /// 9   20
    ///```
    ///
    /// # Proceso de la Rotación Izquierda-Derecha (Caso Complejo)
    ///
    /// 1. **Rotación Simple a la Izquierda en el Hijo Izquierdo**:
    ///    - Se realiza una rotación simple a la izquierda en el nodo `10`, cuyo hijo derecho es `20`.
    ///    - Esto convierte a `20` en la nueva raíz del subárbol izquierdo, con `10` como su hijo izquierdo.
    ///
    ///    La estructura después de la rotación a la izquierda en `10` es:
    ///
    ///```
    ///       20
    ///      /  \
    ///     10   30
    ///    /     \
    ///   9      40
    ///```
    ///
    /// 2. **Rotación Simple a la Derecha en el Nodo Desbalanceado**:
    ///    - Luego, se realiza una rotación simple a la derecha en el nodo `30`
    ///    - El resultado es el nuevo árbol equilibrado con `20` como la nueva raíz.
    ///
    ///    La estructura final del árbol después de la rotación será:
    ///
    ///```
    ///      20
    ///     /  \
    ///    10   30
    ///   /     \
    ///  9      40
    ///```
    pub(crate) fn left_right_rotation(mut unbalanced_node : Option<Box<AVLNode<T>>>) -> Option<Box<AVLNode<T>>>{
        let left_child = unbalanced_node.as_mut().and_then(|n| {n.left.take()});
        let balance_left_node = Self::simple_rotation_left(left_child);
        //Se reasigna la mutacion del balance del hijo derecha al nodo desbalanceado en right
        match unbalanced_node {
            None => {},
            Some(ref mut node) => {
                node.left = balance_left_node;
            }
        }
        let new_root = Self::simple_rotation_right(unbalanced_node);
        new_root

    }
    pub(crate) fn rebalance(mut node : Option<Box<AVLNode<T>>>) -> Option<Box<AVLNode<T>>>{
        let balance_factor : isize = Self::balance_factor(&mut node);
        if balance_factor == 2{
            let left_child_balance_factor: Option<isize> = node.as_mut().and_then(|n| {Some(Self::balance_factor(&mut n.left))});
            if let Some(balance_child) = left_child_balance_factor{
                if balance_child >= 0{ //simple rotation right
                    node = Self::simple_rotation_right(node.take());
                }else if balance_child == -1 { //left_right_rotation
                    node = Self::left_right_rotation(node.take());
                }

            }
        }else if balance_factor == -2 {
            let right_child_balance_factor : Option<isize> = node.as_mut().and_then(|n| {Some(Self::balance_factor(&mut n.right))});
            if let Some(balance_child) = right_child_balance_factor{
                if balance_child >= 0{ //right_left_rotation
                    node = Self::right_left_rotation(node.take());

                }else if balance_child == -1 {//simple rotation left
                    node = Self::simple_rotation_left(node.take());
                }
            }
        }
        node
    }
    ///Un metodo de la implementación que permite hacer una inserción recursiva, dicho metodo es el encargado de implementar
    ///toda la logica dentro del arbol para el balance de mismo, esta función garantiza una capa de abstracción que mantiene 
    /// el arbol totalmente balanceado.
    pub fn insert_node(&mut self , value : T){
        self.root = Self::insert_recursibly(self.root.take(), value);
        self.size += 1;
    }
    pub(crate) fn insert_recursibly(mut node : Option<Box<AVLNode<T>>> , value : T) -> Option<Box<AVLNode<T>>>{
        match node {
            None => {
            //Caso base donde insertamos el nodo
            Some(Box::new(AVLNode::new(value)))
            },
            Some(ref mut n) => {
                match n.value.cmp(&value) {
                    Ordering::Equal => {todo!("")},
                    Ordering::Greater => { // Caso recursivo
                        n.left = Self::insert_recursibly(n.left.take(), value);
                        Self::update_height_node(&mut n.left);
                        Self::update_height_node(&mut node);
                    },
                    Ordering::Less => { //Caso Recursivo
                        n.right = Self::insert_recursibly(n.right.take(), value);
                        Self::update_height_node(&mut n.right); //Actualización de la altura del nodo
                        Self::update_height_node(&mut node); //Actualización de la altura del nodo
                    }
                }
                Self::rebalance(node)
            }            
        }
    }
    pub fn remove_node(&mut self , value : T){
        self.root = Self::remove_recursibly(self.root.take(), value);
        self.size -= 1;
    }
    fn remove_recursibly(mut node : Option<Box<AVLNode<T>>> , value : T) -> Option<Box<AVLNode<T>>>{
        match node{
            None => {todo!("")},
            Some(ref mut n) =>{
                match n.value.cmp(&value){
                    Ordering::Equal => {
                        match (&mut n.left, &mut n.right){
                            (None , None) => {
                                node = None;
                            },
                            (Some(_) , None) => {node = n.left.take();},
                            (None , Some(_)) =>{node = n.right.take();},
                            (Some(_) , Some(_)) => {
                                let max_value_node = Self::find_max(&mut n.left);
                                if let Some(max_left_subtree) = max_value_node{
                                    n.left = Self::remove_recursibly(n.left.take(), max_left_subtree);
                                    n.value = max_left_subtree;
                                }
                            }
                        }
                    },
                    Ordering::Greater => {
                        n.left = Self::remove_recursibly(n.left.take(), value);
                        Self::update_height_node(&mut node);
                    },
                    Ordering::Less => {
                        n.right = Self::remove_recursibly(n.right.take(), value);
                        Self::update_height_node(&mut node);
                    }  
                }
                Self::rebalance(node)
            }
        }
    }
    ///Metodo que retorna opcionalmente un T tipo, que encuentra cualquier nodo en el arbol avl.
    pub fn search(&self , value : T) -> Option<T>{
        let mut current_node = &self.root;
        while let Some(current) = current_node{
            if value == current.value{
                break;
            }else if value > current.value {
                current_node = &current.right;
            }else {
                current_node = &current.left;
            }
        }
        current_node.as_ref().and_then(|n| {Some(n.value)})
    }
    ///Esta función recibe un nodo como raiz y itera hacia la izquierda del nodo hasta que encuentre un caso en que sea None
    ///Esto nos permite encontrar el minimo valor de un subarbol en el AVL.
    pub fn find_min(root_tree : &mut Option<Box<AVLNode<T>>>) -> Option<T> {
        if root_tree.is_none(){
            return None;
        }
        let mut current = root_tree;
        while let Some(current_node) = current{
            if current_node.left.is_none(){
                return Some(current_node.value);
            }
            current = &mut current_node.left;
        }
        None
    }
    ///Esta función recibe un nodo como raiz y itera hacia la derecha del nodo hasta que encuentre un caso en que sea None
    ///Esto nos permite encontrar el minimo valor de un subarbol en el AVL.
    pub fn find_max(root_tree : &mut Option<Box<AVLNode<T>>>) -> Option<T> {
        if root_tree.is_none(){
            return None;
        }
        let mut current = root_tree;
        while let Some(current_node) = current{
            if current_node.right.is_none(){
                return Some(current_node.value);
            }
            current = &mut current_node.right;
        }
        None
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
    pub fn inorder_tree(&self){
        Self::inorder_recursive(&self.root);
    }

    pub(crate) fn inorder_recursive(node : &Option<Box<AVLNode<T>>>){
        if let Some(node_unw) = node{
            Self::inorder_recursive(&node_unw.left);
            println!("{}" , node_unw.value);
            Self::inorder_recursive(&node_unw.right);
        }
    }
    /// ### Recorrido PostOrder
    /// En el recorrido inorder se recorre primero recursivamente el subarbol derecho de la raiz, luego el nodo raiz
    /// y por ultimo recursivamente el subarbol izquierdo del nodo raiz
    /// ```text
    ///                     3°------> root
    ///                         +-------------------+
    ///                     |---|LEFT | 100  | RIGHT|---|
    ///                     |   +-------------------+  |
    ///                    /\                         /\
    ///           1°--->  / \             2°----->   / \ 
    ///                  /__\                       /__\
    /// ```
    pub fn postorder_tree(&self){
        Self::postorder_recursive(&self.root);

    }
    pub(crate) fn postorder_recursive(node : &Option<Box<AVLNode<T>>>){
        if let Some(n) = node{
            Self::postorder_recursive(&n.left);
            Self::postorder_recursive(&n.right);
            println!("{}" , n.value);
        }
    }
    /// ### Recorrido Preorder
    /// En el recorrido preorder se recorre primero el nodo raíz, luego recursivamente el subárbol izquierdo de la raíz,
    /// y por último recursivamente el subárbol derecho de la raíz.
    /// ```text
    ///                     1°------> root
    ///                         +-------------------+
    ///                     |---|LEFT | 100  | RIGHT|---|
    ///                     |   +-------------------+  |
    ///                    /\                         /\
    ///           2°--->  / \             3°----->   / \ 
    ///                  /__\                       /__\
    /// ```
    pub fn preorder_tree(&self){
        Self::preorder_recursive(&self.root);
    }
    pub(crate) fn preorder_recursive(node : &Option<Box<AVLNode<T>>>){
        if let Some(n) = node{
            println!("{}" , n.value);
            Self::preorder_recursive(&n.left);
            Self::preorder_recursive(&n.right);
        }

    }

    ///Llamado recursivo que verifica la propiedad del factor de balance [-1;1]
    pub(crate) fn is_avl(&mut self){
        Self::verify_property_avl(&mut self.root);
    }
    //Llamado recursivo que activa la panic! macro cuando un nodo no cumple con AVL
    pub(crate) fn verify_property_avl(node: &mut Option<Box<AVLNode<T>>>){
        let balance_factor_current_node = Self::balance_factor(node);
        if let Some(n) = node{
            Self::verify_property_avl(&mut n.left);
            Self::verify_property_avl(&mut n.right);
        }
        if balance_factor_current_node >= 2{
            panic!("No cumple la propiedad AVL");
        }
    }

}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_insertion(){
        let mut tree: AVLTree<i32> = AVLTree::new();
        let nodes: Vec<i32> = vec![10 , 20 , 30 , 40 , 50 , 60 , 70 , 80 , 90 , 1_00];
        for i in nodes.iter(){
            tree.insert_node(*i);
        }
        tree.inorder_tree();
        println!("{:?}" , &tree.root);
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
        let node_rotate: Option<Box<AVLNode<i32>>> = AVLTree::simple_rotation_right(tree.root.take());
        println!("{:?}" , &node_rotate);
    }
    #[test]
    fn test_simple_rotation_left(){
        let mut tree: AVLTree<i32> = AVLTree::new();
        tree.insert_node(100);
        tree.insert_node(150); //left_child no posee sub arbol derecho
        tree.insert_node(160);
        AVLTree::simple_rotation_left(tree.root.take());
        tree.insert_node(100);
        tree.insert_node(150); //left_child posee sub arbol derecho
        tree.insert_node(160);
        tree.insert_node(155);
        AVLTree::simple_rotation_left(tree.root.take());
        tree.insert_node(100);
        tree.insert_node(150);
        tree.insert_node(160); //están todos los arboles requeridos
        tree.insert_node(159);
        tree.insert_node(170);
        let node_rotate: Option<Box<AVLNode<i32>>> = AVLTree::simple_rotation_left(tree.root.take());
        println!("{:?}" , &node_rotate);

    }
    #[test]
    fn test_right_left_rotation(){
        let mut tree : AVLTree<i32> = AVLTree::new();
        tree.insert_node(100);
        tree.insert_node(120);
        tree.insert_node(110);
        //println!("{:?}" , tree.root);
        let mut new_node = AVLTree::right_left_rotation(tree.root.take());
        //println!("{:?}" , new_node);
        tree.insert_node(100);
        tree.insert_node(120);
        tree.insert_node(110);
        tree.insert_node(130);
        //println!("{:?}" , tree.root);
        new_node = AVLTree::right_left_rotation(tree.root.take());
        //println!("{:?}" , new_node);
        tree.insert_node(100);
        tree.insert_node(120);
        tree.insert_node(110);
        tree.insert_node(130);
        tree.insert_node(115);
        tree.insert_node(109);
        println!("{:?}" , tree.root);
        new_node = AVLTree::right_left_rotation(tree.root.take());
        println!("{:?}" , new_node);
    }
    #[test]
    fn test_left_right_rotation() {
        // Crear un árbol AVL desbalanceado que requiere una rotación izquierda-derecha
        let mut tree: AVLTree<i32> = AVLTree::new();
        tree.insert_node(30);
        tree.insert_node(10);
        tree.insert_node(20); // Inserción que causa el desbalance
        tree.insert_node(9);
        tree.insert_node(40);
        //          30
        //         / \ 
        //       10  40
        //      /  \
        //     9   20
        // Imprimir la estructura del árbol antes de la rotación
        println!("Antes de la rotación: {:?}", tree.root);

        // Aplicar la rotación izquierda-derecha
        let new_root: Option<Box<AVLNode<i32>>> = AVLTree::left_right_rotation(tree.root.take());

        // Imprimir la estructura del árbol después de la rotación
        println!("Después de la rotación: {:?}", new_root);

        // Verificar la estructura del árbol después de la rotación
        // Esperamos que la estructura del árbol sea:
        //         20
        //        / \
        //      10   30
        //      /     \
        //     9      40
 
    }
    #[test]
    fn test_remove_on_tree(){
        let mut tree : AVLTree<i32> = AVLTree::new();
        for i in vec![10 , 20 , 30 , 40 , 50 , 60 , 70 , 80 , 90 , 1_00].iter(){
            tree.insert_node(*i);
        }
        tree.remove_node(100);
        tree.remove_node(90);
        tree.remove_node(60);
        println!("{:?}" , tree);
        tree.is_avl();
    }
    #[test]
    fn find_min_on_tree(){
        let mut tree : AVLTree<i32> = AVLTree::new();
        let mut nodes: Vec<i32> = vec![10 , 20 , 30 , 40 , 50 , 60 , 70 , 80 , 1_00];
        //insertamos nodos
        nodes.iter().for_each(|n| {
            tree.insert_node(*n);
            tree.is_avl();
        });
        nodes.reverse();
        //eliminamos nodos
        nodes.iter().for_each(|n| {
            tree.remove_node(*n);
            tree.is_avl();
        });
    }
}