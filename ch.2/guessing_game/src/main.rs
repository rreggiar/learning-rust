use std::io;
// to obtain user input and print output, we need io lib in scope
// comes from standard lib: 'std'
use rand::Rng;
// load in Rng trait from rand crate
use std::cmp::Ordering;
// another use statement with possible variants less, greater, equal

fn main() {
// entry point to program
// fn declares new function
// empty () would hold params
    println!("Guess the number!");

    let secret_number = 
        rand::thread_rng().gen_range(1,101);
    // threa_rng gives us particular rng
    // gen_Range method defined by Rng brought into scope above

    println!("Secret Number: {}", secret_number);
    loop {
        println!("Please input guess:");

        let mut guess = String::new();
        // let creates a variable
        // creating var guess bound to value new string
        // mut declares variable to be mutable
        // which is not default behavior
        // :: behaves just like R, calls assoc. fxn

        io::stdin().read_line(&mut guess)
            .expect("failed to read");
        // calling stdin fxn from io
        // then calls read_line method to get input
        // passing guess variable as arg
        // .expect handles potential failure
        // match moves from crashing on error to handling the error
        // Result type is enumeration
        // fixed value variants: Ok, Err

        // let guess: u32 = guess.trim().parse()
        //    .expect("Please type a number :)");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // shadow prev. variable guess
        // bind this to guess.tirm.parse
        // original value.eliminate whitespace.parses str into some num type
        // expect, if parse fails (no number present) ask again

        println!("you guessed: {}", guess);
        // {} is println placeholder
    
        match guess.cmp(&secret_number){
        // cmp method compares two values and can be called on anything comparable
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Correct!");
                break;
                // print line and break the loop{}
            }
        }
    }

}

