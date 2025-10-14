# Rust experiments

Repository to experiment with Rust as a second language for building CLI and projects as well as some libraries for Python with Pyo3.

---
Learning with the official book and the interactive one
checkpoint : https://rust-book.cs.brown.edu/ch03-03-how-functions-work.html

## Memos from the book

- By default variable are not mutable (eg: cannot be changed after initilization) however we can use the keyword 'mut' to make it mutable
- CONST are not mutable and cannot be prefix with the mut keywor
- Rust isn’t limited to low-level systems programming. It’s expressive and ergonomic enough to make CLI apps, web servers, and many other kinds of code quite pleasant to write
- Rust is a _statically typed_ language, which means that it must know the types of all variables at compile time
- We can declare twice or more the same value, the first variable is _shadowed_ by the second, which means that the second variable is what the compiler will see when you use the name of the variable
- `const` can be used in the global scope, and `let` can only be used in a function
- Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
	- An _integer_ is a number without a fractional component.
	- _Signed_ and _unsigned_ refer to whether it’s possible for the numberto be negative—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned).
	- _Compound types_ can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
	- Another way to have a collection of multiple values is with an _array_. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.
	- Arrays are useful when you want your data allocated on the stack
	- An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. You can access elements of an array using indexing


- When a variable is immutable, once a value is bound to a name, you can’t change that value (very important)
- The main function is special: it is always the first code that runs in every executable Rust program
- Cargo is Rust’s build system and package manager. Cargo handles bulding, compiling, testing
- cargo run is a shortcut for building and running the binary
- memo: downside of rust, compile time is very long in comparaison go is very fast, alsmot instant
- cargo check can be used to check if the code can compile, much faster than cargo build (does not procude execatutable)
- In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing
-
```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

- Compilers output and errors are very good, often stating exactly the problem and what to do to solve it
- 



## Rust as a Python programmer

### Miscs
- Null, type, error are check at compilte time whereas Python allow faster developement at the cost of weaker software in production
```py
user.name = None 
user.name # Raise Attribute error
# Rust will not compile if null is not handle (same as Dart)

username: str = "Gaëtan"
username = 3.14 # Valid Python, data in Py can be dynamicaly changed

# Rust will not compile and raise a type error
```

Thoughs : Python is much more faster and suitable for protoping, can be better for scripting, data etc and easier to refacto and iterate at the cost of fiability. Rust make it painful from the get go, once it compile the code is safe to ship.

### Tooling
Python have a much wider tooling ecosystem
