use std::alloc::LayoutError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HeapError {
    #[error("Memory Overflow Error: Allocated {0} bytes but {1} bytes were intended!")]
    MemoryOverflow(usize, usize),
    #[error("Memory Underflow Error: allocate memory with capacity >= {0} bytes!")]
    MemoryUnderflow(usize),
    #[error("Heap Error: failed to compact chunks which led to memory corruption in the heap")]
    CoalesceError,
    #[error("Heap Error: failed to fragment chunks which led to memory leak and wastage")]
    FragmentationError,
    #[error("Chunk Error: failed to initialize chunk")]
    ChunkInitError,
    #[error("Chunk Error: failed to initialize chunk due to fault in layout. {0}")]
    ChunkLayoutError(LayoutError),
}

pub enum HeapResult<T> {
    Ok(T),
    Err(HeapError),
}

impl<T> HeapResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(val) => val,
            Self::Err(e) => panic!("{}", e),
        }
    }
}
