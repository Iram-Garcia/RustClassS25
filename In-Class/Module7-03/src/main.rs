// //Task 1
// fn main() {
//     let operation = |a: i32, b: i32| {
//         a * b
//     };
//
//     println!("Result: {}", operation(10, 5));
// }

//Task 2
// fn track_changes() {
//     let mut tracker = 0;
//     let mut update = || {
//         tracker += 1;
//         println!("Tracker: {}", tracker);
//     };
//
//     update();
//     update();
// }
//
// fn main() {
//     track_changes();
// }

//Task 3
//Map function
// fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
// where
//     F: Fn(i32) -> i32,
// {
//     vec.into_iter().map(f).collect()
// }
//Loop version
// fn process_vector_with_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
// where
//     F: Fn(i32) -> i32,
// {
//     let mut result = Vec::new();
//     for x in vec {
//         result.push(f(x)); // Apply the closure
//     }
//     result
// }
//
// fn main() {
//     let numbers = vec![1, 2, 3];
//
//     let doubled = process_vector(numbers.clone(), |x| {
//         // Implement: multiply each number by 2
//         x * 2
//     });
//
//     let replaced = process_vector(numbers, |x| {
//         // Implement: if number > 2, replace with 0, else keep number
//         if x > 2 {
//             0
//         } else {
//             x
//         }
//     });
//     println!("Doubled: {:?}", doubled);
//     println!("Replaced: {:?}", replaced);
// }

//Task 5 (NO TASK 4, error on instructions)
// use std::{thread, time::Duration};
//
// struct ComputeCache<T>
// where
//     T: Fn() -> String,
// {
//     computation: T,
//     result: Option<String>,
// }
//
// impl<T> ComputeCache<T>
// where
//     T: Fn() -> String,
// {
//     fn new(computation: T) -> Self {
//         ComputeCache {
//             computation,
//             result: None,
//         }
//     }
//
//     fn get_result(&mut self) -> String {
//         match &self.result {
//             Some(v) => v.clone(),
//             None => {
//                 let result = (self.computation)();
//                 self.result = Some(result.clone());
//                 result
//             }
//         }
//     }
// }
//
// fn main() {
//     let mut cache = ComputeCache::new(|| {
//         println!("Computing (this will take 2 seconds)...");
//         thread::sleep(Duration::from_secs(2));
//         "Hello, world!".to_string()
//     });
//
//     println!("First call:");
//     println!("Result: {}", cache.get_result());
//  
//     println!("\nSecond call:");
//     println!("Result (cached): {}", cache.get_result());
// }

