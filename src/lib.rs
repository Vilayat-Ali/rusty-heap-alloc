pub mod chunk;
pub mod error;

use std::{fmt::Debug, ops::Div};

use chunk::Chunk;
use error::{
    HeapError,
    HeapResult::{self, Err, Ok},
};

pub const DEFAULT_HEAP_SIZE: usize = 100; // 100 Bytes

pub struct Heap {
    pub head: Option<Box<Chunk>>,
    pub chunk_count: usize,
    pub total_space: usize,
    pub free_space: usize,
}

impl Debug for Heap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers = vec![
            "Chunk ID",
            "Memory Address",
            "Size In Bytes",
            "Usage Status",
        ];
        let mut size_arr: Vec<usize> = headers.iter().map(|x| x.len() + 2).collect();

        let rows = {
            let mut rows: Vec<Vec<String>> = Vec::new();
            let mut start = &self.head;
            let mut id: usize = 1;

            while start.is_some() {
                let curr_chunk = start.as_ref().unwrap().as_ref();
                rows.push(vec![
                    {
                        let curr_id = id.to_string();

                        if curr_id.len() > size_arr[0] {
                            size_arr[0] = curr_id.len() + 2;
                        }

                        curr_id
                    },
                    {
                        let mem_addr = format!("{:?}", curr_chunk.ptr);

                        if mem_addr.len() > size_arr[1] {
                            size_arr[1] = mem_addr.len() + 2;
                        }

                        mem_addr
                    },
                    {
                        let chunk_size = curr_chunk.size.to_string();

                        if chunk_size.len() > size_arr[2] {
                            size_arr[2] = chunk_size.len() + 2;
                        }

                        chunk_size
                    },
                    {
                        let status = match curr_chunk.in_use {
                            true => "IN USE".to_string(),
                            false => "FREE".to_string(),
                        };

                        if status.len() > size_arr[3] {
                            size_arr[3] = status.len() + 2;
                        }

                        status
                    },
                ]);

                id += 1;
                start = &curr_chunk.next;
            }

            rows
        };

        println!("{:?}", size_arr);

        let draw_hr_line = || {
            let mut line = String::from("+");
            for size in size_arr.iter() {
                line.push_str(&format!("{}", "-".repeat(*size)));
                line.push('+');
            }
            println!("{}", line);
        };

        draw_hr_line();

        let mut header_row = String::from("|");

        for i in 0..headers.len() {
            let spacing = match (size_arr[i] - headers[i].len()) % 2 == 0 {
                true => {
                    let space = (size_arr[i] - headers[i].len()).div(2);
                    (space, space)
                }
                false => {
                    let space = (size_arr[i] - headers[i].len()).div(2);
                    (space, space + 1)
                }
            };

            header_row.push_str(&format!(
                "{}{}{}",
                " ".repeat(spacing.0),
                headers[i],
                " ".repeat(spacing.1)
            ));

            header_row.push('|');
        }

        println!("{}", header_row);
        draw_hr_line();

        for row_no in 0..rows.len() {
            let mut formatted_row = String::from("|");
            for cell_no in 0..rows[row_no].len() {
                let spacing = match (size_arr[cell_no] - rows[row_no][cell_no].len()) % 2 == 0 {
                    true => {
                        let space = (size_arr[cell_no] - rows[row_no][cell_no].len()).div(2);
                        (space, space)
                    }
                    false => {
                        let space = (size_arr[cell_no] - rows[row_no][cell_no].len()).div(2);
                        (space, space + 1)
                    }
                };

                formatted_row.push_str(&format!(
                    "{}{}{}",
                    " ".repeat(spacing.0),
                    rows[row_no][cell_no],
                    " ".repeat(spacing.1)
                ));

                formatted_row.push('|');
            }
            println!("{}", formatted_row);

            draw_hr_line();
        }

        writeln!(f, "")
    }
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
    pub fn init(space: Option<usize>) -> HeapResult<Self> {
        let mut head: Option<Box<Chunk>> = None;

        if space.is_some() {
            let space_val = space.unwrap();
            if space_val < 10 {
                return Err(HeapError::MemoryUnderflow(space_val));
            }
        };

        let size = match space {
            Some(user_desired_space) => user_desired_space / 10,
            None => DEFAULT_HEAP_SIZE / 10,
        };

        let space: usize = space.unwrap_or(DEFAULT_HEAP_SIZE);

        for _ in 0..space {
            let rest_chunks = head.take();
            let new_chunk = Chunk::new(size, None, rest_chunks).unwrap();
            head = Some(Box::new(new_chunk));
        }

        Ok(Self {
            head,
            chunk_count: space,
            total_space: space,
            free_space: space,
        })
    }

    fn has_available_space_as_chunks(&mut self, needed_size: usize) -> bool {
        let mut start = &self.head;

        if needed_size == 0 {
            panic!(
                "{}",
                HeapError::InvalidInputError(needed_size, "has_available_space_as_chunks")
            );
        }

        self.free_space = 0;

        while start.is_some() {
            match start {
                Some(ref chunk_val) => {
                    if !chunk_val.in_use {
                        // chunk is marked not used
                        self.free_space += chunk_val.size;
                    }
                    start = &chunk_val.next;
                }
                None => unreachable!(),
            }
        }

        self.free_space > needed_size
    }

    pub fn alloc(&mut self, size_in_bytes: usize) -> HeapResult<Chunk> {
        if size_in_bytes > self.free_space {
            return Err(HeapError::MemoryOverflow(size_in_bytes, self.free_space));
        }

        println!("{}", self.has_available_space_as_chunks(13));

        Ok(Chunk::new(10, None, None).unwrap())
    }

    pub fn calloc(element_size: usize, count: usize) -> *mut u8 {
        unimplemented!()
    }
}
