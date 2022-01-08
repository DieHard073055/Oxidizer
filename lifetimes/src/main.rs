use std::{ops::Add, process::Output};

fn longest<'a>(x: &'a String, y: &'a String) -> &'a String{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let mut result = String::from("");
    {
        let string1 = String::from("abcd");
        let string2 = String::from("xyz");

        result = longest(&string1, &string2).to_string();
    }
    println!("The longest string is {}", result);
}


/*
Rust ownership

(a: T, b: T)
you now own a and b. You are responsible for free-ing it.

(a: &mut T, b: &mut T)
you have a mutable reference to a and b. You have exclusive access to it
no one else can change this. But you are not responsible for free-ing it.

(a: &T, b: &T)
you have an immutable reference to a and b, you are not allowed to modify a and b.
because a and b is shared with others. They are referencing it as well.
*/

fn gt<T: PartialOrd>(a: T, b: T) -> T{
    if a > b {
        a
    } else {
        b
    }
}
