

fn get_largest_item<T: PartialOrd + Copy>(item_list: Vec<T>) -> T {
    let mut largest = item_list[0];
    for item in item_list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn generics_example_1(){
    let number_list = vec![1, 2, 3, 4, 5];
    let largest = get_largest_item(number_list);
    println!("largest number in the vector is: {}", largest);

    let char_list = vec!['a', 'b', 'c', 'd', 'e'];
    let largest_char = get_largest_item(char_list);
    println!("largest char in the vector is: {}", largest_char);
}

#[derive(Debug)]
struct Points<T, U> {
    x:T,
    y:U,
}

impl<T, U> Points<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<V, W>(self, other: Points<V, W>) -> Points<T, W> {
        Points { x: self.x, y: other.y }
    }
}

fn generic_example_2(){
    let p1 = Points{x: 5, y:10};
    let p2 = Points{x: 'a', y: 'b'};
    let p3 = Points{x: 2.33, y: 4.5};
    let p4 = Points{x: 'a', y: 45.666};

    println!("{:#?}", p1);
    println!("{:#?}", p2);
    println!("{:#?}", p3);
    println!("{:#?}", p4);
    println!("p1.x = {}", p1.x());
    println!("p2.y = {}", p1.x());
    println!("[+] mixup( p3, p4)");
    println!("{:#?}", p3.mixup(p4));
}
fn main() {
    generic_example_2();

}
