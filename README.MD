
# Rust Learning Notes

## Introduction

Repository to experiment with Rust as a second language for building CLI projects and libraries for Python with Pyo3.

**Learning resources:** Official Rust Book and interactive version  
**Checkpoint:** https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html 

----------


### Mutability

-   Variables are **immutable by default** and cannot be changed after initialization
-   Use the `mut` keyword to make a variable mutable
-   `const` is always immutable and cannot be prefixed with `mut`
-   `const` can be declared in global scope, while `let` can only be used inside functions

### Example: Mutable vs Immutable

```rust
let apples = 5;          // immutable
let mut bananas = 5;     // mutable

```

### Shadowing

-   A variable can be declared multiple times with the same name
-   The new declaration creates a new variable that "shadows" the previous one
-   This is different from mutation—a new variable is actually created with the same name

### Type System

-   Rust is a **statically typed** language: all variable types must be known at compile time
-   The compiler almost never needs explicit type annotations elsewhere in the code because function signatures require them

----------



### Scalar Types

Rust has four primary scalar types:

1.  **Integers**: numbers without fractional components
    
    -   **Signed**: can be negative (need a sign)
    -   **Unsigned**: only positive (no sign needed)
2.  **Floating-point numbers**: numbers with decimals
    
3.  **Booleans**: `true` or `false`
    
4.  **Characters**: single Unicode characters
    

### Compound Types

Collections of multiple values into one type:

1.  **Tuples**: group values of different types together, fixed length
2.  **Arrays**: group values of the **same type**, fixed length, allocated on the stack
    -   Arrays are useful when you want data allocated on the stack
    -   Efficient for known, fixed-size collections
    -   Access elements using indexing
    -   Rust protects against invalid indices by exiting immediately (memory safety)

----------


### Defining Functions

-   Use the `fn` keyword to define functions
-   Rust uses **snake_case** for function names
-   **All parameters must have declared types** (deliberate design choice for better error messages and safety)

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

```

### Statements vs Expressions

-   **Statements**: instructions that perform an action and do **not** return a value
-   **Expressions**: evaluate to a resultant value that can be used
-   Rust is an **expression-based language**

**Important:** In Rust, `let y = 6` does **not** return a value (unlike C, Ruby, or JavaScript). You cannot write `x = y = 6` in Rust.

### Return Values

-   Functions can return values to the calling code
-   Declare the return type using an arrow (`->`) in the function signature
-   **The return value is the value of the final expression in the function body** (no need for `return` keyword, though it can be used explicitly)

```rust
fn add_one(x: i32) -> i32 {
    x + 1  // returns x + 1 (no semicolon!)
}

```

### if Expressions

-   `if` is an **expression**, not just a statement
-   The condition **must** be a boolean; Rust has no concept of "truthy" or "falsy" values
-   Can be used on the right side of a `let` statement

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    println!("The value of number is: {number}");
}

```

### Loops

#### `loop` - Infinite Loop

```rust
let mut count = 0;
loop {
    println!("again!");
    count += 1;
    if count == 5 {
        break;  // exit the loop
    }
}

```

-   Use `break` to exit the loop explicitly
-   Use `continue` to skip to the next iteration

#### `while` - Conditional Loop

```rust
let mut number = 3;
while number != 0 {
    println!("{number}!");
    number -= 1;
}

```

-   Evaluates a condition; runs while it's `true`

#### `for` - Iteration Over Collections

```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {element}");
}

```

-   Safer than index-based loops (no risk of going beyond array bounds)
-   Machine code can be more efficient (no index comparison at each iteration)

----------

### Ownership (Checkpoint)

**Ownership is Rust's most unique feature** and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector.

----------

## Tooling

### Cargo

Rust's build system and package manager that handles:

-   Building
-   Compiling
-   Testing

**Useful commands:**

-   `cargo build`: compiles and produces an executable
-   `cargo run`: builds and runs in one step
-   `cargo check`: checks if code can compile without producing an executable (much faster than `cargo build`)

### Compiler Quality

-   Rust's compiler output and error messages are **excellent**
-   Error messages typically state exactly what the problem is and how to fix it
-   This makes debugging and learning easier

### Trade-off Note

One downside of Rust compared to Go: **compile time is very long** (Go is almost instant).

----------

## Rust vs Python: A Programmer's Perspective

### Philosophy Differences

Aspect

Rust

Python

**Null handling**

Checked at compile time (won't compile if not handled)

Runtime errors (`AttributeError`)

**Type safety**

Static typing enforced at compile time

Dynamic typing (can reassign different types)

**Error checking**

Compile-time guarantees

Runtime failures

**Development speed**

Slower (painful to get right initially)

Faster (quick prototyping)

**Production reliability**

Once it compiles, code is safe to ship

Weaker guarantees in production

### Example: Null Handling

```python
# Python - fails at runtime
user.name = None 
user.name  # Raises AttributeError

```

```rust
// Rust - won't compile if not handled
// (Type system prevents this at compile time)

```

### Example: Type Flexibility

```python
# Python - valid, but risky
username: str = "Gaëtan"
username = 3.14  # No error, data type changed dynamically

```

```rust
// Rust - compilation error
username: str = "Gaëtan";
username = 3.14;  // Type error - won't compile

```

### Summary

-   **Python**: Better for prototyping, scripting, and data work. Faster iteration and refactoring, but weaker reliability
-   **Rust**: Makes development painful initially, but once code compiles, it's safe to ship. Excellent for systems programming and production reliability
-   **Rust tooling**: Wider ecosystem in Python, but Rust tooling is robust and growing

----------


