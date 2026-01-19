use std::io;

// My first rust program, can be refactored in future

fn main() {

    loop {
        let mut choice = String::new();
        println!("Please choose the option");
        println!("1: to celsius");
        println!("2: to fahrenheit");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read the input");

            
        let choice = choice.trim();
        let degree = get_degree();

        if choice == "1" {
            println!("{}", to_celcius(degree));
        } else if choice == "2" {
            println!("{}", to_fahrenheit(degree));
        } else {
            println!("invalid input");
        }
    }
}

fn get_degree() -> f64 {
    println!("Enter temperature");

    let mut degree = String::new();
    io::stdin()
        .read_line(&mut degree)
        .expect("Failed to read the input");


    let degree: f64 = degree
        .trim()
        .parse()
        .expect("invalid input, not integer");

    degree
}
 
fn to_fahrenheit(degree: f64) -> f64 {
    (degree * (9.0/5.0)) + 32.0
}

fn to_celcius(degree: f64) -> f64 {
    (degree - 32.0) * 5.0/9.0
}