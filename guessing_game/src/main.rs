use std::io;    //acting like a namespace operator, 
                // otherwise can use std::io::stdin directly without this import
                // in this case, its probably because the std libs are included by default? 

// The Rng trait defines methods that random number generators implement, 
// and this trait must be in scope for us to use those methods.
use rand::Rng; // called a trait? 


use std::cmp::Ordering;


fn main() {
    println!("Guess that number!");

    // one that is local to the current thread of execution and is seeded by the operating system
    // gen_range is inclusive on the lower and upper bounds.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret number = {secret_number}");

    // infinite loop
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        // equivalent to below
        // io::stdin().read_line(&mut guess).expect("failed to read line");

        io::stdin()
            // references are immutable by default, so you need to add mut.
            .read_line(&mut guess)

            // readline returns a result
            // result is a enum and each state of an enum is a variant.

            // in this case, will cause program to crash if the result is an Err value insted of Ok
            // if expect gets "Ok" then it will return the value from stdin.
            .expect("failed to read line"); 

        // shadowing is a common thing in rust when referring to same variable by just different types
        // figures out which one it needs based on type
        // parse method on strings converts the variable to another type.
        // infers the type from the variable its assigning to
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // if ok, pass the number through to guess. doesnt have to be called "num" this is moreso an argument
            Err(_) => continue, // otherwise continue if error
                                // _ is a catchall. match all err valus no matter what information is inside them
        };


        println!("You guessed: {guess}");

        // placeholds with println
        // let x = 5;
        // let y = 10;
        // println!("x = {x} and y + 2 = {}", y + 2);

        match guess.cmp(&secret_number) {
            // ordering has these variants defined that are returned when you call cmp
            // these arent called cases but "arms"
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("Equal u did it!");
                break;
            }
        }
    }
}
