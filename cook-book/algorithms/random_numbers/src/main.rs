use rand::Rng;
use rand_distr::{Distribution, Uniform, Normal};

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

fn main() {
    println!("\ngenerate random numbers");
    generate_random_numbers();
    println!("\ngenerate random numbers within a range");
    generate_random_numbers_within_a_rang();
    println!("\ngenerate uniform distribution of random numbers");
    generate_uniform_distribution_of_random_numbers();
    println!("\ngenerate random numbers with a given distribution");
    generate_random_numbers_with_a_given_distribution();
}
