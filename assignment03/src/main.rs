fn check_guess(guess: i32, secret: i32) -> i32 {
    // Simple if else statememnt to check the guess
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let mut secret = 42;
    let mut attempts = 0;


    let mut guess = 42; 


    loop {
        attempts += 1;

        // check the function result
        let result = check_guess(guess, secret);

        // use logic to determine what to print
        if result == 0 {
            println!("Guess {guess} is correct!");

            break;
        } else if result == 1 {
            println!("Guess {guess} is too high!");
        } else {
            println!("Guess {guess} is too low!");
        }
        //This is where we would repeat the loop by asking for a new guess
    }

    println!("It took {attempts} attempts.");
}
