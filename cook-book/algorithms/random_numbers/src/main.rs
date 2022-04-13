use rand::Rng;
use rand_distr::{Distribution, Uniform, Normal, Standard};
use rand::distributions::Alphanumeric;

fn generate_random_numbers(){
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random f64: {}", rng.gen::<f64>());

}

fn generate_random_numbers_within_a_rang(){
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
}

fn generate_uniform_distribution_of_random_numbers(){
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

fn generate_random_numbers_with_a_given_distribution(){
    let mut rng = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0).unwrap();
    let v = normal.sample(&mut rng);

    println!("N(2, 9) distribution: {}", v);
}

// generate random values for custom type

#[allow(dead_code)]
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point{
            x: rand_x,
            y: rand_y,
        }


    }
}

fn generate_random_numbers_for_custom_type(){
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);

}

fn generate_random_password(){
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("{}", rand_string);
}

fn generate_random_password_from_a_set_of_chars(){
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789!@#$%^&*()~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    println!("{}", password);
}

fn main() {
    println!("\ngenerate random numbers");
    generate_random_numbers();
    println!("\ngenerate random numbers within a range");
    generate_random_numbers_within_a_rang();
    println!("\ngenerate uniform distribution of random numbers");
    generate_uniform_distribution_of_random_numbers();
    println!("\ngenerate random numbers with a given distribution");
    generate_random_numbers_with_a_given_distribution();
    println!("\ngenerate random numbers for custom types.");
    generate_random_numbers_for_custom_type();
    println!("\ngenerate random password");
    generate_random_password();
    println!("\ngenerate random password from a set of defined chars");
    generate_random_password_from_a_set_of_chars();
}
