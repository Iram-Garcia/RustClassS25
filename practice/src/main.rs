
    struct Car {
        body: String,
        year: u16,
        color: String,
    }
    impl Car {
       fn new(b: String, y: u16, c: String) -> Car {
            Car {
                body: b,
                year: y,
                color: c,
            }
       } 
    }
    //function to print car
    fn get_car(car: &Car) {
        println!("Car model: {} year: {} color: {}", car.body, car.year, car.color);
    }
fn main() {

    let my_car = Car {
        body: "Sedan".to_string(),
        year: 2021,
        color: "Black".to_string(),

    };
    get_car(&my_car);
    println!("My printed car model: {} year: {} color: {}", my_car.body, my_car.year, my_car.color);
}