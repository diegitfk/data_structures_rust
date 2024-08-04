//! # Introducción a `data_structures`: Estructuras de Datos Seguras y No Seguras en Rust
//! 
//! Bienvenido a `data_structures`, un crate diseñado para proporcionar implementaciones tanto seguras como no seguras de diversas estructuras de datos en Rust. Este crate tiene como objetivo ilustrar las diferencias y las consideraciones que se deben tener en cuenta al trabajar con enfoques de seguridad (safe) y no seguridad (unsafe) en Rust.
//! 
//! ## Seguridad en Rust: `Safe`
//! 
//! Rust es un lenguaje conocido por su enfoque en la seguridad de la memoria, ofreciendo garantías de seguridad en tiempo de compilación mediante su sistema de tipos y el concepto de propiedad. En el módulo `safe`, encontrarás implementaciones de estructuras de datos que aprovechan al máximo estas características de Rust, asegurando que las operaciones comunes se realicen sin riesgo de errores comunes como desbordamientos de buffer, uso después de liberar (use-after-free), y condiciones de carrera.
//! 
//! ### Ejemplos de Estructuras de Datos Seguras
//! - **Listas Enlazadas**: Implementadas utilizando `Option<Box<Node<T>>>` para gestionar de manera segura los nodos de la lista.
//! - **Árboles Binarios**: Implementados utilizando referencias y contadores de referencias (`Rc<T>` y `RefCell<T>`) para garantizar la seguridad de acceso.
//! 
//! ## No Seguridad en Rust: `Unsafe`
//! 
//! Aunque Rust ofrece potentes herramientas para la seguridad de la memoria, hay situaciones donde el rendimiento o las necesidades específicas del hardware requieren el uso de código no seguro (unsafe). El módulo `unsafe` contiene implementaciones de estructuras de datos que utilizan punteros crudos (`raw pointers`) y bloques `unsafe`, permitiendo un mayor control sobre la memoria y el comportamiento del programa, pero con la responsabilidad añadida de evitar errores que el compilador no puede detectar.
//! 
//! ### Ejemplos de Estructuras de Datos No Seguras
//! - **Listas Enlazadas**: Implementadas utilizando punteros crudos (`*mut Node<T>`) para gestionar manualmente la memoria y los enlaces entre nodos.
//! - **Vectores**: Construidos desde cero utilizando asignación manual de memoria y gestión explícita de los límites y capacidades.
//! - **Árboles Binarios**: Implementaciones que utilizan punteros crudos para la creación y manipulación directa de nodos en la memoria.
//! 
//! ## Consideraciones de Diseño
//! 
//! ### Enfoque Seguro
//! El enfoque seguro se centra en garantizar que todas las operaciones son seguras en términos de memoria y concurrencia. Estas implementaciones están diseñadas para ser fáciles de usar y mantener, aprovechando las características de seguridad de Rust.
//! 
//! ### Enfoque No Seguro
//! El enfoque no seguro proporciona una visión más cercana al hardware y puede ser necesario para ciertos casos de uso donde el control detallado y el rendimiento son críticos. Sin embargo, estas implementaciones requieren un entendimiento profundo de la gestión de la memoria y un cuidado extremo para evitar errores.
//! 
//! ### Elección del Enfoque
//! La elección entre los enfoques seguro y no seguro depende del contexto y los requisitos específicos de tu aplicación. Si la seguridad y la facilidad de mantenimiento son prioritarias, el enfoque seguro es el adecuado. Si necesitas un control fino y rendimiento óptimo, y estás dispuesto a asumir la responsabilidad adicional, el enfoque no seguro puede ser la mejor opción.
//! ### Acerca de los dos grandes Modulos
//! El create posee dos grandes modulos que concentran el enfoque Safe (safety) y Unsafe (unsafety), estos son `lineal` y `no_lineal`
//! estos modulos almacenan respectivamente estructuras de caracter lineal, como tablas de hash, linked lists , etc y el otro arboles de busqueda
//! binaria, arboles AVL , arboles rojo-negros, arboles B+ , grafos , etc.
/// Un modulo netamente enfocado a estructuras de datos que son lineales!
pub mod lineal{
    //! # `data_structures`: Estructuras Lineales en Rust
    //! 
    //! Este crate proporciona implementaciones de estructuras de datos lineales, diseñadas para ilustrar y educar sobre el uso de Rust en el manejo seguro y eficiente de memoria. Actualmente, este crate incluye implementaciones de listas enlazadas (simples y dobles) y pilas.
    //! 
    //! ## Estructuras de Datos Incluidas
    //! 
    //! ### Listas Enlazadas Simplemente Enlazadas (Singly Linked Lists)
    //! 
    //! Las listas enlazadas simplemente enlazadas son estructuras de datos en las que cada nodo contiene un valor y una referencia al siguiente nodo en la secuencia. Son útiles para situaciones en las que las inserciones y eliminaciones frecuentes ocurren en la parte delantera de la lista.
    //! 
    //! #### Características
    //! - **Inserción**: Puede insertar elementos al frente o al final de la lista.
    //! - **Eliminación**: Puede eliminar elementos del frente de la lista.
    //! - **Iteración**: Permite iterar sobre los elementos de la lista.
    //! 
    //! ### Listas Enlazadas Doblemente Enlazadas (Doubly Linked Lists)
    //! 
    //! Las listas enlazadas doblemente enlazadas son una extensión de las listas simplemente enlazadas, donde cada nodo contiene una referencia tanto al siguiente nodo como al nodo anterior. Esto permite una navegación bidireccional y facilita las operaciones de inserción y eliminación en cualquier posición de la lista.
    //! 
    //! #### Características
    //! - **Inserción**: Puede insertar elementos al frente, en el medio o al final de la lista.
    //! - **Eliminación**: Puede eliminar elementos del frente, del medio o del final de la lista.
    //! - **Iteración**: Permite iterar sobre los elementos de la lista en ambas direcciones (hacia adelante y hacia atrás).
    //! 
    //! ### Pilas (Stacks)
    //! 
    //! Las pilas son estructuras de datos LIFO (Last In, First Out), donde el último elemento insertado es el primero en ser eliminado. Son útiles en situaciones donde se necesita un almacenamiento temporal de datos, como en la evaluación de expresiones, la gestión de la recursión y la reversión de operaciones.
    //! 
    //! #### Características
    //! - **Inserción (Push)**: Puede insertar elementos en la parte superior de la pila.
    //! - **Eliminación (Pop)**: Puede eliminar y devolver el elemento en la parte superior de la pila.
    //! - **Peek**: Permite ver el elemento en la parte superior de la pila sin eliminarlo.
    //! 
    //! ## Enfoque Seguro y No Seguro
    //! 
    //! Este crate está diseñado para proporcionar implementaciones tanto seguras como no seguras de las estructuras de datos. 
    //! 
    //! ### Enfoque Seguro (`Safe`)
    //! 
    //! Las implementaciones seguras aprovechan al máximo las características de Rust, como la propiedad, el sistema de tipos y los contadores de referencias (`Rc<T>`, `RefCell<T>`) para garantizar la seguridad de la memoria y evitar errores comunes.
    //! 
    //! ### Enfoque No Seguro (`Unsafe`)
    //! 
    //! Las implementaciones no seguras utilizan punteros crudos (`raw pointers`) y bloques `unsafe` para proporcionar un mayor control sobre la memoria y el rendimiento. Estas implementaciones requieren un cuidado adicional para evitar errores que el compilador no puede detectar.
    //! 
    //! ## Ejemplos de Uso
    //! 
    //! ```rust
    //! // Ejemplo de uso de una lista simplemente enlazada
    //! use data_structures::safety::singly_linked_list::SinglyLinkedList;
    //! 
    //! let mut list = SinglyLinkedList::new();
    //! list.append(1);
    //! list.append(2);
    //! list.append(3);
    //! 
    //! while let Some(value) = list.pop() {
    //!     println!("{}", value);
    //! }
    //! 
    //! // Ejemplo de uso de una pila
    //! use data_structures::safety::stack::Stack;
    //! 
    //! let mut stack = Stack::new();
    //! stack.push(1);
    //! stack.push(2);
    //! stack.push(3);
    //! 
    //! while let Some(value) = stack.pop() {
    //!     println!("{}", value);
    //! }
    //! ```
    //! 
    //! ## Conclusión
    //! 
    //! `data_structures` proporciona una base sólida para el aprendizaje y la implementación de estructuras de datos lineales en Rust, abordando tanto enfoques seguros como no seguros. Explora las diferentes estructuras y sus implementaciones para comprender mejor cómo manejar la memoria y optimizar el rendimiento en Rust.

    pub mod safety{
        ///Una impelemntación safe de una lista simplemente enlazada!!
        pub mod singly_linked_list;
        ///Una implementación safe de una lista doblemente enlazada!!
        pub mod double_linked_list;
        /// Una implementación safe de un stack!!
        pub mod stack;
    }
    pub mod unsafety{
        pub mod linked_list;
        pub mod double_linked_list;
        pub mod stack;
    }
}
/// Un modulo netamente enfocado a estructuras de datos que no son lineales!
pub mod no_lineal{
    //! # `data_structures::no_lineal`: Estructuras de Datos No Lineales en Rust
    //! 
    //! Este módulo proporciona implementaciones de estructuras de datos no lineales, diseñadas para ilustrar y educar sobre el uso de Rust en el manejo seguro y eficiente de memoria. Actualmente, este módulo incluye implementaciones de árboles de búsqueda binaria (Binary Search Trees), árboles AVL (AVL Trees) y árboles rojo-negro (Red-Black Trees).
    //! 
    //! ## Estructuras de Datos Incluidas
    //! 
    //! ### Árboles de Búsqueda Binaria (Binary Search Trees)
    //! 
    //! Los árboles de búsqueda binaria son estructuras de datos que mantienen sus elementos en un orden específico, permitiendo una búsqueda, inserción y eliminación eficientes. Cada nodo tiene como máximo dos hijos, y el subárbol izquierdo contiene solo nodos con valores menores que el nodo raíz, mientras que el subárbol derecho contiene solo nodos con valores mayores.
    //! 
    //! #### Características
    //! - **Inserción**: Permite insertar elementos en su posición correcta para mantener el orden.
    //! - **Eliminación**: Permite eliminar elementos, ajustando el árbol para mantener el orden.
    //! - **Búsqueda**: Permite buscar elementos de manera eficiente debido a la estructura ordenada del árbol.
    //! 
    //! ### Árboles AVL (AVL Trees)
    //! 
    //! Los árboles AVL son una variante de los árboles de búsqueda binaria que mantienen un balanceo estricto para asegurar que las operaciones de búsqueda, inserción y eliminación se realicen en tiempo logarítmico. Cada nodo en un árbol AVL tiene un factor de balance que indica la diferencia de altura entre sus subárboles izquierdo y derecho.
    //! 
    //! #### Características
    //! - **Inserción**: Permite insertar elementos mientras mantiene el balance del árbol.
    //! - **Eliminación**: Permite eliminar elementos ajustando el árbol para mantener el balance.
    //! - **Rotaciones**: Utiliza rotaciones simples y dobles para reequilibrar el árbol después de las inserciones y eliminaciones.
    //! 
    //! ### Árboles Rojo-Negro (Red-Black Trees)
    //! 
    //! Los árboles rojo-negro son otra variante de los árboles de búsqueda binaria que aseguran un balanceo aproximado utilizando nodos con colores (rojo o negro) y un conjunto de reglas de balance. Esto garantiza que las operaciones se realicen en tiempo logarítmico.
    //! 
    //! #### Características
    //! - **Inserción**: Permite insertar elementos mientras aplica las reglas de coloración y rotaciones para mantener el balance.
    //! - **Eliminación**: Permite eliminar elementos ajustando el árbol y aplicando las reglas necesarias para mantener el balance.
    //! - **Propiedades de los Nodos**: Mantiene reglas estrictas sobre la coloración y la estructura del árbol para asegurar su balance.
    //! 
    //! ## Enfoque Seguro y No Seguro
    //! 
    //! Este módulo está diseñado para proporcionar implementaciones tanto seguras como no seguras de las estructuras de datos.
    //! 
    //! ### Enfoque Seguro (`Safe`)
    //! 
    //! Las implementaciones seguras aprovechan al máximo las características de Rust, como la propiedad, el sistema de tipos y los contadores de referencias (`Rc<T>`, `RefCell<T>`) para garantizar la seguridad de la memoria y evitar errores comunes.
    //! 
    //! ### Enfoque No Seguro (`Unsafe`)
    //! 
    //! Las implementaciones no seguras utilizan punteros crudos (`raw pointers`) y bloques `unsafe` para proporcionar un mayor control sobre la memoria y el rendimiento. Estas implementaciones requieren un cuidado adicional para evitar errores que el compilador no puede detectar.
    //! 
    //! ## Ejemplos de Uso
    //! 
    //! ```rust
    //! // Ejemplo de uso de un árbol de búsqueda binaria
    //! use data_structures::no_lineal::safety::binary_search_tree::BinarySearchTree;
    //! 
    //! let mut bst = BinarySearchTree::new();
    //! bst.insert_node(10);
    //! bst.insert_node(5);
    //! bst.insert_node(15);
    //! 
    //! if let Some(node) = bst.search(10) {
    //!     println!("Encontrado: {}", node.value);
    //! }
    //! 
    //! // Ejemplo de uso de un árbol AVL
    //! use data_structures::no_lineal::unsafety::avl_tree::AvlTree;
    //! 
    //! let mut avl = AvlTree::new();
    //! avl.insert_node(10);
    //! avl.insert_node(5);
    //! avl.insert_node(15);
    //! 
    //! if let Some(node) = avl.search(10) {
    //!     println!("Encontrado: {}", node.value);
    //! }
    //! ```
    //! ## Conclusión
    //! 
    //! `data_structures::non_linear` proporciona una base sólida para el aprendizaje y la implementación de estructuras de datos no lineales en Rust, abordando tanto enfoques seguros como no seguros. Explora las diferentes estructuras y sus implementaciones para comprender mejor cómo manejar la memoria y optimizar el rendimiento en Rust.
    pub mod safety{
        ///Una implementación safe de binary search tree
        pub mod binary_search_tree;
        ///Una implementación safe de avl
        pub mod avl_tree;
        ///Una implementación safe de un red black tree
        pub mod rb_tree;
    }
    pub mod unsafety{
        pub mod binary_search_tree;
        pub mod avl_tree;
        pub mod rb_tree;
    }
}