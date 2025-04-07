
// Define our data structure
struct Student {
    major: String,
}

 // Higher order functions update majors

 fn update_majors(collection: &mut Vec<Student>, behavior: fn(&mut Student, String), major: String) {
    for item in collection.iter_mut() {
        behavior(item, major.clone());
    }
 }

 // First Order functions, assign_major(student,major_declared)
 fn assign_major(s: &mut Student,major:String){
        s.major = major;
 }



fn main() {
    //temporry assign
        let mut _items = vec![
            Student { major: String::from("") },
            Student { major: String::from("") },
            Student { major: String::from("") },
        ];
    
    //Call the higher order function that we created
    update_majors(&mut _items, assign_major, String::from("Computer Science"));

    //Loop through the collection and print the majors

    for item in _items.iter() {
        println!("Major: {}", item.major);
    }

}