# 01 Rust Portrait

## Getting started

1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2. Add rust-analyzer extension to your IDE (optional but really helpful)

3. Creating new project: `cargo new <project name>`

4. Code something cool

5. Run project from `main.rs`: `cargo run`


## Seeking help

Rust book is a great resource: [The Book](https://doc.rust-lang.org/book/), it can be also accessed offline by running `rustup docs --book`.

There is also Rust presented in code snippets: [Rust by Example](https://doc.rust-lang.org/rust-by-example/). 

If you want to practice take look at rustlings. Project contains small exercises to get you used to reading and writing Rust code. Can be installed by `cargo install rustlings`

## Exercise

1. Construct unbalanced BST from vector of integers

   - you may assume values are unique

   - Hint: use `Option<Box<Node>>` for references. Box is used to allocate data on the heap rather than the stack. If your tree is recursive the size is not known at compile time.

2. Add search function
3. Add order traversal
4. Add function to calculate height of the tree

If you up for more:
- add delete function, generics, tree iterator ...