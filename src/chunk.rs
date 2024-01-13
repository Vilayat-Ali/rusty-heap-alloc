use crate::error::{HeapError, HeapResult};
use std::{
    alloc::{alloc, dealloc, Layout},
    fmt::{Debug, Display},
    ops::Deref,
};

pub struct Chunk {
    pub ptr: *mut u8,
    pub size: usize,
    pub in_use: bool,
    layout: Layout,
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
                    ptr,
                    size,
                    in_use: false,
                    layout: user_layout,
                    next,
                })
            }
            None => match Layout::from_size_align(size, 1) {
                Ok(validated_layout) => {
                    let ptr = unsafe { alloc(validated_layout) };

                    if ptr.is_null() {
                        return HeapResult::Err(HeapError::ChunkInitError);
                    }

                    HeapResult::Ok(Chunk {
                        ptr,
                        size,
                        in_use: false,
                        layout: validated_layout,
                        next,
                    })
                }
                Err(e) => HeapResult::Err(HeapError::ChunkLayoutError(e)),
            },
        }
    }

    pub fn free(&mut self) {
        unsafe { dealloc(self.ptr, self.layout) };
    }

    pub fn get_layout(&self) -> Layout {
        self.layout
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Chunk [Address: {:#?}] ==> {} bytes",
            self.ptr, self.size
        )
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Chunk [Address: {:#?}] ==> {} bytes",
            self.ptr, self.size
        )
    }
}

impl Deref for Chunk {
    type Target = *mut u8;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}
