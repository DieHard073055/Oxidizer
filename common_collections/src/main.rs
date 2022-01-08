use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

/*
    You cannot have a mutable reference and an immutable reference
    to the same instance variable.
*/
fn vectors_1() {
    let a = [1, 2, 3];
    println!("array a {:#?}", a);

    // declare an empty vector and add numbers
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // declare a vector with the pre-defined numbers
    let v2 = vec![1, 2, 3];
}

fn vectors_2() {
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(20){
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
}

fn vectors_3(){
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i *= 50;
    }
    for i in &v {
        print!("{}, ", i);
    }
    println!();
}

fn vectors_4(){
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("not an Integer"),
    }
}

fn strings_example(){

    // Strings are stored as a collection of UTF-8 encoded bytes
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    // How to add strings and characters to string
    let mut s = String::from("hello");
    s.push(' ');
    s.push_str("world");
    s.push_str(&String::from(" again!"));
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 is borrowed, and therefore cannot be used
    // after the following statement.
    let s3: String = s1 + &s2;
    println!("{}", s3);

    // Another way of doing the above
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = format!("{}{}", s1, s2);
    // the format macro does not take ownership  of the s1 or s2
    // So we can still use s1, s2, and s3 afterwards
    println!("{}|{}|{}", s1, s2, s3);

    let hello: String = String::from("hello");
    // print the bytes
    for b in hello.bytes() {
        println!("{}", b);
    }
    // print the chars
    for b in hello.chars() {
        println!("{}", b);
    }
    // print the grapheme clusters
    for g in hello.graphemes(true) {
        println!("{}", g);
    }
}

fn hashmaps_example(){
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    println!("{:#?}", scores);

    let team_name = String::from("blue");
    let score = scores.get(&team_name);
    println!("{}'s score is {}", team_name, score.unwrap_or(&0));

    // iterate over the elements in the hashmap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // update the values in the Hashmap
    
    // the following line will replace existing value
    scores.insert(String::from("blue"), 11); 
    // the following line will update the value for yellow
    // if there is no existing value
    scores.entry(String::from("yellow")).or_insert(40);

    // add a list of words to a hash map, and mutate the values
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // we are able to obtain a mutable reference to the value below
        let count = map.entry(word).or_insert(0);
        // we can de-reference the value and mutate it.
        *count += 1;
    }

    println!("{:?}", map);

}
fn main() {
    hashmaps_example();
}

