use std::cell::RefCell;
use std::rc::{Rc , Weak};
use std::fmt::{Debug , Display};
use std::cmp::Ordering::{self, Equal, Greater, Less};
use num::Integer;
#[derive(Debug , PartialEq , PartialOrd)]
enum Color{
    Red,
    Black
}
#[derive(Debug)]
struct RbNode<T>{
    left : Option<Rc<RefCell<RbNode<T>>>>,
    value : T,
    right : Option<Rc<RefCell<RbNode<T>>>>,
    parent : Option<Weak<RefCell<RbNode<T>>>>,
    color : Color
}

impl <T> RbNode<T>
where T : Integer + Clone + Copy + Display + Debug + Ord{
    pub fn new(value : T) -> Self{
        Self { 
            left: None, 
            value, 
            right: None, 
            parent: None, 
            color: Color::Red 
        }

    }
}
#[derive(Debug)]
struct RedBlackTree<T>{
    root : Option<Rc<RefCell<RbNode<T>>>>,
    size : usize
}
impl <T> RedBlackTree<T>
where T : Integer + Clone + Copy + Display + Debug + Ord{
    pub fn new() -> Self{
        Self { 
            root : None, 
            size: 0 
        }
    }
    pub fn empty(&self) -> bool{
        self.root.is_none() && self.size == 0
    }
    pub fn len(&self) -> usize{
        self.size
    }
    pub fn insert_node(&mut self, value : T){
        self.root = Self::insert_recursibly(self.root.take() , value);
        self.size += 1;
    }
    pub fn insert_recursibly(mut node : Option<Rc<RefCell<RbNode<T>>>> , value : T) -> Option<Rc<RefCell<RbNode<T>>>>{
        match node{
            None => {
                Some(Rc::new(RefCell::new(RbNode::new(value))))
            },
            Some(ref n) => {
                let mut n_borrow = n.borrow_mut(); //una referencia mutable al mismo tiempo para hacer mutaciones secuenciales
                match n_borrow.value.cmp(&value) {
                    Ordering::Greater => {
                        let left = n_borrow.left.clone();
                        n_borrow.left = Self::insert_recursibly(left, value); 
                        n_borrow.parent = Some(Rc::downgrade(n));
                    },
                    Ordering::Equal | Ordering::Less => {
                        let right = n_borrow.right.clone();
                        n_borrow.right = Self::insert_recursibly(right, value);
                        n_borrow.parent = Some(Rc::downgrade(n));
                    }
                }
                drop(n_borrow);
                node
            }

        }

    }
    
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_insertion(){
        let mut tree : RedBlackTree<i32> = RedBlackTree::new();
        tree.insert_node(100);
        //tree.insert_node(200);
        tree.insert_node(60);
        tree.insert_node(60);
        println!("{:?}" , tree);

    }
    fn test_basic_struct(){
        let mut root : Option<Rc<RefCell<RbNode<i32>>>> = Some(Rc::new(RefCell::new(RbNode::new(20))));
        let mut node1 :Option<Rc<RefCell<RbNode<i32>>>> = Some(Rc::new(RefCell::new(RbNode::new(50))));
        if let Some(ref r) = root{
            r.borrow_mut().right = node1.clone();
        }
        if let Some(n) = node1{
            n.borrow_mut().parent = Some(Rc::downgrade(&root.as_ref().and_then(|r| {Some(r.clone())}).unwrap()))
        }
    }
}