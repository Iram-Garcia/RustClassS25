fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // 1. Create an array of 10 numbers
    let numbers = [1, 2, 3, 5, 10, 15, 16, 18, 20, 30];


    for &num in &numbers {
        // check if even or odd
        if is_even(num) {
            println!("Even");
        } else {
            println!("Odd");
        }
        //fizzbuzz
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        }
    }

    //a while loop to find and print the sum of all numbers in the array
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("\nSum of all numbers: {sum}");

    //find and print the largest number
    let mut largest = numbers[0];
    for &num in &numbers {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {largest}");
}
