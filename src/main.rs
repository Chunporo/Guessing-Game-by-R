use std::io;

fn main() {
    println!("Gues the number");
    println!("Please input your guess.");
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line"); //Handling Potenial Failure with the Result 

    println!("You guessed: {}", guess);
}
