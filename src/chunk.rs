use std::{
    alloc::{alloc, handle_alloc_error, Layout},
    fmt::{Debug, Display},
    mem,
};

pub struct Chunk {
    pub ptr: Option<*mut u8>,
    pub size: usize,
    layout: Option<Layout>,
    pub next: Option<Box<Chunk>>,
}

impl Chunk {
    pub fn new(
        ptr: Option<*mut u8>,
        size: usize,
        layout: Option<Layout>,
        next: Option<Box<Chunk>>,
    ) -> Self {
        Self {
            ptr,
            size,
            layout,
            next,
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
