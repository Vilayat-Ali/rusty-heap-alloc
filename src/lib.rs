pub mod chunk;
pub mod error;

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
    pub fn init(space: usize) -> Self {
        let mut head: Option<Box<Chunk>> = None;

        for _ in 0..space {
            let mut new_chunk = Chunk::new(1, None, None).unwrap();
            let rest_chunks = head.take();
            new_chunk.next = rest_chunks;
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

    pub fn free(&mut self) {
        unimplemented!()
    }
}
