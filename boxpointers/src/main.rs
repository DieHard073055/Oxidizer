/*
1 - When you have a type whos exact size,
cannot be known at compile time, and you
want to use a value of that type in a
context which requires you knowing the
exact type.

2 - When you have a large amount of data,
and you want to make sure the data isnt
copied.

3 - When you own a value, and you want to
make sure the value owns a specific trait.
rather then it being a specific type.
*/

// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use List::{Cons, Nil};

// fn cons_list_example(){

//     let list = Box::new(
//         Cons(1, Box::new(
//             Cons(2, Box::new(
//                 Cons(3, Box::new(
//                     Nil
//                 ))
//             ))
//         ))
//     );

//     println!("{:?}", list);
// }

// use std::ops::Deref;

// struct MyBox<T>(T);
// impl<T> MyBox<T> {
//     fn new(x:T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// fn say_hello(name: &str){
//     println!("hello {}", name);
// }

// fn deref_trait(){
//     let x = 5;
//     let y = MyBox::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *(y.deref()));

//     let m = MyBox::new(String::from("Rust"));

//     // &MyBox<String> -> &String -> &str
//     say_hello(&m);

//     // &MyBox<String> -> &String -> &str
//     say_hello(&(*m)[..])

// }

// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self){
//         println!("Dropping the CustomSmartPointer with data `{}`!", self.data);
//     }
// }

// fn custom_smart_pointer(){
//     let c = CustomSmartPointer {
//         data: String::from("my stuff"),
//     };
//     let d = CustomSmartPointer {
//         data: String::from("Other stuff"),
//     };
//     drop(c);
//     println!("CustomSmartPointers created.");
// }

use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn reference_counting() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b: {}", Rc::strong_count(&a));

    {
        let c = Cons(4, a.clone());
        println!("count after creating c: {}", Rc::strong_count(&a));
    }
    println!("count outside the scope of c: {}", Rc::strong_count(&a));
}
fn main() {
    // cons_list_example();
    // deref_trait();
    // custom_smart_pointer();
    reference_counting();
}
