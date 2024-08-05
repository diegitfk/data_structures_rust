use criterion::{criterion_group, criterion_main , Criterion};
use data_structures::lineal::{safety::{
    double_linked_list::DoublyLinkedList , 
    singly_linked_list::SinglyLinkedList , 
}, unsafety::linked_list};

fn bench_singly_linked_list_append(c : &mut Criterion){
    let mut linked_list : SinglyLinkedList<i32> = SinglyLinkedList::new();
    c.bench_function("append singly", |b| {
        b.iter(||{
            for i in 1..=100{
                linked_list.append(i);
            }
        })
    });
}
fn bench_singly_linked_list_pop(c : &mut Criterion){
    let mut linked_list : SinglyLinkedList<i32> = SinglyLinkedList::new();
    for i in 1..=100{
        linked_list.append(i);
    }
    println!("{:?}" , linked_list.len());
    c.bench_function("pop singly", |b| {
        b.iter(||{
            for _ in 1..=100{
                linked_list.pop();
            }
        })   
    });
    println!("{:?}" , linked_list.len());
}
criterion_group!(
    bench , 
    bench_singly_linked_list_append,
    bench_singly_linked_list_pop
);
criterion_main!(bench);

