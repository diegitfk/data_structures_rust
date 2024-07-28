use std::fmt::Display;
use std::rc::{Rc, Weak};
use std::cell::{Ref, RefCell, RefMut};
use num::Integer;
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
pub struct DoublyNode<T> {
    prev: Option<Weak<RefCell<DoublyNode<T>>>>, //Me permite mutar a traves de Weak mediante el metodo upgrade que devuelve un Rc<T> con RefCell<NodeDoubly>
    value: T,
    next: Option<Rc<RefCell<DoublyNode<T>>>> //Referencia en heap compartida, aumenta strong_count a medida que aumenta la lista enlazada hacia el siguiente nodo y disminuye al eliminar nodos
}

impl<T> DoublyNode<T>
where T : Integer + Clone + Copy + Display {
    fn new(value : T) -> Self{
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
pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<DoublyNode<T>>>>,
    tail: Option<Rc<RefCell<DoublyNode<T>>>>,
    size: i32
}
impl<T> DoublyLinkedList<T>
where T : Integer + Clone + Copy + Display{
    pub fn new() -> Self{
        Self{
            head : None , 
            tail : None, 
            size : 0
        }
    }
    pub fn empty(&self) -> bool{
        self.head.is_none() && self.tail.is_none()
    }
    pub fn len(&self) -> i32{
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
    pub fn push_back(&mut self , value : T){
        if  self.empty(){
            let new_node : Rc<RefCell<DoublyNode<T>>> = Rc::new(RefCell::new(DoublyNode::new(value)));
            self.head = Some(new_node); //head : |20|
            self.tail = Some(self.head.as_ref().unwrap().clone()); //tail : |20|;
            self.size +=1;
        }
        else{
            let new_node : Rc<RefCell<DoublyNode<T>>> = Rc::new(RefCell::new(DoublyNode::new(value)));
            if let Some(ref old_tail) = self.tail{
                let mut old_tail_properties: RefMut<DoublyNode<T>> = old_tail.borrow_mut();
                let mut new_node_properties: RefMut<DoublyNode<T>> = new_node.borrow_mut();
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

    pub fn push_front(&mut self , value : T){
        if  self.empty(){
            let new_node : Rc<RefCell<DoublyNode<T>>> = Rc::new(RefCell::new(DoublyNode::new(value)));
            self.head = Some(new_node); //head : |20|
            self.tail = Some(self.head.as_ref().unwrap().clone()); //tail : |20|;
            self.size +=1;
        }else {
            let new_node : Rc<RefCell<DoublyNode<T>>> = Rc::new(RefCell::new(DoublyNode::new(value)));
            if let Some(ref old_head) = self.head{
                let mut properties_new_node: RefMut<DoublyNode<T>> = new_node.borrow_mut(); //prestamo mutable 
                let mut properties_old_head: RefMut<DoublyNode<T>> = old_head.borrow_mut(); //prestamo mutable
                properties_new_node.next = Some(old_head.clone()); //siguiente del nuevo es 
                properties_old_head.prev = Some(Rc::downgrade(&new_node)); 
            }
            self.head = Some(new_node);
            self.size += 1;
        }
    }
    ///## Pop_Back()
    /// ```text
    /// pop_back()
    ///                       head ↓                                                         tail ↓
    ///              ----------------------          ----------------------          ----------------------         
    ///             |      |       |       | <----- |      |       |       | <----  |      |       |       | 
    /// None <----  | PREV |  10   |  NEXT | -----> | PREV |   20  |  NEXT | -----> | PREV |   30  |  NEXT | -----> None
    ///             |      |       |       |        |      |       |       |        |      |       |       |
    ///              ----------------------          ----------------------          ----------------------  
    ///                     head ↓                         tail ↓
    ///              ----------------------          ----------------------                  
    ///             |      |       |       | <----- |      |       |       | <----  
    /// None <----  | PREV |  10   |  NEXT | -----> | PREV |   20  |  NEXT | ----->  None
    ///             |      |       |       |        |      |       |       |        
    ///              ----------------------          ---------------------- 
    ///  pop_back() 
    ///                       tail ↓
    ///                       head ↓                  
    ///              ----------------------                        
    ///             |      |       |       |   
    /// None <----  | PREV |  10   |  NEXT | ----->  None
    ///             |      |       |       |              
    ///              ----------------------                     
    /// ```     
    /// 
    /// ### Casos de eliminación.
    /// - `Caso 1`: _tail y head se encuentran apuntando al mismo nodo_ 
    /// ```text
    ///                 pop_back() 
    ///                       tail ↓
    ///                       head ↓                  
    ///              ----------------------                        
    ///             |      |       |       |   
    /// None <----  | PREV |  10   |  NEXT | ----->  None
    ///             |      |       |       |              
    ///              ----------------------  
    /// 
    /// tail => None
    /// head => None             
    /// ```
    /// _`Solucion: Debemos tomar el valor y asignar self.head y self.tail en None`_
    /// - `Caso 2`: _tail y head no apuntan al mismo nodo en la lista_
    /// ```text
    ///                     pop_back()
    ///                       head ↓                                                         tail ↓
    ///              ----------------------          ----------------------          ----------------------         
    ///             |      |       |       | <----- |      |       |       | <----  |      |       |       | 
    /// None <----  | PREV |  10   |  NEXT | -----> | PREV |   20  |  NEXT | -----> | PREV |   30  |  NEXT | -----> None
    ///             |      |       |       |        |      |       |       |        |      |       |       |
    ///              ----------------------          ----------------------          ----------------------  
    /// ```
    /// _`Solucion:`_
    /// - Reasignar el puntero next del nodo previo a None
    /// - Reasignar el puntero tail de la `doublylinkedlist` en el puntero previo al valor actual de tail.
    /// - Disminuir el contador de nodos de la `doublylinkedlist`
    /// ```text
    ///                        head ↓                         tail ↓
    ///              ----------------------          ----------------------                  
    ///             |      |       |       | <----- |      |       |       |   
    /// None <----  | PREV |  10   |  NEXT | -----> | PREV |   20  |  NEXT | ----->  None
    ///             |      |       |       |        |      |       |       |        
    ///              ----------------------          ----------------------
    /// 
    /// ```
    pub fn pop_back(&mut self) -> Result<T, String>{
        if self.empty(){
            Err(String::from("La lista se encuentra vacia"))
        }else {
            let tail_prev: Option<Rc<RefCell<DoublyNode<T>>>> = {
                let tail_borrow: Ref<DoublyNode<T>> = self.tail.as_ref().unwrap().borrow();
                let tail_prev_option: Option<Rc<RefCell<DoublyNode<T>>>> = {
                    if let Some(prev_node) = &tail_borrow.prev{
                        prev_node.upgrade()
                    }else {
                        None
                    }
                };
                tail_prev_option
            };
            //si prev no es None entonces tail diverge en dirección de nodos con respecto a head.
            if let Some(prev_node_to_tail) = tail_prev{
                let mut properties_prev_node_to_tail: RefMut<DoublyNode<T>> = prev_node_to_tail.borrow_mut();
                let taking_current_tail: Rc<RefCell<DoublyNode<T>>> = self.tail.take().unwrap();
                let del_val: T = taking_current_tail.borrow().value;
                properties_prev_node_to_tail.next = None;
                self.tail = Some(prev_node_to_tail.clone());
                self.size -= 1;
                Ok(del_val)

            }else {
                //si prev es None se da el caso en el que tail y head se encuentran los dos posicionados en el mismo nodo.
                let taking_current_tail: Rc<RefCell<DoublyNode<T>>> = self.tail.take().unwrap();
                let del_val: T = taking_current_tail.borrow().value;
                self.head = None;
                self.size -= 1;
                Ok(del_val)
            }
        }
    }
    ///## Pop()
    /// ```text
    /// pop()
    ///                       head ↓                                                         tail ↓
    ///              ----------------------          ----------------------          ----------------------         
    ///             |      |       |       | <----- |      |       |       | <----  |      |       |       | 
    /// None <----  | PREV |  10   |  NEXT | -----> | PREV |   20  |  NEXT | -----> | PREV |   30  |  NEXT | -----> None
    ///             |      |       |       |        |      |       |       |        |      |       |       |
    ///              ----------------------          ----------------------          ----------------------  
    ///                     head ↓                         tail ↓
    ///              ----------------------          ----------------------                  
    ///             |      |       |       | <----- |      |       |       | <----  
    /// None <----  | PREV |  20   |  NEXT | -----> | PREV |   30  |  NEXT | ----->  None
    ///             |      |       |       |        |      |       |       |        
    ///              ----------------------          ---------------------- 
    ///  pop() 
    ///                       tail ↓
    ///                       head ↓                  
    ///              ----------------------                        
    ///             |      |       |       |   
    /// None <----  | PREV |  30   |  NEXT | ----->  None
    ///             |      |       |       |              
    ///              ----------------------                     
    /// ``` 
    /// ### Casos de eliminación
    /// - _`Caso1`_: _head y tail se encuentran apuntando al mismo nodo._
    /// ```text
    /// pop_front() 
    ///                       tail ↓
    ///                       head ↓                  
    ///              ----------------------                        
    ///             |      |       |       |   
    /// None <----  | PREV |  30   |  NEXT | ----->  None
    ///             |      |       |       |              
    ///              ----------------------   
    /// head => None
    /// tail => None           
    /// ```
    /// `Solución`: Debemos tomar el valor a eliminar y asignar self.head y self.tail en None
    /// - _`Caso2`_: _`head y tail difieren en dirección`_
    /// ```text
    ///             pop_front()
    ///                       head ↓                                                         tail ↓
    ///              ----------------------          ----------------------          ----------------------         
    ///             |      |       |       | <----- |      |       |       | <----  |      |       |       | 
    /// None <----  | PREV |  10   |  NEXT | -----> | PREV |   20  |  NEXT | -----> | PREV |   30  |  NEXT | -----> None
    ///             |      |       |       |        |      |       |       |        |      |       |       |
    ///              ----------------------          ----------------------          ----------------------  
    ///                     head ↓                         tail ↓
    ///              ----------------------          ----------------------                  
    ///             |      |       |       | <----- |      |       |       | <----  
    /// None <----  | PREV |  20   |  NEXT | -----> | PREV |   30  |  NEXT | ----->  None
    ///             |      |       |       |        |      |       |       |        
    ///              ----------------------          ---------------------- 
    /// ```
    /// `Solución`:
    /// - Reasignar el puntero prev del siguiente nodo en None
    /// - Tomar el valor de head actual.
    /// - Reasignar head al nodo que le sigue con respecto al actual de head.
    /// - Disminuir la cantidad del total de nodos en 1
    fn pop_front(&mut self) -> Result<T , String>{
        if self.empty(){
            Err(String::from("La lista se encuentra vacia"))
        }
        else {
            let next_head: Option<Rc<RefCell<DoublyNode<T>>>> = {
                let current_head: &Rc<RefCell<DoublyNode<T>>> = self.head.as_ref().unwrap();
                let borrow_head: Ref<DoublyNode<T>> = current_head.borrow();
                if borrow_head.next.is_some(){
                    borrow_head.next.clone()
                }else {
                    None
                }
            };
            let del_node : T;
            let current_head_node: Rc<RefCell<DoublyNode<T>>> = self.head.take().unwrap();
            del_node = current_head_node.borrow().value;
            if next_head.is_some(){
                self.head = next_head;
            }else {
                self.head = None;
                self.tail = None
            }
            self.size -= 1;
            Ok(del_node)
        }
    }
    ///La estructura actual de la lista asumiendo que existen nodos es la siguiente
    /// ```rust 
    /// Some(
    ///     RefCell(
    ///         value : DoublyNode{
    ///                 prev : None , 
    ///                 value : X , 
    ///                 next : Some(
    ///                     RefCell(
    ///                         value : DoublyNode{
    ///                                 prev : Some(Weak) , 
    ///                                 value : Y , 
    ///                                 next : Some(...)}))})
    /// )
    /// ```
    /// 
    pub fn print_list_directly(&self) -> Result<String , String>{
        if self.empty(){
            Err(String::from("La lista se encuentra vacia"))
        }
        else {
            let mut string_list: String = String::new();
            let mut current: Option<Rc<RefCell<DoublyNode<T>>>> = self.head.clone(); //Recuerda si quieres multiples propietarios entonces has .clone()
            string_list.push_str("Head-> ");
            while let Some(current_node) = current{
                let borrow_current: Ref<DoublyNode<T>> = current_node.borrow();
                println!("{}" , borrow_current.value);
                let format_string: String = format!(" {} ->" , borrow_current.value);
                string_list.push_str(&format_string);
                current = borrow_current.next.clone();
            }
            string_list.push_str(" <- Tail");
            Ok(string_list)
        }
    }
    pub fn print_list_reversely(&self){
        if self.empty(){
            todo!("");
        }else {
            let mut current: Option<Rc<RefCell<DoublyNode<T>>>> = self.tail.clone();
            while let Some(current_node) = current{
                let borrow_current : Ref<DoublyNode<T>> = current_node.borrow();
                println!("{}" , borrow_current.value);
                if let Some(rc) = borrow_current.prev.clone(){
                    let up: Option<Rc<RefCell<DoublyNode<T>>>> = rc.upgrade();
                    current = up;
                }else {
                    break;
                }
            }
        }
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
    #[test]
    fn push_front_test(){
        let mut doubly : DoublyLinkedList<i32> = DoublyLinkedList::new();
        doubly.push_front(30);
        doubly.push_front(40);
        doubly.push_front(50);
        println!("{:?}" , doubly);
    }

    #[test]
    fn push_front_with_print_directly(){
        let mut doubly : DoublyLinkedList<i32> = DoublyLinkedList::new();
        doubly.push_front(30);
        doubly.push_front(40);
        doubly.push_front(50);//<-Head
        println!("{}",doubly.print_list_directly().unwrap());

    }
    #[test]
    fn push_back_with_print_directly(){
        let mut doubly : DoublyLinkedList<i32> = DoublyLinkedList::new();
        doubly.push_back(30); //<- Head
        doubly.push_back(40);
        doubly.push_back(50);
        println!("{}",doubly.print_list_directly().unwrap());

    }
    #[test]
    fn push_back_with_print_reversely(){
        let mut doubly : DoublyLinkedList<i32> = DoublyLinkedList::new();
        doubly.push_back(30);
        doubly.push_back(40);
        doubly.push_back(50); //<-Tail
        doubly.print_list_reversely();

    }
    #[test]
    fn push_front_with_print_reversely(){
        let mut doubly : DoublyLinkedList<i32> = DoublyLinkedList::new();
        doubly.push_front(30); //<-Tail
        doubly.push_front(40);
        doubly.push_front(50);
        doubly.print_list_reversely();

    }
    #[test]
    fn pop_back_test_with_push_back(){
        let mut doubly : DoublyLinkedList<i32> = DoublyLinkedList::new();
        doubly.push_back(30);
        doubly.push_back(50);
        doubly.push_back(100); //<-Tail
        for i in 1..=doubly.len(){
            let data = doubly.pop_back().unwrap();
            println!("{}" , data);
        }
    }
    #[test]
    fn pop_back_test_with_push_front(){
        let mut doubly : DoublyLinkedList<i32> = DoublyLinkedList::new();
        doubly.push_front(30); //<-Tail
        doubly.push_front(50);
        doubly.push_front(100);
        for i in 1..=doubly.len(){
            let data = doubly.pop_back().unwrap();
            println!("{}" , data);
        }
    }
    #[test]
    fn pop_front_test_with_push_front(){
        let mut doubly : DoublyLinkedList<i32> = DoublyLinkedList::new();
        doubly.push_front(30); //<-Tail
        doubly.push_front(50);
        doubly.push_front(100);
        for i in 1..=doubly.len(){
            let data = doubly.pop_front().unwrap();
            println!("{}" , data);
        }
        println!("{:?}" , doubly);
    }
    #[test]
    fn pop_front_test_with_push_back(){
        let mut doubly : DoublyLinkedList<i32> = DoublyLinkedList::new();
        doubly.push_back(30); //<-Tail
        doubly.push_back(50);
        doubly.push_back(100);
        for i in 1..=doubly.len(){
            let data = doubly.pop_front().unwrap();
            println!("{}" , data);
        }
        println!("{:?}" , doubly);
    }
}
