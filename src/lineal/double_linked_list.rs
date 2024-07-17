use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::{Ref, RefCell, RefMut};
///### Nodo doble
/// Está es la estructura básica del nodo de una lista enlazada, este posee tolerancia opcional de tipo recursiva
///para poder tolerar el mismo tipo dentro de el.
/// #Ejemplo Visual de la estructura
/// ```text
///       ----------------------
///      |      |       |       |
///      | PREV | VALUE |  NEXT |
///      |      |       |       |
///       ----------------------
/// ```
/// ### Uso de Option<Rc<RefCell<DoublyNode<T>>>>
/// La finalidad de utilizar este tipo complejo, es con el proposito de permitir la mutabilidad interior compartida de propietarios 
/// osea que multiples propietarios puedan mutar un valor, de todas maneras posee una finalidad que es guiar el contador fuerte, mediante
/// `strong_count` con respecto a los siguientes del nodo actual de esta manera no se crea una referencia ciclica debido a que si se agrega
/// un nodo se hace +1 al strong_count del DoublyNode<T> y no hay referencia infinita, evitamos el `stack overflow`.
/// `Veamos un ejemplo de ello`
/// ```text
///     -+ReferenceCounter Rc<T>+-                     -+ReferenceCounter Rc<T>+-
///     | +--------------------+  |                     +---------------------+
///     ||      |       |       | |                    |      |       |       |
///     || PREV | VALUE |  NEXT | --strong_count+1-->  | PREV | VALUE |  NEXT | -strong_count+1-->
///     ||      |       |       | |                    |      |       |       |
///     | +--------------------+  |                    +---------------------+
///     -+++++++++++++++++++++++++-
/// ```
/// Con esto logramos que el contador de referencia aumente en uno y se disminuya cuando exista.
/// ### Uso de Option<Weak<RefCell<DoublyNode<T>>>>
/// Con Weak<T> lo que logramos es evitar el ciclo de referencias ciclo que se produciria en caso de utilizar
/// Rc<T>, Y si queremos modificar desde Weak<T>, entonces si queremos hacerlo utilizamos `.upgrade()` y si es que existe una
/// referencia retorna Rc<T> y desde Rc<T> podemos modificar
/// ```text
///                   -+++WeakCounter Weak<T>+++-                     -+WeakCounter Weak<T>+-
///                   | +--------------------+  |                     +---------------------+
///                   ||      |       |       | |                    |      |       |       |
/// ..<-weak_count+1--|| PREV | VALUE |  NEXT |  <---weak_count+1--  | PREV | VALUE |  NEXT | 
///                   ||      |       |       | |                    |      |       |       |
///                   | +--------------------+  |                    +---------------------+
///                   -+++++++++++++++++++++++++-
/// ```
/// Al final el enlace entre cada nodo queda de la siguiente manera
/// ```text
///                       |                   |
///                       |<-weak_count +1 -- |
///                       |--strong_count+1-> |
///                       |                   |
/// ```
/// 

#[derive(Debug)]
struct DoublyNode<T> {
    prev: Option<Weak<RefCell<DoublyNode<T>>>>, //Me permite mutar a traves de Weak mediante el metodo upgrade que devuelve un Rc<T> con RefCell<NodeDoubly>
    value: T,
    next: Option<Rc<RefCell<DoublyNode<T>>>> //Referencia en heap compartida, aumenta strong_count a medida que aumenta la lista enlazada hacia el siguiente nodo y disminuye al eliminar nodos
}

impl DoublyNode<i32> {
    fn new(value : i32) -> Self{
        Self{
            prev : None,
            value,
            next : None
        }
    }
    
}

///### Nodo doble
/// Está es la estructura básica del nodo de una lista enlazada, este posee tolerancia opcional de tipo recursiva
///para poder tolerar el mismo tipo dentro de el.
/// #Ejemplo Visual de la estructura
/// ```text
///                     head ↓                                                           tail ↓
///              ----------------------          ----------------------          ----------------------
///             |      |       |       | <----- |      |       |       | <----- |      |       |       |
/// None <----  | PREV | VALUE |  NEXT | -----> | PREV | VALUE |  NEXT | -----> | PREV | VALUE |  NEXT | ---->  None
///             |      |       |       |        |      |       |       |        |      |       |       |
///              ----------------------          ----------------------          ----------------------   
/// ```
#[derive(Debug)]
struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<DoublyNode<T>>>>,
    tail: Option<Rc<RefCell<DoublyNode<T>>>>,
    size: i32
}
impl DoublyLinkedList<i32>{
    fn new() -> Self{
        Self{
            head : None , 
            tail : None, 
            size : 0
        }
    }
    fn empty(&self) -> bool{
        self.head.is_none() && self.tail.is_none()
    }
    fn len(&self) -> i32{
       self.size
    }
    ///## Push Back Doblemente Enlazado
    /// Bueno hacer Push Back en una lista Doblemente enlazada es mucho más sencillo debido a que no tenemos que recorrer constantemente el array hasta el ultimo
    /// nodo porque tenemos un puntero haciendo referencia al ultimo, por ello solamente actualizamos el puntero .next al siguiente nodo, dicha inserción tiene
    /// un coste O(1) por que son solamente reajustes de punteros.
    /// ### Casos a tener en cuenta si deseas implementarla
    /// - `Caso 1`: El nodo a agregar es el primero de la lista
    ///```rust 
    ///   fn main(){
    ///     let mut list : DoubleLinkedList<i32> = DoubleLinkedList::new();
    ///     list.push_back(20);
    ///   }
    ///```
    ///```text
    ///
    ///                                 head ↓ 
    ///                                 tail ↓
    ///                          ----------------------          
    ///                         |      |       |       |  
    ///             None <----  | PREV |  20   |  NEXT | ----->  None
    ///                         |      |       |       |        
    ///                          ----------------------            
    /// ```
    /// - `Caso 2`: El nodo a agregar es el ultimo en la lista
    ///```rust 
    ///   fn main(){
    ///     let mut list : DoubleLinkedList<i32> = DoubleLinkedList::new();
    ///     list.push_back(20);
    ///     list.push_back(40)
    ///   }
    ///``` 
    /// ```text
    ///                                 head ↓ 
    ///                                 tail ↓ -------------------↓
    ///                          ----------------------          
    ///                         |      |       |       |  
    ///             None <----  | PREV |  20   |  NEXT | ----->  None
    ///                         |      |       |       |        
    ///                          ----------------------  
    ///     
    ///                     head ↓                           tail ↓
    ///              ----------------------          ----------------------          
    ///             |      |       |       | <----- |      |       |       |  
    /// None <----  | PREV |  20   |  NEXT | -----> | PREV |   40  |  NEXT | -----> None
    ///             |      |       |       |        |      |       |       |        
    ///              ----------------------          ----------------------            
    ///                      head ↓                                                         tail ↓
    ///              ----------------------          ----------------------          ----------------------         
    ///             |      |       |       | <----- |      |       |       | <----  |      |       |       | 
    /// None <----  | PREV |  20   |  NEXT | -----> | PREV |   40  |  NEXT | -----> | PREV |   50  |  NEXT | -----> None
    ///             |      |       |       |        |      |       |       |        |      |       |       |
    ///              ----------------------          ----------------------          ----------------------  
    /// ```
    /// - _El push back se hace desde tail_
    fn push_back(&mut self , value : i32){
        if  self.empty(){
            let new_node : Rc<RefCell<DoublyNode<i32>>> = Rc::new(RefCell::new(DoublyNode::new(value)));
            self.head = Some(new_node); //head : |20|
            self.tail = Some(self.head.as_ref().unwrap().clone()); //tail : |20|;
            self.size +=1;
        }
        else{
            let new_node : Rc<RefCell<DoublyNode<i32>>> = Rc::new(RefCell::new(DoublyNode::new(value)));
            if let Some(ref old_tail) = self.tail{
                let mut old_tail_properties = old_tail.borrow_mut();
                let mut new_node_properties = new_node.borrow_mut();
                old_tail_properties.next = Some(new_node.clone()); //next de la old_tail = nuevo nodo
                //Propiedades del nodo inicial
                new_node_properties.prev = Some(Rc::downgrade(&old_tail)); //Creamos una referencia debil al nodo anterior. None<-weak-|value|
            }
            self.tail = Some(new_node);
            self.size += 1;
        }
    }
    /// ## Push Front Doblemente Enlazado
    /// Hacer `push_front` en una lista doblemente enlazada es sencillo debido a que no tenemos que recorrer la lista completa. 
    /// En lugar de eso, simplemente actualizamos el puntero `.prev` del nodo actual `head` al nuevo nodo, y luego ajustamos el nuevo nodo para que apunte a `head`.
    /// Esta operación tiene un coste O(1) porque solo implica reajustes de punteros.
    /// ### Casos a tener en cuenta si deseas implementarla
    /// - `Caso 1`: El nodo a agregar es el primero de la lista
    /// ```rust 
    ///   fn main() {
    ///     let mut list: DoubleLinkedList<i32> = DoubleLinkedList::new();
    ///     list.push_front(20);
    ///   }
    /// ```
    /// ```text
    ///                                 head ↓ 
    ///                                 tail ↓
    ///                          ----------------------          
    ///                         |      |       |       |  
    ///             None <----  | PREV |  20   |  NEXT | ----->  None
    ///                         |      |       |       |        
    ///                          ----------------------            
    /// ```
    /// - `Caso 2`: El nodo a agregar es el primero en la lista cuando ya existen otros nodos
    /// ```rust 
    ///   fn main() {
    ///     let mut list: DoubleLinkedList<i32> = DoubleLinkedList::new();
    ///     list.push_front(20);
    ///     list.push_front(10);
    ///   }
    /// ``` 
    /// ```text
    ///                                 head ↓ 
    ///                                 tail ↓ -------------------↓
    ///                          ----------------------          
    ///                         |      |       |       |  
    ///             None <----  | PREV |  20   |  NEXT | ----->  None
    ///                         |      |       |       |        
    ///                          ----------------------  
    ///     
    ///                     head ↓                           tail ↓
    ///              ----------------------          ----------------------          
    ///             |      |       |       | <----- |      |       |       |  
    /// None <----  | PREV |  10   |  NEXT | -----> | PREV |   20  |  NEXT | -----> None
    ///             |      |       |       |        |      |       |       |        
    ///              ----------------------          ----------------------            
    ///                      head ↓                                                         tail ↓
    ///              ----------------------          ----------------------          ----------------------         
    ///             |      |       |       | <----- |      |       |       | <----  |      |       |       | 
    /// None <----  | PREV |  10   |  NEXT | -----> | PREV |   20  |  NEXT | -----> | PREV |   30  |  NEXT | -----> None
    ///             |      |       |       |        |      |       |       |        |      |       |       |
    ///              ----------------------          ----------------------          ----------------------  
    /// ```
    /// - _El `push_front` se hace desde `head`_

    fn push_front(){
        todo!("")
    }
    fn pop(&mut self) -> Result<i32 , String>{
        todo!("")
    }
    fn shirt(&mut self) -> Result<i32 , String>{
        todo!("")
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn creation_doubly_test(){
        let doubly : DoublyLinkedList<i32> = DoublyLinkedList::new();
    }
    #[test]
    fn empty_test(){
        let doubly : DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert_eq!(doubly.empty(), true);
    }
    #[test]
    fn push_back_test(){
        let mut doubly : DoublyLinkedList<i32> = DoublyLinkedList::new();
        doubly.push_back(30);
        doubly.push_back(40);
        doubly.push_back(50);
        println!("{:?}" , doubly);

    }
}
