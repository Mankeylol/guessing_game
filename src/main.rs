use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;


fn main() {
   
    let number = rand::thread_rng().gen_range(1..101);
   
    println!("the number is: {}", number);

    loop {
        println!("input your guess. ");
        let mut guess = String::new();
        io::stdin()
                .read_line( &mut guess)
                .expect("failed to read line");

    let guess: u32 = match guess.trim().parse(){
        Ok( num) => num,
        Err(_) => continue,
    };

    println!("you guessed: {}", guess);
    

    match guess.cmp(&number){
        Ordering::Equal => {
            println!("{}","you win".green());
            break;},
        Ordering::Greater => println!("{}","too big".red()) ,
        Ordering::Less => println!("{}","too small".red())
    }
    
    }
    
}
