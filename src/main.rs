use data_structures::LinkedList;
fn main() {
    let mut linkedList: LinkedList<i32> = LinkedList::new();
    linkedList.append(20);
    linkedList.append(40);
    linkedList.append(50);
    linkedList.append(70);
    //println!("Nodos:{:?}\nEspacio: {}\nVacia: {}" , linkedList , &linkedList.len() , &linkedList.empty());
    let value = linkedList.pop().unwrap();
    println!("{:?}" , linkedList);
}
