///Está es la estructura básica del nodo de una lista enlazada, este posee tolerancia opcional de tipo recursiva
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
struct NodeLink<T> {
    value: T,
    next: Option<Box<NodeLink<T>>>,
}
///Esta estructura compone la opcionalidad de un nodo almacenado en heap y linkea los nodos, de manera logica.
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
impl LinkedList<i32> {
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
    pub fn append(&mut self, value: i32) {
        let new_node: Box<NodeLink<i32>> = Box::new(NodeLink { value, next: None });
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
    fn pop(&mut self){
        todo!("Implementar la función pop");
    }
    fn shirt(&mut self){

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
    pub fn remove_first_ocurrence(&mut self , value : i32) -> Result<i32 , String>{
        if self.empty() { // Head is None -> False
            return Err(String::from("La lista se encuentra vacia"));
        }
        else {
            if let Some(_node) = &mut self.head{
                if _node.value == value{
                    let delete_node = _node.value;
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
                }else {
                    let mut current = &mut self.head;
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
            }
            return Err(String::from("El valor no se encuentra en la lista"));
        }
    }
    pub fn empty(&self) -> bool {
        self.head.is_none()
    }
    pub fn len(&self) -> i32 {
        self.size
    }
    ///### Search
    ///El metodo search de LinkedList retorna una referencia inmutable al nodo que se encuentra en la lista.
    pub fn search(&self, search_value: i32) -> Result<&NodeLink<i32>, bool> {
        if self.empty() {
            Err(false)
        } else {
            let mut current = &self.head;
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
            let mut current = &self.head;
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