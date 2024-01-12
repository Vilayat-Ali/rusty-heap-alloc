# Rusty Heap Alloc

Rusty Heap Alloc is a Rust project that provides a simple memory allocator for heap allocation. This README provides an overview of heap memory, the purpose of this project, and usage examples.

## Table of Contents

- [What is Heap Memory?](#what-is-heap-memory)
  - [Heap vs Stack](#heap-vs-stack)
- [Rusty Heap Alloc](#rusty-heap-alloc)
  - [Features](#features)
- [Usage](#usage)
  - [Allocation](#allocation)
  - [Deallocation](#deallocation)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## What is Heap Memory?

In computer science, the heap is a region of memory used for dynamic memory allocation. Unlike the stack, which is a region of memory used for function calls and local variables, the heap is used for memory that needs to persist beyond the scope of a single function. Heap memory is managed explicitly by the programmer and can be allocated and deallocated as needed.

### Heap vs Stack

- **Heap:**
  - Allocated at runtime.
  - Dynamic size.
  - Manual memory management (allocating and deallocating).
  - Longer lifespan.

- **Stack:**
  - Allocated at compile time.
  - Fixed size.
  - Automatic memory management (push and pop operations).
  - Shorter lifespan.

## Rusty Heap Alloc

Rusty Heap Alloc is a lightweight memory allocator for heap memory management in Rust. It aims to provide a simple interface for allocating and deallocating memory.

### Features

- Allocation of memory chunks on the heap.
- Deallocation of memory chunks.

## Usage

To use Rusty Heap Alloc in your Rust project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
rusty_heap_alloc = "0.1.0"
```

## Allocation

To allocate a memory chunk, use the allocate function:

```rust
use rusty_heap_alloc::heap::Chunk;

fn main() {
    let mut chunk = Chunk::allocate(64);
    // Your code here...
}
```

## Deallocatin

To deallocate a memory chunk, use the deallocate function:

```rust
use rusty_heap_alloc::heap::deallocate;

fn main() {
    let mut chunk = Chunk::allocate(64);
    // Your code here...
    unsafe {
        deallocate(chunk);
    }
}
```

## Contributing
Contributions are welcome! If you find any issues or have suggestions for improvement, feel free to open an issue or submit a pull request.

## License
This project is licensed under the MIT License. [Click here](./LICENSE) to view license.