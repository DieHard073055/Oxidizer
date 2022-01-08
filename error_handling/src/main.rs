use std::fs::{self, File};
use std::io;
use std::io::{Read, ErrorKind};
use std::error::Error;

fn a() {
    b();
}

fn b(){
    c(21)
}

fn c(num: i32){
    if num == 22 {
        panic!("crash and burn! you passed in 22!");
    }
}

/*
Result Enum

T: Generic Type
E: Generic Error

enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

fn error_handling_example(){
    // handling errors opening a file
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            match error.kind(){
                ErrorKind::NotFound => match File::create("hello.txt"){
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            }
        },
    };
    // how to do the above with closures
    let f = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("problem opening the file: {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}
fn error_handling_example_2(){

}
fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("hello.txt")?;
    Ok(())
}