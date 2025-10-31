use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!"); // println! is a macro and not a function

    let secret_number = rand::thread_rng().gen_range(1..=100);
   
    loop{
    println!("Please input your guess. Type 'quit' to quit the game!!");

    // let is used to create variables, by default a variable is immutable
    // so once we define it and put a value in it, it cannot change
    // similar to const in other languages
    // mut is added to make it mutable, similar to normal variable in other languages
    let mut guess = String::new(); // mut is for mutable,
    
    io::stdin()
	.read_line(&mut guess)
	.expect("Failed to read line");

    let guess: u32 = match guess.trim().parse(){
    	Ok(num)=> num,
	Err(_) => continue, 
    };
    println!("You guessed: {guess}");
    match guess.cmp(&secret_number){
    	  Ordering::Less => println!("Too small!!"),
	  Ordering::Greater => println!("Too big!!"),
	  Ordering::Equal =>{
	  		  println!("You win!!");
			  break;
			  },
    }
    }
}

