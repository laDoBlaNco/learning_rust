use rand::Rng;
use std::io; // so this is like c's #include <stdio.h> // the Rng trait defines methods that random number
             // generators implement.
use std::cmp::Ordering;

fn main() {
    println!("Guess a number!");

    // here we are using our dependency rand crate that we added to
    // the .toml file and did a cargo build to get it downloaded
    // along with other crates that we needed. Then with the use statement
    // above we have access to the Rng function. Interestingly the book
    // gave me deprecated code, but the hints from the editor helped me
    // fix it. I'm assing the syntax ..= is inclusive range. I've seen
    // something similar Odin.
    let secret_number = rand::rng().random_range(1..=100); // as I thought
                                                           // start..=end is inclusive of the lower and upper bounds. so assuming
                                                           // start..end would be exclusive of the top bound. How do I know what
                                                           // methods are on the trait? Each crate comes with documentation
                                                           // So with cargo doc --open I get all the documentation for my
                                                           // dependencies and can click on the one I want and read away.

    // println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");

        // in rust vars are immuttable by default so we need to make it
        // mut if we are going to use fill it with something later.
        // so here we are creating a mutable var that is currently bound
        // to a new, empty instance of String
        let mut guess = String::new();

        // so this :: syntax is to use associated functions for types
        // I guess it like a method. In Go methods are called assoc functions
        // as well.
        io::stdin()
            // ok so :: is associated functions and the normal (.) is for methods
            // What we are saying is here take the users input and store it in our
            // guess variable. The & indicates that the arg is a reference, which
            // gives me a way to let multiple parts of the code access one piece
            // of data without needing to copy that data into memory multiple
            // times. References is a complex feature and of the major advantages
            // is how safe and easy it is to use refs. like variables refs are
            // immutable by default, which is why I have to write &mut guess
            // rather than just &guess
            .read_line(&mut guess)
            // this is still the same line of code, not the semi-colon here
            // and not above. I could've written it all together but its
            // easier to ready when we break up long lines. read_line returns
            // a Result which is an enum. It returns either Ok or Err
            // Values of Result type have methods defined on them. An instance
            // Result has an expect method, so this is normal chaining here
            // really. We are just calling the expect method from the result
            // of the read_line method. If the read_line returns an err its
            // probably from the stdin on the underlying system. If this instance
            // of Result is OK, expect will take the return value that Ok is
            // holding and return just that value to me to use. I we don't use
            // expect the compiler will work but with a warning.
            .expect("Failed to read line");

        // Rust allows us a shadow the var guess to we can just replace
        // the old with the new and reuse the var name. This feature is often
        // used when we convert from one type to another.
        // .trim() does as expected and then .parse() is what does the conversion

        // we switched from our .expect on Result to a match statement to act
        // directly on the results of Ok and Err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // here we have the println macro using interpolation with {}
        // pretty simple stuff.
        println!("You guessed this: {}", guess);

        match guess.cmp(&secret_number) {
            // Ordering is another enum and has variants Less, Greater, and Equal
            // the .cmp method compares two values and can be called on anything
            // that can be compared. It takes a reference to whatever I want
            // to compare with. Then it is what returns the enum Ordering
            // I then set the match branches to the variants of Ordering
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
