-----------------------
# Rust Learning Notes

## Introduction

Repository to experiment with Rust as a second language for building CLI projects and libraries for Python with Pyo3.

Notes are written as is.

**Learning resources:** Official Rust Book and interactive version  
**Checkpoint:** https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html 

----------


### Mutability

In Rust unlike many dynamic programming language such as Python/Ruby variables are immutable by default, however using the special keyword 'mut' make the variable mutable.
Exception for the CONST keyword that like JS an other langage cannot be mutate and prefix with the mut keyword

### Example: Mutable vs Immutable

```rust
let apples = 5;          // immutable
let mut bananas = 5;     // mutable

```

### Type System

Rust is staticaly typed, all variable types must be known for the compiler or it will fail. Each value in Rust has a type, including but not limited to primitives (bool, numeric, text) sequence (tuple, array, slice) user defined (struct, enum..) and much more.
Rust encouraghe strong typing. For example if we are working to build a user management system, a Struct User would be useful. Struct is very similiar in the impletentation to creating a datatype in Python (it seems). 

### Ownership

One reason for Rust efficianty is that it does very few checks at run time, instead Rust check are compile-tile. Rust is very strict at detecting undefined behavior, whioch is the cause of 70% of reported vuln in low leve system.




Mutability
Variables are immutable by default and cannot be changed after initialization
Use the mut keyword to make a variable mutable
const is always immutable and cannot be prefixed with mut
const can be declared in global scope, while let can only be used inside functions
Example: Mutable vs Immutable
let apples = 5;          // immutable
let mut bananas = 5;     // mutable

Shadowing
A variable can be declared multiple times with the same name
The new declaration creates a new variable that “shadows” the previous one
This is different from mutation—a new variable is actually created with the same name
Type System
Rust is a statically typed language: all variable types must be known at compile time (unlike Python where wrong type will raise an error in production)
The compiler almost never needs explicit type annotations elsewhere in the code because function signatures require them
Ownership
Rust’s goal is to compile programs into efficient binaries that require as few runtime checks as possible. Therefore Rust does not check at runtime whether a variable is defined before being used. Instead, Rust checks at compile-time

About 70% of reported security vulnerabilities in low-level systems are caused by memory corruption, which is one form of undefined behavior.

A foundational goal of Rust is to ensure that your programs never have undefined behavior

Box deallocation principle (almost correct): If a variable is bound to a box, when Rust deallocates the variable’s frame, then Rust deallocates the box’s heap memory.

Box deallocation principle (fully correct): If a variable owns a box, when Rust deallocates the variable’s frame, then Rust deallocates the box’s heap memory.

Ownership is primarily a discipline of heap management

All heap data must be owned by exactly one variable.

Rust deallocates heap data once its owner goes out of scope.

Ownership can be transferred by moves, which happen on assignments and function calls.

Heap data can only be accessed through its current owner, not a previous owner.

The core idea behind the borrow checker is that variables have three kinds of permissions on their data:

Read ®: data can be copied to another location.

Write (W): data can be mutated.

Own (O): data can be moved or dropped.

These permissions don’t exist at runtime, only within the compiler. They describe how the compiler “thinks” about your program before the program is executed.

Scalar Types
Rust has four primary scalar types:

Integers: numbers without fractional components

Signed: can be negative (need a sign)
Unsigned: only positive (no sign needed)
Floating-point numbers: numbers with decimals

Booleans: true or false

Characters: single Unicode characters

Compound Types
Collections of multiple values into one type:

Tuples: group values of different types together, fixed length
Arrays: group values of the same type, fixed length, allocated on the stack
Arrays are useful when you want data allocated on the stack
Efficient for known, fixed-size collections
Access elements using indexing
Rust protects against invalid indices by exiting immediately (memory safety)
Defining Functions
Use the fn keyword to define functions
Rust uses snake_case for function names
All parameters must have declared types (deliberate design choice for better error messages and safety)
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

Statements vs Expressions
Statements: instructions that perform an action and do not return a value
Expressions: evaluate to a resultant value that can be used
Rust is an expression-based language
Important: In Rust, let y = 6 does not return a value (unlike C, Ruby, or JavaScript). You cannot write x = y = 6 in Rust.

Return Values
Functions can return values to the calling code
Declare the return type using an arrow (->) in the function signature
The return value is the value of the final expression in the function body (no need for return keyword, though it can be used explicitly)
fn add_one(x: i32) -> i32 {
    x + 1  // returns x + 1 (no semicolon!)
}

if Expressions
if is an expression, not just a statement
The condition must be a boolean; Rust has no concept of “truthy” or “falsy” values
Can be used on the right side of a let statement
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    println!("The value of number is: {number}");
}

Loops
loop - Infinite Loop
let mut count = 0;
loop {
    println!("again!");
    count += 1;
    if count == 5 {
        break;  // exit the loop
    }
}

Use break to exit the loop explicitly
Use continue to skip to the next iteration
while - Conditional Loop
let mut number = 3;
while number != 0 {
    println!("{number}!");
    number -= 1;
}

Evaluates a condition; runs while it’s true
for - Iteration Over Collections
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {element}");
}

Safer than index-based loops (no risk of going beyond array bounds)
Machine code can be more efficient (no index comparison at each iteration)
Ownership (Checkpoint)
Ownership is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector.

Tooling
Cargo
Rust’s build system and package manager that handles:

Building
Compiling
Testing
Useful commands:

cargo build: compiles and produces an executable
cargo run: builds and runs in one step
cargo check: checks if code can compile without producing an executable (much faster than cargo build)
Compiler Quality
Rust’s compiler output and error messages are excellent
Error messages typically state exactly what the problem is and how to fix it
This makes debugging and learning easier
Trade-off Note
One downside of Rust compared to Go: compile time is very long (Go is almost instant).

Rust vs Python: A Programmer’s Perspective
Philosophy Differences
Aspect

Rust

Python

Null handling

Checked at compile time (won’t compile if not handled)

Runtime errors (AttributeError)

Type safety

Static typing enforced at compile time

Dynamic typing (can reassign different types)

Error checking

Compile-time guarantees

Runtime failures

Development speed

Slower (painful to get right initially)

Faster (quick prototyping)

Production reliability

Once it compiles, code is safe to ship

Weaker guarantees in production

Example: Null Handling
# Python - fails at runtime
user.name = None 
user.name  # Raises AttributeError

// Rust - won't compile if not handled
// (Type system prevents this at compile time)

Example: Type Flexibility
# Python - valid, but risky
username: str = "Gaëtan"
username = 3.14  # No error, data type changed dynamically

// Rust - compilation error
username: str = "Gaëtan";
username = 3.14;  // Type error - won't compile

Summary
Python: Better for prototyping, scripting, and data work. Faster iteration and refactoring, but weaker reliability
Rust: Makes development painful initially, but once code compiles, it’s safe to ship. Excellent for systems programming and production reliability
Rust tooling: Wider ecosystem in Python, but Rust tooling is robust and growing
