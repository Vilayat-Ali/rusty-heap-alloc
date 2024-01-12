use std::{
    alloc::{alloc, Layout},
    mem,
};

use rusty_heap_alloc::Heap;

fn main() {
    let my_heap = Heap::init(10);
    println!("{:#?}", my_heap);
}
