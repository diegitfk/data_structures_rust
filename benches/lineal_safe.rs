use criterion::{criterion_group, criterion_main , Criterion};
use data_structures::lineal::{safety::{
    double_linked_list::DoublyLinkedList , 
    singly_linked_list::SinglyLinkedList , 
}, unsafety::linked_list};

fn bench_all_operations_singly_linked_list(c : &mut Criterion){
    let mut linked_list : SinglyLinkedList<i32> = SinglyLinkedList::new();
    c.bench_function("append singly", |b| {
        b.iter(||{
            for i in 1..=100{
                linked_list.append(i);
            }
        })
    });
    c.bench_function("pop singly", |b| {
        b.iter(||{
            for _ in 1..=50{
                linked_list.pop();
            }
        })   
    });
}

criterion_group!(
    bench , 
    bench_all_operations_singly_linked_list
);
criterion_main!(bench);

