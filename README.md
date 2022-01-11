# Oxidizer

Ive been following the [rust book](https://doc.rust-lang.org/book/) tutorials.
This repo is for me to document everything I have been learning in Rust.


## Rust Learnings

### Immutability is the default
Once a value is bound to a name, it cannot be changed.
```rust
fn main(){
    let x = 5;
    println!("{}", x);
    x = 6; // This will cause a compiler error
}
```
When variables are mutable, they stand out since `mut` is in front of the declaration. Can convert to immutable without Copying.

### Zero Cost Abstraction
1. You dont pay for what you dont use.
2. You cant code the code you use any better.
No matter how many levels of abstraction you have, the compiler will optimize to the best implementation of the solution that someone would have written with the lower level primitives.

The point of this is to improve user's experience, by introducing abstraction you reduce the complexity of the code.

### No Null / Nil in Rust.
You do not get `NULLs` or `Nils` in rust by default. You can use `Option<T>` as return values. [Option](https://doc.rust-lang.org/std/option/enum.Option.html) is an enum, which contains 2 enums. 
- `None:` representing the `NULLs` or `Nils`. 
- `Some(T):` which represents the case where we have a value. You can call `unwrap` on a `Some(T)` to obtain the value its wrapping.
```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

let result: Option<f64> = divide(2.0, 3.0);

// Using pattern matching to retrive the value.
match result {
    Some(x) => println!("{}", x),
    None => println!("we did not get any results") 
}
```

### Error Handling
When you encouter a problem which isnt recoverable, just panic!
```rust
fn add(a: &i32, b: &i32){
    if a == 69 && b == 420 {
        panic!("The two legendary numbers cant be added!");
    }
}
```

For recoverable errors you can use the `Result<T>` enum which is availble by default.

### Ownership
All variables are assigned an owner. Ownership can be transfered. Variables can be borrowed. Either with read permissions or write permissions. See the demonstration below.


```rust
fn add(a: i32, b: i32)
```
We are transfering the ownership of those varibles from the parent function to the `add` function. This means, when we are done with the `add` function, the variables `a` and `b` will be destroyed.


```rust
fn add(a: &i32, b: &i32)
```
We are only lending the varibles with read permissions to the `add` function. The function will not be able to mutate the variables. The scope or lifetime of a and b will be tied to the parent function.


```rust
fn add(a: &mut i32, b: &mut i32)
```
We lending the varibles with mutable permissions to the `add` function. This is exclusive access to the `add` function, no other function will be able to mutate or read the values of `a` and `b` while `add` function is borrowing them.

Attempting to mutate a value while the variable has another owner results in compile time error.
A variable can have multiple owners. You can do this with `Rc<T>` the [Reference Counted Smart Pointer](https://doc.rust-lang.org/book/ch15-04-rc.html).

### There are a few Common pointer types
- [Box<T>](https://doc.rust-lang.org/std/boxed/struct.Box.html)
  - When you have a type whos size cant be known at compile time.
  - When you want to transfer ownership without copying (Imagine the data is massive).
  - When you want own a value that implements a specific trait but dont care about the type.
- [Rc<T>](https://doc.rust-lang.org/book/ch15-04-rc.html)
  - When you want to enable multiple ownership of a variable. Uses a count of all the owners to determine lifetime. When the count goes to 0, the memory will be released.
- [Ref<T>](https://doc.rust-lang.org/std/cell/struct.Ref.html) and [RefMut<T>](https://doc.rust-lang.org/std/cell/struct.RefMut.html) accessed through [RefCell<T>](https://doc.rust-lang.org/std/cell/struct.RefCell.html), Slightly tricky to work with. It enforces borrowing rules at runtime instead of compile time.
  - At any given time, you can have only one mutable reference and any number of immutable references to this.
  - References must always be valid.
  
As you can see rust allows memory leaks using `Rc<T>` and `RefCell<T>`. Its possible to create references where items refer to each other in a cycle. The reference count of each item will never reach 0. Therefore the memory will never be released.


### Macros
Macros are basically rust code that generates rust code. Rust dosent support reflection, but macros have access to the [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree).
Macros also support variable arguments!
```rust
println("{} + {} = {}", a, b, a + b);
```

### Traits
Traits are to rust what interfaces are to java, or abstract classes are to C++. It tells the compiler about functionality a particular type has and can share with other types. We can use traits to define a shared behaviour in an abstract way. We can use trait bounds to specify that a generic type can be any type that has a certain behaviour.

### Unit tests and Integration tests
The convention is to write the unit tests right below the code. Unit tests are enclosed in a `mod`
```rust
fn add(a: &i32, b: &i32){
    // Todo
}

#[cfg(test)] // attribute definition to let the compiler this is a test module.
mod test_calculations{
    #[test] // attribute definition to let test runner know this is a test function.
    fn test_add(){
        let result = add(10, 10);
        assert_eq!(result, 20); // assert macro to ensure the result is as expected.
    }
}
```

Integration tests are defined outside the `src` directory. Its stored in the `tests` directory. 
```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```
In the `integration_test.rs` you would need to import the `common` test module in order to use the common functionality that you want to share across other tests.
```rust
use test_adder;
mod common;

#[test]
fn it_adds_two(){
    common::setup();
    assert_eq!(4, test_adder::add_two(2))
}
```
