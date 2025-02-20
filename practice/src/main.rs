// //Problem 1
// fn concat_strings(s1: &String, s2: &String) -> String {
//     let mut result = String::new();
//     result.push_str(s1);
//     result.push_str(s2);
//     return result 
// }

// fn main() {
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("World!");
//     let result = concat_strings(&s1, &s2);
//     println!("{}", result); // Should print: "Hello, World!"
// }

// //Problem 2
// fn clone_and_modify(s: &String) -> String {
//     let mut cloned = s.clone();

//     cloned.push_str("World!");

//     cloned
// }

// fn main() {
//     let s = String::from("Hello, ");
//     let modified = clone_and_modify(&s);
//     println!("Original: {}", s); // Should print: "Original: Hello, "
//     println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
// }

//Problem 3
#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    
    let mut sum_value = 0;
    for i in low..=high {
        sum_value += i;
    }
    // Store the result in the variable pointed to by `total`
    *total = sum_value;
}

fn main() {
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Total sum from 0 to 100 is: {}", total);
}