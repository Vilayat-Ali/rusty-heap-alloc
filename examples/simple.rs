use std::{
    alloc::{alloc, Layout},
    mem,
};

use rusty_heap_alloc::Heap;

fn main() {
    let mut my_heap = Heap::init(Some(100)).unwrap();
    my_heap.alloc(5).unwrap();
    println!("{:#?}", my_heap);
}
