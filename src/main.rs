extern crate rand; // this will let rust now we will use rand as external dependency
                   // This library comes from standard library called std
use std::cmp::Ordering; // Greater, Equal. These are possible outcome when comparing two values

use std::io; // this is input output library imported to our scope // Ordering is another enum like Result with three variants on it Less,
                                                                   
use rand::Rng;

fn main() {
    println!("Guess the nubmer!");
    println!("The secren number is generated using rand crate");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let other_number = rand::random::<u8>();
    println!("This is not your secret_number ;), {}", other_number);
    loop {
        println!("Please input your guess");

        // all variables in rust are immutable by defautl, hence you need to define the variable
        // as a mutable by using a 'mut' in front of the name
        let mut guess = String::new(); // String::new() this syntax indicate that new is an associated function of
                                       // the type String. Associate functions are on types in this
                                       // case String. Some language call this static method
                                       // let used to create a variable
                                       // Sum up: this line creates new mutable variable that is
                                       // currently bound to a new empty instance of a String

        io::stdin()
            .read_line(&mut guess) // Now we're calling associated function on io 'stdin'
            .expect("Failed to read line"); // The stdin function returns an instace of std::io:Stdin
                                            // which  is a type that represents a handle to the standard
                                            // input for your terminal
                                            // The .read_line calls the read_line method on the standard
                                            // input handle to get input from the user. We're passing one
                                            // argument, variable guess to it. The & indicate that this
                                            // argument is the reference which gives you a way to let
                                            // multiple parts of your code access one piece of data
                                            // without needing to copy that data into memory multiple
                                            // times.
                                            //
                                            //
                                            // What is .expect .expect("Failed to read line");
                                            // read_line put the user typed value in the string, but it
                                            // does one more thign. Returns a value in this case
                                            // io::Result. The Result is an enum and have to variants in
                                            // it Ok and Err. Ok indicates the operation was sucessful
                                            // and inside of it is stored value.
                                            // The Err means operation failed and contains information on
                                            // why it did fail

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { // _ is a catchcall value, we use match that must equal the type 'Ok(num)', 
                // otherwise there's Error hadnled by us by simply printing the message when it 
                // occurs and continue execution of the program until the value matches the type
                println!("Numbers only");
                continue;
            },
        };
            // guess: u32 This is called shadowing in Rust
            // This feature is used when you want to change the type of the variable
            // we need to trim the guess input because the moment user press enter this is added
            // to the variable guess, meaning if user put 6 enter. the value of guess = 6\n
            // the parse method on the string parse the string into some kind of number. We need to
            // tell rust what type we want, in this case is u32
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            // cmp mehtod is used to comapre values and can be called on
            // anything
            // math will look at the result of the comparison from cmp and execute one of the below
            // Ordering methods if it finds the match. For examlpe if secret_number is 25 and user
            // guess 50 the returned value from cmp will be Ordering::Greater, match will look at the
            // body of the function and execute the method that matches ! meaning Ordering::Greater ...
            // Too Big in this case
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => { 
                println!("Well done that was the number");
                break;
            }
        }
    }
}
