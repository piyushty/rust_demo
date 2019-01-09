use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

fn fahrenheit_to_celcius(frhn: f32) -> f32 {
    (frhn - 32.0) * 5.0 / 9.0
}

fn fibonacci(no_of_terms: u32) {
    let mut x = 1;
    let mut y = 1;
    println!("{}", x);
    println!("{}", y);
    for _counter in (0..no_of_terms).rev() {
        let temp = x;
        x = x + y;
        println!("{}", x);
        y = temp;
    }
}
