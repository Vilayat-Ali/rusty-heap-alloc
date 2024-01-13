pub mod chunk;
pub mod error;

use std::alloc::dealloc;

use chunk::Chunk;

pub const DEFAULT_HEAP_SIZE: usize = 100; // 100 Bytes

#[derive(Debug)]
pub struct Heap {
    pub head: Option<Box<Chunk>>,
    pub chunk_count: usize,
    pub total_space: usize,
    pub free_space: usize,
}

impl Default for Heap {
    fn default() -> Self {
        Self {
            head: None,
            chunk_count: 0,
            total_space: DEFAULT_HEAP_SIZE,
            free_space: DEFAULT_HEAP_SIZE,
        }
    }
}

impl Heap {
    pub fn init(space: Option<usize>) -> Self {
        let mut head: Option<Box<Chunk>> = None;
        let space = space.unwrap_or(DEFAULT_HEAP_SIZE);

        for _ in 0..space {
            let rest_chunks = head.take();
            let new_chunk = Chunk::new(1, None, rest_chunks).unwrap();
            head = Some(Box::new(new_chunk));
        }

        Self {
            head,
            chunk_count: space,
            total_space: space,
            free_space: space,
        }
    }

    fn coalesce(&mut self) {}

    fn fragment(&mut self) {}

    pub fn alloc(size_in_bytes: usize) -> *mut u8 {
        unimplemented!()
    }

    pub fn calloc(element_size: usize, count: usize) -> *mut u8 {
        unimplemented!()
    }
}
