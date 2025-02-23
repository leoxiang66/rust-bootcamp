---
date: dd.MM.YYYY
theme: ./tokyo_night.json
---

# Advanced features 
- Atomics/Memory Ordering
- Unsafe Rust
- Typestate Patterns
- Macros(skipped)

---

# Atomics

What happens if two threads access data at the '*same*' time?

```rust
// Initial state
let mut data = 0;
// Thread 1:
data = 1;
// Thread 2:
println!("{data}");


```
```
Thread 1     data    Thread 2
╭───────╮   ┌────┐   ╭───────╮
│  = 1  ├╌┐ │  0 │ ?╌┤  data │
╰───────╯ ├╌┼╌╌╌╌┤   ╰───────╯
          └╌┼╌╌╌╌┤
            │  1 │
            └────┘



```

**=> Undefined behavior!**

---
# Atomics
```rust
// Initial state
let data = AtomicU64::new(0);
// Thread 1:
data.store(1, atomic::Ordering::Relaxed);
// Thread 2:
data.load(atomic::Ordering::Relaxed);

```

```
     Possible Execution 1       ┃       Possible Execution 2
                                ┃
Thread 1     data    Thread 2   ┃  Thread 1     data    Thread 2
╭───────╮   ┌────┐   ╭───────╮  ┃  ╭───────╮   ┌────┐   ╭───────╮
│ store ├─┐ │  0 ├───┤  load │  ┃  │ store ├─┐ │  0 │ ┌─┤  load │
╰───────╯ │ └────┘   ╰───────╯  ┃  ╰───────╯ │ └────┘ │ ╰───────╯
          └─┬────┐              ┃            └─┬────┐ │
            │  1 │              ┃              │  1 ├─┘
            └────┘              ┃              └────┘

```

**=> Race condition!**


## RMW(Read-modify-write) operations are sufficient in many scenarios
```rust
static COUNTER: AtomicU64 = AtomicU64::new(0);
pub fn get_id() -> u64 {
    COUNTER.fetch_add(1, atomic::Ordering::Relaxed)
}
```


---
# Acquire/Release
```rust
// Initial state
let data = AtomicU32::new(0);
// Thread 1
data.store(1, atomic::Ordering::Release);
// Thread 2
data.load(atomic::Ordering::Acquire);
```

```
                              
Thread 1      a     Thread 2 
╭───────╮   ┌───┐   ╭──────╮
│ store ├─┐ │ 0 │ ┌─→ load │
╰───────╯ │ └───┘ │ ╰──────╯
          └─↘───┐ │         
            │ 1 ├─┘         
            └───┘           

```
- __Release__ is synchronized with __Aquire__
- __Release__ (and every opertion before that) happen before __Aquire__
---
# Ordering
```rust
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}
```
```
The compiler is allowed to re-order your instructions if a single thread would not notice the difference,
and it will do so to try to get better CPU utilization. 

If you have multiple threads and two pieces of shared data A and B,
and A must be visible by the time B changes, this might not be obvious to the compiler. 
The compiler can freely re-order A to happen after B.
```

**=> Acquire/Release, AcqRel, SeqCst have a performance overhead**

Some useful resources:
- https://research.swtch.com/hwmm
- https://sabrinajewson.org/rust-nomicon/atomics/multithread.html
- https://github.com/tokio-rs/loom

---

# Unsafe Rust

- Unsafe Rust is exactly like safe rust with all the same rules and semantics.
- It lets you do extra things that the compiler can not verify to be safe.

## Syntax

- The `unsafe` keyword acts as an interface between unsafe and save rust. 
- Functions and Traits can be marked as unsafe aswell
```rust
fn main() {
    unsafe{
        println!("Hello World");
    }
}
unsafe fn Foo() {

}
unsafe trait Bar {

}
unsafe impl Bar for i32 {

}
```





---
# Unsafe Rust
## Safe operations in rust
1. Deadlocks
2. Have a race condition
3. Leak memory
4. Overflow integers
5. Abort the program
6. Delete the production database

## Unsafe operations in rust
1. Dereference raw pointers
2. Call unsafe functions
3. Implement unsafe traits
4. Mutate statics
5. Access fields of unions

There is an asymetric relationship between safe and unsafe rust!
- **Safe rust has to trust unsafe rust**
    - There is no way safe rust can verify the correctness of unsafe rust
- **Unsafe rust cannot trust safe rust**
    - Unsafe rust has to account for potential logic errors in safe rust that can lead to undefined behavior in unsafe rust.
 

---

# Typestate Patterns
## What is a typestate pattern?
```
Typestate pattern is a design pattern that encodes information about an object’s run-time state in its compile-time type.
```
## What does that mean?
1. Operations on an object are only available when the object is in certain states
2. Encode states at a type level so attempts to use wrong operations will fail to compile

## Why do we want that?
1. Certain types of errors are moved from run-time to compile-time.
3. Eliminates run-time checks, making code faster/smaller.

### In rust these patterns are easy to implement because of its type system.
```
On the contrary it's very difficult to implement in most other programming languages
which is why this pattern is not widely spread.
```
The reason most programming languages cannot implement the Typestate pattern is missing **move** semantics
(For more information see Substructural type systems: https://without.boats/blog/ownership/)

