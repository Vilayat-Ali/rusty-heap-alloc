use crate::error::{HeapError, HeapResult};
use std::{
    alloc::{alloc, Layout},
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub struct Chunk {
    pub ptr: Option<*mut u8>,
    pub size: usize,
    layout: Option<Layout>,
    pub next: Option<Box<Chunk>>,
}

impl Chunk {
    pub fn new(size: usize, layout: Option<Layout>, next: Option<Box<Chunk>>) -> HeapResult<Self> {
        match layout {
            Some(user_layout) => {
                let ptr = unsafe { alloc(user_layout) };

                if ptr.is_null() {
                    return HeapResult::Err(HeapError::ChunkInitError);
                }

                HeapResult::Ok(Chunk {
                    ptr: Some(ptr),
                    size,
                    layout: Some(user_layout),
                    next: None,
                })
            }
            None => match Layout::from_size_align(size, 1) {
                Ok(validated_layout) => {
                    let ptr = unsafe { alloc(validated_layout) };

                    if ptr.is_null() {
                        return HeapResult::Err(HeapError::ChunkInitError);
                    }

                    HeapResult::Ok(Chunk {
                        ptr: Some(ptr),
                        size,
                        layout: Some(validated_layout),
                        next: None,
                    })
                }
                Err(e) => HeapResult::Err(HeapError::ChunkLayoutError(e)),
            },
        }
    }

    pub fn get_layout(&self) -> Option<&Layout> {
        self.layout.as_ref()
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            ptr: None,
            size: 0,
            layout: None,
            next: None,
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Chunk [Address: {:#?}] ==> {} bytes",
            self.ptr.unwrap(),
            self.size
        )
    }
}

// impl Debug for Chunk {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "Chunk [Address: {:#?}] ==> {} bytes",
//             self.ptr, self.size
//         )
//     }
// }
