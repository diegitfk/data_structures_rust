use num::Integer;
use std::fmt::{Display , Debug};
///### Nodo simple
/// Está es la estructura básica del nodo de una lista enlazada, este posee tolerancia opcional de tipo recursiva
///para poder tolerar el mismo tipo dentro de el.
/// #Ejemplo Visual de la estructura
/// ```text
///             ----------------
///            |       |       |
///            | VALUE |  NEXT |
///            |       |       |
///             ----------------
/// ```
#[derive(Debug)]
pub struct NodeLink<T> {
    value: T,
    next: Option<Box<NodeLink<T>>>,
}
///### LinkedList
/// Esta estructura compone la opcionalidad de un nodo almacenado en heap y linkea los nodos, de manera logica.
///```text
///             ---------------          ---------------              
///            |       |       |        |       |       |            
///            | VALUE |  NEXT | ---->  | VALUE |  NEXT | ----> ....    
///            |       |       |        |       |       |            
///             ---------------          ---------------            
/// ```
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<NodeLink<T>>>,
    size: i32,
}
impl<T> LinkedList<T>
where T : Integer + Clone + Copy + Display + Debug{
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }
    ///### Append
    /// Es un metodo de `LinkedList` encargado de agregar elementos al final de la lista secuencialmente, en este se agrega a la lista al final
    /// y en cada agregación se le aumenta en uno el espacio de la lista.
    /// ```Rust
    ///  fn main() {
    ///    let mut linkedList: LinkedList<i32> = LinkedList::new();
    ///    linkedList.append(20);
    ///    linkedList.append(40);
    /// ```
    /// ```text
    /// 
    /// append(20) -----↓
    ///             ---------------           \  /  
    ///            |       |       |           \/
    ///            |   20  |  NEXT | ---->     /\
    ///            |       |       |          /  \  
    ///             ---------------          /    \  
    ///  
    /// append(40)------------------------------↓
    ///             ---------------          ---------------           \  /  
    ///            |       |       |        |       |       |           \/
    ///            |   20  |  NEXT | ---->  |   40  |  NEXT | ---->     /\
    ///            |       |       |        |       |       |          /  \  
    ///             ---------------          ---------------          /    \  
    /// 
    /// 
    /// ```
    /// 
    pub fn append(&mut self, value: T) {
        let new_node: Box<NodeLink<T>> = Box::new(NodeLink { value, next: None });
        match self.head {
            None => {
                self.head = Some(new_node);
                self.size += 1;
            }
            Some(ref mut node) => {
                let mut current = node;
                while let Some(ref mut next) = current.next {
                    current = next;
                }
                current.next = Some(new_node);
                self.size += 1;
            }
        }
    }
    ///### Pop
    /// Pop es una forma de eliminar datos en una lista simplemente enlazada, la complejidad de hacer pop en una lista simplemente enlazada
    /// en el peor de los casos es O(n) y en el mejor O(1) siempre y cuando la lista posea un espacio de un nodo en la misma.
    /// ```text
    /// Supongamos que contamos con la siguiente lista
    ///           
    ///             ---------------          ---------------         ---------------              \  /  
    ///            |       |       |        |       |       |       |       |       |              \/
    ///            |  10   |  NEXT | ---->  |   20  |  NEXT | ----> |  30   |  NEXT | -------->    /\
    ///            |       |       |        |       |       |       |       |       |             /  \  
    ///             ---------------          ---------------         ---------------             /    \  
    /// ```
    /// #### pop()
    /// ```text
    /// Cuando hacemos pop el penultimo nodo de la lista se enlaza a la referencia siguiente del ultimo nodo de la lista para perder
    /// el enlaze secuencial , eliminandolo de las referencias secuenciales de la siguiente manera.
    ///         
    ///             ---------------          ---------------         ---------------              \  /  
    ///            |       |       |        |       |       |       |       |       |              \/
    ///            |  10   |  NEXT | ---->  |   20  |  NEXT | ----> |  30   |  NEXT | -------->    /\
    ///            |       |       |        |       |       |       |       |       |             /  \  
    ///             ---------------          ---------------         ---------------             /    \  
    /// 
    ///             ---------------          ---------------         ---------------              \  /  
    ///            |       |       |        |       |       |       |       |       |              \/
    ///            |  10   |  NEXT | ---->  |   20  |  NEXT |       |  30   |  NEXT | -------->    /\
    ///            |       |       |        |       |       |       |       |       |             /  \  
    ///             ---------------          ---------------         ---------------             /    \ 
    ///                                                |____________________________________________↑
    /// 
    ///             ---------------          ---------------             \  /  
    ///            |       |       |        |       |       |             \/
    ///            |  10   |  NEXT | ---->  |   20  |  NEXT | ---->       /\
    ///            |       |       |        |       |       |            /  \  
    ///             ---------------          ---------------            /    \  
    /// 
    /// ```
 
    pub fn pop(&mut self) -> Result<T , String>{
        if self.empty(){
            return Err(String::from("La lista se encuentra vacia"));
        }
        if let Some(ref mut node) = self.head{
            if node.next.is_none(){ //en el caso de que solamente sea el head en la lista
                let remove_value: T = node.value;
                self.head = node.next.take();
                self.size -= 1;
                return Ok(remove_value);
            }

        }
        let mut current: &mut Box<NodeLink<T>> = self.head.as_mut().unwrap();
        while let Some(ref mut next) = current.next{
            if next.next.is_none(){
                let last_one: Box<NodeLink<T>> = current.next.take().unwrap();
                return Ok(last_one.value);
            }
            current = current.next.as_mut().unwrap();
        }
        Err(String::from("Error inesperado"))
    }
    ///### Shirt
    /// Shirt es una forma de eliminar datos en una lista simplemente enlazada, la complejidad de hacer shirt en una lista simplemente enlazada
    /// en es O(1) debido a que son unos pequeños movimientos de la cabeza de la lista, a donde apunta.
    /// ```text
    /// Supongamos que contamos con la siguiente lista
    ///           
    ///             ---------------          ---------------         ---------------              \  /  
    ///            |       |       |        |       |       |       |       |       |              \/
    ///            |  10   |  NEXT | ---->  |   20  |  NEXT | ----> |  30   |  NEXT | -------->    /\
    ///            |       |       |        |       |       |       |       |       |             /  \  
    ///             ---------------          ---------------         ---------------             /    \  
    /// ```
    /// #### shirt()
    /// ```text
    /// Cuando hacemos shirt el penultimo nodo de la lista se enlaza a la referencia siguiente del primer nodo de la lista para perder
    /// el enlaze secuencial , eliminandolo de las referencias secuenciales de la siguiente manera.
    /// 
    ///                head --------------------↓  
    ///             ---------------          ---------------         ---------------              \  /  
    ///            |       |       |        |       |       |       |       |       |              \/
    ///            |  10   |  NEXT | ---->  |   20  |  NEXT | ----> |  30   |  NEXT | -------->    /\
    ///            |       |       |        |       |       |       |       |       |             /  \  
    ///             ---------------          ---------------         ---------------             /    \  
    ///               head -------------------↓
    ///             ---------------         ---------------              \  /  
    ///            |       |       |       |       |       |              \/
    ///            |   20  |  NEXT | ----> |  30   |  NEXT | -------->    /\
    ///            |       |       |       |       |       |             /  \  
    ///             ---------------         ---------------             /    \
    ///            
    ///              head ↓
    ///             ---------------        \  /  
    ///            |       |       |        \/
    ///            |   30  |  NEXT | ---->  /\
    ///            |       |       |       /  \  
    ///             ---------------       /    \ 
    /// 
    /// ```
    pub fn shirt(&mut self) -> Result<T , String>{
        if self.empty(){
            return Err(String::from("La lista se encuentra vacia"));
        }
        let current: &mut Option<Box<NodeLink<T>>> = &mut self.head;
        if let Some(ref mut head) = current{
            let removed_value: T = head.value;
            if head.next.is_none(){
                self.head = None;
                self.size -= 1;
                return Ok(removed_value);
            }else {
                self.head = head.next.take();
                self.size -= 1;
                return Ok(removed_value);
            }
        } 
        Err(String::from("No se pudo eliminar el nodo"))

    }
    ///Hay que buscar el nodo previo a la primera ocurrencia y enlazarlo con el siguiente a la primera ocurrencia
    ///por tanto hay que tener el previo y el siguiente al nodo a eliminar para enlazarlos
    /// ### Remove_first_ocurrence
    ///```text
    ///  Casos de eliminación
    ///  Caso1: El nodo a eliminar se encuentra entre dos nodos `nodo -> nodo_a_eliminar -> nodo`
    ///                  previo↓                 eliminar ↓             
    ///             ---------------          ---------------         ---------------              \  /  
    ///            |       |       |  \ /   |       |       |  \ /  |       |       |              \/
    ///            | VALUE |  NEXT | ---->  | VALUE |  NEXT | ----> | VALUE |  NEXT | -------->    /\
    ///            |       |       |  / \   |       |       |  / \  |       |       |             /  \  
    ///             ---------------          ---------------         ---------------             /    \  
    ///                        |_______________________________________↑
    ///  Caso2 : El nodo a eliminar se encuentra en la cabeza `nodo_a_eliminar->nodo->...`
    ///
    ///               head ↓ ----Move head---------↓
    ///               eliminar ↓             head=siguiente↓
    ///             ---------------          ---------------         ---------------              \  /  
    ///            |       |       |        |       |       |       |       |       |              \/
    ///            | VALUE |  NEXT | ---->  | VALUE |  NEXT | ----> | VALUE |  NEXT | -------->    /\
    ///            |       |       |        |       |       |       |       |       |             /  \  
    ///             ---------------          ---------------         ---------------             /    \  
    ///              
    ///  Caso3: El nodo a eliminar se encuentra al final de la lista `nodo -> nodo_a_eliminar -> None`
    ///                                            previo↓               eliminar ↓            siguiente↓
    ///             ---------------          ---------------         ---------------              \  /  
    ///            |       |       |        |       |       |       |       |       |              \/
    ///            | VALUE |  NEXT | ---->  | VALUE |  NEXT | ----> | VALUE |  NEXT | -------->    /\
    ///            |       |       |        |       |       |       |       |       |             /  \  
    ///             ---------------          ---------------         ---------------             /    \  
    ///                                                  |__________________________________________↑
    /// ```
    pub fn remove_first_ocurrence(&mut self , value : T) -> Result<T , String>{
        if self.empty() { // Head is None -> False
            return Err(String::from("La lista está vacía"));
        }
        if let Some(_node) = &mut self.head{
            if _node.value == value{
                let delete_node: T = _node.value;
                match &_node.next {
                    None => {
                        self.head = None;
                        self.size -= 1;
                        return Ok(delete_node);
                    }, 
                    Some(_node_next) => {
                        self.head = _node.next.take();
                        self.size -= 1;
                        return Ok(delete_node);
                    }
                }
            }
        let mut current: &mut Option<Box<NodeLink<T>>> = &mut self.head;
        while let Some(ref mut _node) = current{
            if let Some(_node_next) = &mut _node.next{
                if _node_next.value == value{ //estamos a la mitad de la lista
                    let value_deleted = _node_next.value;
                    _node.next = _node_next.next.take();
                    self.size -= 1;
                    return Ok(value_deleted);
                    }
                }
                current = &mut _node.next;
            }
        }
        Err(String::from("No se encuentra el valor en la lista"))
    }
    pub fn empty(&self) -> bool {
        self.head.is_none()
    }
    pub fn len(&self) -> i32 {
        self.size
    }
    ///### Search
    ///El metodo search de LinkedList retorna una referencia inmutable al nodo que se encuentra en la lista.
    pub fn search(&self, search_value: T) -> Result<&NodeLink<T>, bool> {
        if self.empty() {
            Err(false)
        } else {
            let mut current: &Option<Box<NodeLink<T>>> = &self.head;
            while let Some(node) = &current {
                if node.value == search_value {
                    return Ok(&node);
                }
                current = &node.next;
            }
            Err(false)
        }
    }
    pub fn see_list(&self) -> Result<String, String> {
        let mut string_list: String = String::new();
        if self.empty() {
            Err(String::from(
                "No se puede visualizar la lista porque se encuentra vacia!",
            ))
        } else {
            let mut current: &Option<Box<NodeLink<T>>> = &self.head;
            while let Some(node) = current {
                println!("{:?}", node);
                let format_string: String = format!(" {} ->", node.value);
                string_list.push_str(&format_string);
                current = &node.next;
            }

            Ok(string_list)
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn correct_creation(){
        let mut list : LinkedList<i32> = LinkedList::new();
    }
    #[test]
    fn add_values_to_list(){
        let mut list: LinkedList<i32> = LinkedList::new();
        list.append(10);
        list.append(20);
        list.append(30);
        list.append(40);
        assert_eq!(list.len() , 4 , "No coinciden los valores de prueba con espacio de la lista {}" , list.see_list().unwrap());
    }
    #[test]
    fn delete_value_on_head(){
        let mut list: LinkedList<i32> = LinkedList::new();
        list.append(10);
        list.append(20);
        list.append(30);
        list.append(40);
        assert_eq!(list.shirt().unwrap() , 10);
        assert_eq!(list.shirt().unwrap() , 20);
        assert_eq!(list.shirt().unwrap() , 30);
        assert_eq!(list.shirt().unwrap() , 40);
    }
    #[test]
    fn delete_value_with_pop(){
        let mut list: LinkedList<i32> = LinkedList::new();
        list.append(10);
        list.append(20);
        list.append(30);
        list.append(40);
        assert_eq!(list.pop().unwrap() , 40);
        assert_eq!(list.pop().unwrap() , 30);
        assert_eq!(list.pop().unwrap() , 20);
        assert_eq!(list.pop().unwrap() , 10);
    }
    #[test]
    fn remove_first_ocurrence_test(){
        let mut list: LinkedList<i32> = LinkedList::new();
        list.append(10);
        list.append(20);
        list.append(30);
        list.append(40);
        list.append(50);
        list.append(60);
        list.append(70);
        assert_eq!(list.remove_first_ocurrence(10).unwrap() , 10 , "No coincide el valor eliminado con el comparado");
        assert_eq!(list.len() , 6);
        assert_eq!(list.remove_first_ocurrence(30).unwrap() , 30 , "No coincide el valor eliminado con el comparado");
        assert_eq!(list.len() , 5);
        assert_eq!(list.remove_first_ocurrence(70).unwrap() , 70 , "No coincide el valor eliminado con el comparado");
        assert_eq!(list.len() , 4);
    }
}